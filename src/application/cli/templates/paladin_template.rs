//! Paladin configuration template generator

/// Generate a Paladin YAML configuration template
///
/// # Arguments
/// * `name` - Name for the Paladin
/// * `provider` - LLM provider (openai, deepseek, or anthropic)
///
/// # Returns
/// A formatted YAML template string with all configuration options documented
pub fn generate_paladin_template(name: &str, provider: &str) -> String {
    format!(
        r#"# Paladin Configuration Template
# Generated for: {name}

# Required: Name of the Paladin
name: "{name}"

# Required: System prompt defining the Paladin's behavior and personality
# This is the core instruction that shapes how the Paladin responds
system_prompt: |
  You are a helpful AI assistant named {name}.
  Your goal is to provide accurate, helpful, and thoughtful responses.
  Always be clear, concise, and considerate in your communication.

# Required: Model to use for generation
# Examples: "gpt-4", "gpt-4-turbo", "deepseek-chat", "claude-3-5-sonnet-20241022"
model: "{default_model}"

# Optional: Temperature for response generation (0.0 - 2.0)
# Lower values (0.0-0.7) = more focused and deterministic
# Higher values (0.7-2.0) = more creative and random
# Default: 0.7
temperature: 0.7

# Optional: Maximum number of reasoning loops
# The Paladin will iterate up to this many times to refine its response
# Default: 3
max_loops: 3

# Optional: Timeout in seconds for the entire execution
# Default: 300 (5 minutes)
timeout_seconds: 300

# Optional: Stop words that trigger early termination
# If any of these strings appear in the output, execution stops
# Default: []
stop_words: []
  # - "DONE"
  # - "FINISHED"

# Required: LLM Provider Configuration
provider:
  # Provider type: openai, deepseek, or anthropic
  type: {provider}
  # Note: API key is loaded from environment variable:
  # - OPENAI_API_KEY for openai
  # - DEEPSEEK_API_KEY for deepseek
  # - ANTHROPIC_API_KEY for anthropic

# Optional: Garrison (Memory) Configuration
# Uncomment to enable conversation history and context retention
# garrison:
#   # Type: in_memory (fast, ephemeral) or sqlite (persistent)
#   type: in_memory
#   config:
#     # For in_memory: maximum number of entries to retain
#     max_entries: 100
#     
#     # For sqlite: path to database file
#     # path: "./garrison.db"

# Optional: Arsenal (Tools/MCP) Configuration
# Uncomment to enable external tool access via Model Context Protocol
# arsenal:
#   mcp_servers:
#     # STDIO-based MCP server (runs as subprocess)
#     - name: web_search
#       type: stdio
#       command: uvx
#       args:
#         - mcp-web-search
#     
#     # SSE-based MCP server (connects to HTTP endpoint)
#     # - name: api_service
#     #   type: sse
#     #   endpoint: http://localhost:3000/mcp
"#,
        name = name,
        provider = provider,
        default_model = match provider {
            "openai" => "gpt-4",
            "deepseek" => "deepseek-chat",
            "anthropic" => "claude-3-5-sonnet-20241022",
            _ => "gpt-4",
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_template_openai() {
        let template = generate_paladin_template("test-agent", "openai");
        assert!(template.contains("name: \"test-agent\""));
        assert!(template.contains("type: openai"));
        assert!(template.contains("model: \"gpt-4\""));
        assert!(template.contains("system_prompt"));
    }

    #[test]
    fn test_generate_template_deepseek() {
        let template = generate_paladin_template("analyzer", "deepseek");
        assert!(template.contains("name: \"analyzer\""));
        assert!(template.contains("type: deepseek"));
        assert!(template.contains("model: \"deepseek-chat\""));
    }

    #[test]
    fn test_generate_template_anthropic() {
        let template = generate_paladin_template("assistant", "anthropic");
        assert!(template.contains("name: \"assistant\""));
        assert!(template.contains("type: anthropic"));
        assert!(template.contains("model: \"claude-3-5-sonnet-20241022\""));
    }

    #[test]
    fn test_template_is_valid_yaml() {
        let template = generate_paladin_template("test", "openai");
        // Verify it parses as valid YAML
        let result = serde_yaml::from_str::<serde_yaml::Value>(&template);
        assert!(result.is_ok(), "Generated template should be valid YAML");
    }
}
