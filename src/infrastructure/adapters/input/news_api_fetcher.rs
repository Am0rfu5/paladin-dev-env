use crate::application::services::content::content_fetching_service::ContentFetchingService;
use crate::application::services::content::content_list_fetching_service::ContentListFetchingService;
use crate::core::platform::container::content::{ContentItem, ContentType, TextContent};
use crate::core::platform::container::content_list::ContentList;
use crate::infrastructure::adapters::input::http_content_fetcher::HttpContentFetcher;
use serde::Deserialize;
use url::Url;
use urlencoding;

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct NewsApiFetcher {
    api_key: String,
    http_client: reqwest::blocking::Client,
    content_fetcher: Option<HttpContentFetcher>,
}

#[derive(Debug, Deserialize)]
struct NewsApiResponse {
    status: String,
    #[serde(rename = "totalResults")]
    #[allow(dead_code)]
    total_results: u32,
    articles: Vec<NewsArticle>,
}

#[derive(Debug, Deserialize, Clone)]
struct NewsArticle {
    source: NewsSource,
    author: Option<String>,
    title: String,
    description: Option<String>,
    url: String,
    #[serde(rename = "urlToImage")]
    #[allow(dead_code)]
    url_to_image: Option<String>,
    #[serde(rename = "publishedAt")]
    #[allow(dead_code)]
    published_at: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct NewsSource {
    #[allow(dead_code)]
    id: Option<String>,
    name: String,
}

impl NewsApiFetcher {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: reqwest::blocking::Client::new(),
            content_fetcher: None,
        }
    }

    pub fn with_content_fetcher(mut self, fetcher: HttpContentFetcher) -> Self {
        self.content_fetcher = Some(fetcher);
        self
    }

    fn build_news_api_url(&self, query: &str, page_size: u32, page: u32) -> String {
        format!(
            "https://newsapi.org/v2/everything?q={}&pageSize={}&page={}&apiKey={}",
            urlencoding::encode(query),
            page_size,
            page,
            self.api_key
        )
    }

    fn create_content_item_from_article(
        &self,
        article: &NewsArticle,
    ) -> Result<ContentItem, String> {
        // Create basic text content with article metadata
        let text_content = TextContent::new(None, article.content.clone())
            .map_err(|e| format!("Failed to create text content: {}", e))?;

        let content_type = ContentType::Text(text_content);
        let mut content_item = ContentItem::new(content_type)
            .map_err(|e| format!("Failed to create content item: {}", e))?;

        // Set article metadata using setter methods
        let article_url =
            Url::parse(&article.url).map_err(|e| format!("Invalid article URL: {}", e))?;

        content_item.set_url(Some(article_url.clone()));
        content_item.set_source_url(Some(article_url));
        content_item.set_title(Some(article.title.clone()));
        content_item.set_description(article.description.clone());
        content_item.set_author(article.author.clone());
        content_item.set_source(Some(article.source.name.clone()));

        // Note: source_id and pub_date are not available in the current ContentItem API
        // These would need to be added to the ContentItem if needed

        // Add news-related tags
        let mut tags = vec!["news".to_string(), article.source.name.clone()];
        if let Some(author) = &article.author {
            tags.push(format!("author:{}", author));
        }
        content_item.set_tags(Some(tags));

        Ok(content_item)
    }

    fn fetch_article_content(&self, article: &NewsArticle) -> Result<ContentItem, String> {
        if let Some(ref content_fetcher) = self.content_fetcher {
            // Use HTTP content fetcher to get full article content
            let mut content_item = content_fetcher.fetch_content(&article.url)?;

            // Override with news API metadata which might be more accurate
            content_item.set_title(Some(article.title.clone()));
            content_item.set_description(article.description.clone());
            content_item.set_author(article.author.clone());
            content_item.set_source(Some(article.source.name.clone()));

            // Add news-specific tags
            let mut existing_tags = content_item.tags().cloned().unwrap_or_default();
            existing_tags.extend(vec!["news".to_string(), article.source.name.clone()]);
            if let Some(author) = &article.author {
                existing_tags.push(format!("author:{}", author));
            }
            content_item.set_tags(Some(existing_tags));

            Ok(content_item)
        } else {
            // Fall back to creating content item from article metadata only
            self.create_content_item_from_article(article)
        }
    }
}

