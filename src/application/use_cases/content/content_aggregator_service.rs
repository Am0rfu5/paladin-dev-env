/*
Content Aggregator Service Use Case

This use case offers a service for aggregating content.

It takes a list of content items as input and returns a single aggregated content list.
*/
use crate::core::platform::container::content_list::ContentList;
use serde_json::Value;

pub trait ContentListService {
    fn fetch_content_list(&self, url: &str) -> Result<ContentList, String>;
    fn aggregate_content(&self, data: Vec<Value>) -> Value;
}

pub struct AggregateContent<T: ContentListService> {
    service: T,
}

impl<T: ContentListService> AggregateContent<T> {
    pub fn new(service: T) -> Self {
        Self { service }
    }

    pub fn execute(&self, data: Vec<Value>) -> Value {
        self.service.aggregate_content(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct MockContentListService;

    impl ContentListService for MockContentListService {
        fn fetch_content_list(&self, _url: &str) -> Result<ContentList, String> {
            Ok(ContentList::new())
        }

        fn aggregate_content(&self, data: Vec<Value>) -> Value {
            Value::Array(data)
        }
    }

    struct CustomMockContentListService {
        aggregate_fn: Box<dyn Fn(Vec<Value>) -> Value>,
    }

    impl ContentListService for CustomMockContentListService {
        fn fetch_content_list(&self, _url: &str) -> Result<ContentList, String> {
            Ok(ContentList::new())
        }

        fn aggregate_content(&self, data: Vec<Value>) -> Value {
            (self.aggregate_fn)(data)
        }
    }

    #[test]
    fn test_aggregate_content_new() {
        let service = MockContentListService;
        let use_case = AggregateContent::new(service);
        let data = vec![Value::String("Test Data".to_string())];
        let result = use_case.execute(data.clone());
        assert_eq!(result, Value::Array(data));
    }

    #[test]
    fn test_aggregate_content_empty_data() {
        let service = MockContentListService;
        let use_case = AggregateContent::new(service);
        let data = vec![];
        let result = use_case.execute(data);
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_aggregate_content_multiple_items() {
        let service = MockContentListService;
        let use_case = AggregateContent::new(service);
        let data = vec![
            json!({"title": "Article 1"}),
            json!({"title": "Article 2"}),
            json!({"title": "Article 3"}),
        ];
        let result = use_case.execute(data.clone());
        assert_eq!(result, Value::Array(data));
    }

    #[test]
    fn test_aggregate_content_complex_data() {
        let service = MockContentListService;
        let use_case = AggregateContent::new(service);
        let data = vec![json!({
            "title": "Complex Article",
            "author": "Test Author",
            "tags": ["rust", "testing"],
            "metadata": {
                "published": "2026-01-28",
                "views": 1000
            }
        })];
        let result = use_case.execute(data.clone());
        assert_eq!(result, Value::Array(data));
    }

    #[test]
    fn test_aggregate_content_with_custom_aggregation() {
        let service = CustomMockContentListService {
            aggregate_fn: Box::new(|data| {
                // Custom aggregation: merge all objects into one
                let mut merged = serde_json::Map::new();
                merged.insert("count".to_string(), json!(data.len()));
                merged.insert("items".to_string(), Value::Array(data));
                Value::Object(merged)
            }),
        };
        let use_case = AggregateContent::new(service);
        let data = vec![json!({"id": 1}), json!({"id": 2})];
        let result = use_case.execute(data);

        assert!(result.is_object());
        assert_eq!(result["count"], 2);
        assert_eq!(result["items"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_aggregate_content_preserves_order() {
        let service = MockContentListService;
        let use_case = AggregateContent::new(service);
        let data = vec![json!({"id": 1}), json!({"id": 2}), json!({"id": 3})];
        let result = use_case.execute(data.clone());
        let result_array = result.as_array().unwrap();

        assert_eq!(result_array.len(), 3);
        assert_eq!(result_array[0]["id"], 1);
        assert_eq!(result_array[1]["id"], 2);
        assert_eq!(result_array[2]["id"], 3);
    }

    #[test]
    fn test_aggregate_content_different_types() {
        let service = MockContentListService;
        let use_case = AggregateContent::new(service);
        let data = vec![
            json!("string value"),
            json!(42),
            json!(true),
            json!({"key": "value"}),
            json!(["array", "items"]),
        ];
        let result = use_case.execute(data.clone());
        let result_array = result.as_array().unwrap();

        assert_eq!(result_array.len(), 5);
        assert!(result_array[0].is_string());
        assert!(result_array[1].is_number());
        assert!(result_array[2].is_boolean());
        assert!(result_array[3].is_object());
        assert!(result_array[4].is_array());
    }
}
