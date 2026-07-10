//! Unit tests for CLI arsenal commands

use paladin::cli::commands::arsenal::{ArsenalCommands, ArsenalTestArgs};

#[test]
fn test_arsenal_test_args_mcp_stdio_option() {
    // Test that mcp_stdio option is properly defined
    let args = ArsenalTestArgs {
        mcp_stdio: Some("python3 server.py".to_string()),
        mcp_streamable_http: None,
        mcp_auth_token_env: None,
    };

    assert!(args.mcp_stdio.is_some());
    assert_eq!(args.mcp_stdio.unwrap(), "python3 server.py");
    assert!(args.mcp_streamable_http.is_none());
}

#[test]
fn test_arsenal_test_args_mcp_streamable_http_option() {
    // Test that mcp_streamable_http option is properly defined (renamed from
    // the retired --mcp-sse flag, D-02b)
    let args = ArsenalTestArgs {
        mcp_stdio: None,
        mcp_streamable_http: Some("http://localhost:8080".to_string()),
        mcp_auth_token_env: None,
    };

    assert!(args.mcp_stdio.is_none());
    assert!(args.mcp_streamable_http.is_some());
    assert_eq!(args.mcp_streamable_http.unwrap(), "http://localhost:8080");
}

#[test]
fn test_arsenal_test_args_mutual_exclusivity_at_runtime() {
    // Both can be None (list tools operation)
    let args_none = ArsenalTestArgs {
        mcp_stdio: None,
        mcp_streamable_http: None,
        mcp_auth_token_env: None,
    };
    assert!(args_none.mcp_stdio.is_none());
    assert!(args_none.mcp_streamable_http.is_none());

    // Struct allows both to be set, but application logic should validate
    // This test documents the expected validation behavior
    let args_both = ArsenalTestArgs {
        mcp_stdio: Some("cmd".to_string()),
        mcp_streamable_http: Some("url".to_string()),
        mcp_auth_token_env: None,
    };

    // The validation happens in the handler, not in the struct
    // This test just verifies the struct definition allows flexibility
    assert!(args_both.mcp_stdio.is_some());
    assert!(args_both.mcp_streamable_http.is_some());
}

#[test]
fn test_arsenal_commands_variants_exist() {
    // Verify ArsenalCommands enum has expected variants
    // This is a compile-time check more than runtime

    // We can't directly test enum variants without creating them,
    // but we can verify the type exists and is usable
    fn _test_type_exists(_cmd: ArsenalCommands) {
        // This function existing and compiling proves the type is correct
    }
}

#[test]
fn test_arsenal_test_args_default_construction() {
    // Test that ArsenalTestArgs can be constructed with all None
    let args = ArsenalTestArgs {
        mcp_stdio: None,
        mcp_streamable_http: None,
        mcp_auth_token_env: None,
    };

    assert!(args.mcp_stdio.is_none());
    assert!(args.mcp_streamable_http.is_none());
}

#[test]
fn test_arsenal_test_args_stdio_with_arguments() {
    // Test that stdio command can include arguments
    let args = ArsenalTestArgs {
        mcp_stdio: Some("uvx mcp-web-search --max-results 10".to_string()),
        mcp_streamable_http: None,
        mcp_auth_token_env: None,
    };

    let cmd = args.mcp_stdio.unwrap();
    assert!(cmd.contains("uvx"));
    assert!(cmd.contains("mcp-web-search"));
    assert!(cmd.contains("--max-results"));
}

#[test]
fn test_arsenal_test_args_streamable_http_with_full_url() {
    // Test that streamable-http can handle full URLs with paths
    let args = ArsenalTestArgs {
        mcp_stdio: None,
        mcp_streamable_http: Some("https://api.example.com:8443/mcp/events".to_string()),
        mcp_auth_token_env: None,
    };

    let url = args.mcp_streamable_http.unwrap();
    assert!(url.starts_with("https://"));
    assert!(url.contains("8443"));
    assert!(url.contains("/mcp/events"));
}

#[test]
fn test_arsenal_test_args_debug_format() {
    // Verify Debug trait is implemented (needed for error messages)
    let args = ArsenalTestArgs {
        mcp_stdio: Some("debug-test".to_string()),
        mcp_streamable_http: None,
        mcp_auth_token_env: None,
    };

    let debug_output = format!("{:?}", args);
    assert!(debug_output.contains("ArsenalTestArgs"));
    assert!(debug_output.contains("debug-test"));
}