impl ContentListFetchingService for NewsApiFetcher {
    fn fetch_content_list(&self, query: &str) -> Result<ContentList, String> {
        let url = self.build_news_api_url(query, 20, 1);

        let response = self
            .http_client
            .get(&url)
            .header("User-Agent", "paladin-content-fetcher/1.0")
            .send()
            .map_err(|e| format!("Failed to fetch from News API: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("News API error: HTTP {}", response.status()));
        }

        let news_response: NewsApiResponse = response
            .json()
            .map_err(|e| format!("Failed to parse News API response: {}", e))?;

        if news_response.status != "ok" {
            return Err(format!(
                "News API returned error status: {}",
                news_response.status
            ));
        }

        // Create content list
        let mut content_list = ContentList::new();

        // Set metadata
        let list_name = format!("News: {}", query);
        let list_url = Url::parse(&format!("newsapi://query/{}", urlencoding::encode(query)))
            .map_err(|e| format!("Failed to create list URL: {}", e))?;

        content_list.set_name(Some(list_name));
        content_list.set_url(Some(list_url));
        content_list.set_source(Some("news_api".to_string()));

        // Add content items
        for article in &news_response.articles {
            match self.fetch_article_content(article) {
                Ok(content_item) => {
                    content_list.add_item(content_item);
                }
                Err(e) => {
                    eprintln!(
                        "Failed to fetch content for article '{}': {}",
                        article.title, e
                    );
                    // For failed items, we could create a placeholder content item
                    // or implement a retry mechanism later
                    if let Ok(placeholder_item) = self.create_content_item_from_article(article) {
                        content_list.add_item(placeholder_item);
                    }
                }
            }
        }

        Ok(content_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    fn create_mock_news_response() -> serde_json::Value {
        serde_json::json!({
            "status": "ok",
            "totalResults": 2,
            "articles": [
                {
                    "source": {
                        "id": "test-news",
                        "name": "Test News"
                    },
                    "author": "John Doe",
                    "title": "Test Article 1",
                    "description": "This is a test article",
                    "url": "https://example.com/article1",
                    "urlToImage": "https://example.com/image1.jpg",
                    "publishedAt": "2023-01-01T12:00:00Z",
                    "content": "This is the content of the test article..."
                },
                {
                    "source": {
                        "id": "tech-news",
                        "name": "Tech News"
                    },
                    "author": "Jane Smith",
                    "title": "Test Article 2",
                    "description": "Another test article",
                    "url": "https://example.com/article2",
                    "urlToImage": null,
                    "publishedAt": "2023-01-02T10:30:00Z",
                    "content": "Content of the second test article..."
                }
            ]
        })
    }

    #[test]
    fn test_fetch_content_list_success() {
        let mut server = Server::new();

        // Mock the News API response
        let mock_response = create_mock_news_response();
        let _mock = server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        // Create fetcher with mock server URL as base (in real implementation, you'd inject the base URL)
        let _fetcher = NewsApiFetcher::new("test-api-key".to_string());

        // Note: This test would need the fetcher to accept a base URL for the API
        // For now, we'll test the parsing logic separately
    }

    #[test]
    fn test_create_content_item_from_article() {
        let article = NewsArticle {
            source: NewsSource {
                id: Some("test-id".to_string()),
                name: "Test Source".to_string(),
            },
            author: Some("Test Author".to_string()),
            title: "Test Title".to_string(),
            description: Some("Test Description".to_string()),
            url: "https://example.com/test".to_string(),
            url_to_image: None,
            published_at: "2023-01-01T12:00:00Z".to_string(),
            content: Some("Test content".to_string()),
        };

        let fetcher = NewsApiFetcher::new("test-key".to_string());
        let result = fetcher.create_content_item_from_article(&article);

        assert!(result.is_ok());
        let content_item = result.unwrap();

        assert_eq!(content_item.title(), Some(&"Test Title".to_string()));
        assert_eq!(
            content_item.description(),
            Some(&"Test Description".to_string())
        );
        assert_eq!(content_item.author(), Some(&"Test Author".to_string()));
        assert_eq!(content_item.source(), Some(&"Test Source".to_string()));
        assert!(content_item.tags().unwrap().contains(&"news".to_string()));
    }

    #[test]
    fn test_build_news_api_url() {
        let fetcher = NewsApiFetcher::new("test-key".to_string());
        let url = fetcher.build_news_api_url("rust programming", 10, 1);

        assert!(url.contains("q=rust%20programming"));
        assert!(url.contains("pageSize=10"));
        assert!(url.contains("page=1"));
        assert!(url.contains("apiKey=test-key"));
    }

    #[test]
    fn test_with_content_fetcher() {
        let http_fetcher = HttpContentFetcher::new();
        let fetcher =
            NewsApiFetcher::new("test-key".to_string()).with_content_fetcher(http_fetcher);

        assert!(fetcher.content_fetcher.is_some());
    }

    #[test]
    fn test_parse_news_api_response() {
        let json_response = create_mock_news_response();
        let news_response: Result<NewsApiResponse, _> = serde_json::from_value(json_response);

        assert!(news_response.is_ok());
        let response = news_response.unwrap();
        assert_eq!(response.status, "ok");
        assert_eq!(response.total_results, 2);
        assert_eq!(response.articles.len(), 2);
        assert_eq!(response.articles[0].title, "Test Article 1");
        assert_eq!(response.articles[1].source.name, "Tech News");
    }

    #[test]
    fn test_fetcher_creation() {
        let fetcher = NewsApiFetcher::new("my-api-key".to_string());
        assert_eq!(fetcher.api_key, "my-api-key");
        assert!(fetcher.content_fetcher.is_none());
    }

    #[test]
    fn test_build_url_with_special_characters() {
        let fetcher = NewsApiFetcher::new("key".to_string());
        let url = fetcher.build_news_api_url("rust & python", 20, 2);

        // Verify URL encoding
        assert!(url.contains("rust%20%26%20python"));
        assert!(url.contains("pageSize=20"));
        assert!(url.contains("page=2"));
    }

    #[test]
    fn test_build_url_pagination() {
        let fetcher = NewsApiFetcher::new("key".to_string());

        let url1 = fetcher.build_news_api_url("test", 50, 1);
        let url2 = fetcher.build_news_api_url("test", 50, 5);

        assert!(url1.contains("page=1"));
        assert!(url2.contains("page=5"));
        assert!(url1.contains("pageSize=50"));
        assert!(url2.contains("pageSize=50"));
    }

    #[test]
    fn test_article_without_optional_fields() {
        let article = NewsArticle {
            source: NewsSource {
                id: None,
                name: "Source".to_string(),
            },
            author: None,
            title: "Title".to_string(),
            description: None,
            url: "https://example.com".to_string(),
            url_to_image: None,
            published_at: "2023-01-01T00:00:00Z".to_string(),
            content: None,
        };

        let fetcher = NewsApiFetcher::new("key".to_string());
        let result = fetcher.create_content_item_from_article(&article);

        assert!(result.is_ok());
        let content_item = result.unwrap();

        assert_eq!(content_item.title(), Some(&"Title".to_string()));
        assert_eq!(content_item.description(), None);
        assert_eq!(content_item.author(), None);
        assert!(content_item.tags().unwrap().contains(&"news".to_string()));
    }

    #[test]
    fn test_article_with_all_fields() {
        let article = NewsArticle {
            source: NewsSource {
                id: Some("src-123".to_string()),
                name: "Full Source".to_string(),
            },
            author: Some("Full Author".to_string()),
            title: "Full Title".to_string(),
            description: Some("Full Description".to_string()),
            url: "https://example.com/full".to_string(),
            url_to_image: Some("https://example.com/image.jpg".to_string()),
            published_at: "2023-12-25T18:30:00Z".to_string(),
            content: Some("Full content text here".to_string()),
        };

        let fetcher = NewsApiFetcher::new("key".to_string());
        let result = fetcher.create_content_item_from_article(&article);

        assert!(result.is_ok());
        let content_item = result.unwrap();

        assert_eq!(content_item.title(), Some(&"Full Title".to_string()));
        assert_eq!(
            content_item.description(),
            Some(&"Full Description".to_string())
        );
        assert_eq!(content_item.author(), Some(&"Full Author".to_string()));
        assert_eq!(content_item.source(), Some(&"Full Source".to_string()));

        let tags = content_item.tags().unwrap();
        assert!(tags.contains(&"news".to_string()));
        assert!(tags.contains(&"Full Source".to_string()));
        assert!(tags.contains(&"author:Full Author".to_string()));
    }

    #[test]
    fn test_article_with_invalid_url() {
        let article = NewsArticle {
            source: NewsSource {
                id: None,
                name: "Source".to_string(),
            },
            author: None,
            title: "Title".to_string(),
            description: None,
            url: "not-a-valid-url".to_string(),
            url_to_image: None,
            published_at: "2023-01-01T00:00:00Z".to_string(),
            content: Some("content".to_string()),
        };

        let fetcher = NewsApiFetcher::new("key".to_string());
        let result = fetcher.create_content_item_from_article(&article);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Invalid article URL"));
    }

    #[test]
    fn test_news_source_structure() {
        let source = NewsSource {
            id: Some("test-id".to_string()),
            name: "Test Name".to_string(),
        };

        assert_eq!(source.id, Some("test-id".to_string()));
        assert_eq!(source.name, "Test Name");
    }

    #[test]
    fn test_news_source_without_id() {
        let source = NewsSource {
            id: None,
            name: "Name Only".to_string(),
        };

        assert_eq!(source.id, None);
        assert_eq!(source.name, "Name Only");
    }

    #[test]
    fn test_news_article_clone() {
        let article = NewsArticle {
            source: NewsSource {
                id: Some("id".to_string()),
                name: "Name".to_string(),
            },
            author: Some("Author".to_string()),
            title: "Title".to_string(),
            description: Some("Desc".to_string()),
            url: "https://example.com".to_string(),
            url_to_image: Some("https://example.com/img.jpg".to_string()),
            published_at: "2023-01-01T00:00:00Z".to_string(),
            content: Some("Content".to_string()),
        };

        let cloned = article.clone();
        assert_eq!(article.title, cloned.title);
        assert_eq!(article.author, cloned.author);
        assert_eq!(article.source.name, cloned.source.name);
    }

    #[test]
    fn test_api_response_deserialization() {
        let json = r#"{
            "status": "ok",
            "totalResults": 1,
            "articles": [
                {
                    "source": {
                        "id": "bbc-news",
                        "name": "BBC News"
                    },
                    "author": "BBC Reporter",
                    "title": "Breaking News",
                    "description": "Latest update",
                    "url": "https://bbc.com/news/123",
                    "urlToImage": "https://bbc.com/img.jpg",
                    "publishedAt": "2023-06-15T14:20:00Z",
                    "content": "Full article content..."
                }
            ]
        }"#;

        let response: Result<NewsApiResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());

        let parsed = response.unwrap();
        assert_eq!(parsed.status, "ok");
        assert_eq!(parsed.total_results, 1);
        assert_eq!(parsed.articles.len(), 1);
        assert_eq!(parsed.articles[0].source.id, Some("bbc-news".to_string()));
    }

    #[test]
    fn test_fetcher_debug_format() {
        let fetcher = NewsApiFetcher::new("secret-key".to_string());
        let debug_str = format!("{:?}", fetcher);

        assert!(debug_str.contains("NewsApiFetcher"));
    }
}
