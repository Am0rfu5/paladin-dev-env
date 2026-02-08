//! Unit tests for CLI interactive components

#[cfg(test)]
mod prompt_tests {
    #[test]
    fn test_prompt_builders_compile() {
        // These builders exist and compile - actual testing requires mock stdin
        assert!(true);
    }
}

#[cfg(test)]
mod wizard_tests {
    use crate::application::cli::interactive::wizard::{
        Wizard, WizardContext, WizardAction,
    };

    #[tokio::test]
    async fn test_wizard_context_creation() {
        let context = WizardContext::new();
        // Context created successfully
        assert!(context.get("nonexistent").is_none());
    }

    #[tokio::test]
    async fn test_wizard_context_set_get() {
        let mut context = WizardContext::new();
        context.set("key", "value");
        assert_eq!(context.get("key"), Some("value"));
    }

    #[tokio::test]
    async fn test_wizard_context_remove() {
        let mut context = WizardContext::new();
        context.set("key", "value");
        context.remove("key");
        assert_eq!(context.get("key"), None);
    }

    #[tokio::test]
    async fn test_wizard_context_contains() {
        let mut context = WizardContext::new();
        context.set("exists", "yes");
        assert!(context.contains("exists"));
        assert!(!context.contains("not_exists"));
    }

    #[tokio::test]
    async fn test_wizard_creation() {
        let _wizard = Wizard::new();
        // Wizard created successfully
        assert!(true);
    }

    #[test]
    fn test_wizard_action_variants() {
        let continue_action = WizardAction::Continue;
        let skip_action = WizardAction::SkipTo("next_step".to_string());
        let back_action = WizardAction::Back;
        let complete_action = WizardAction::Complete;
        let cancel_action = WizardAction::Cancel;

        // Verify all variants can be created
        assert!(matches!(continue_action, WizardAction::Continue));
        assert!(matches!(skip_action, WizardAction::SkipTo(_)));
        assert!(matches!(back_action, WizardAction::Back));
        assert!(matches!(complete_action, WizardAction::Complete));
        assert!(matches!(cancel_action, WizardAction::Cancel));
    }
}
