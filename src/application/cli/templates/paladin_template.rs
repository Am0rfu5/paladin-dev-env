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

# Optional: Autonomous Features Configuration
# Uncomment to enable planning, prompt generation, dynamic temperature
# adjustment, or agent handoffs. Note: the --auto-plan / --auto-prompt /
# --dynamic-temp / --enable-handoffs CLI flags force their feature on over
# this section but cannot turn one off.
# autonomous:
#   planning:
#     enabled: false        # default: false
#     max_subtasks: 10      # default: 10
#   prompt_generation:
#     enabled: false        # default: false
#     description: null     # default: null (required if enabled)
#   dynamic_temperature:
#     enabled: false        # default: false
#     min: 0.1               # default: 0.1
#     max: 0.9               # default: 0.9
#   handoffs:
#     enabled: false        # default: false
#     strategy: Automatic   # default: Automatic (Explicit, Threshold also available)
#     max_depth: 5          # default: 5
#     retry:
#       max_retries: 3            # default: 3
#       initial_backoff_ms: 1000  # default: 1000
#       backoff_multiplier: 2.0   # default: 2.0
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

    #[test]
    fn test_generate_template_documents_autonomous_section() {
        // The generated template documents the `autonomous:` section and
        // all four sub-sections, matching `PaladinYamlConfig.autonomous`'s
        // schema key-for-key (D-06's `AutonomousConfig` reuse). Resolves
        // 06-CONTEXT.md's open discretion question in favour of emitting
        // the example.
        let template = generate_paladin_template("test-agent", "openai");
        assert!(template.contains("autonomous:"));
        assert!(template.contains("planning"));
        assert!(template.contains("prompt_generation"));
        assert!(template.contains("dynamic_temperature"));
        assert!(template.contains("handoffs"));
    }

    #[test]
    fn test_generate_template_autonomous_section_is_commented_out() {
        // Every line mentioning `autonomous:` or one of its four
        // sub-sections is a `#` comment -- required for
        // `test_template_is_valid_yaml` to keep passing once this section
        // exists, and for the section to be inert until an operator
        // deliberately uncomments it.
        let template = generate_paladin_template("test-agent", "openai");
        let markers = [
            "autonomous:",
            "planning",
            "prompt_generation",
            "dynamic_temperature",
            "handoffs",
        ];

        for line in template.lines() {
            if markers.iter().any(|marker| line.contains(marker)) {
                let trimmed = line.trim_start();
                assert!(
                    trimmed.starts_with('#'),
                    "expected autonomous-section line to be commented out: {line:?}"
                );
            }
        }
    }
}
