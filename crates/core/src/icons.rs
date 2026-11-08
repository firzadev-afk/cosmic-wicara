//! Symbolic icon map.

use cosmic::widget::icon::{IconFallback, Named, from_name};

/// Question dialog icon.
pub const QUESTION_ICON: &str = "dialog-question-symbolic";

/// Info dialog icon.
pub const INFORMATION_ICON: &str = "dialog-information-symbolic";

/// Warning dialog icon.
pub const WARNING_ICON: &str = "dialog-warning-symbolic";

/// Error dialog icon.
pub const ERROR_ICON: &str = "dialog-error-symbolic";

/// Update applet icon.
pub const UPDATE_ICON: &str = "system-software-update-symbolic";

/// Fallback update icon.
pub const UPDATE_FALLBACK_ICON: &str = "system-software-install-symbolic";

/// Core dialog icons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogIcon {
    /// --question.
    Question,
    /// --info.
    Information,
    /// --warning.
    Warning,
    /// --error.
    Error,
    /// Update action.
    Update,
}

impl DialogIcon {
    /// Symbolic XDG name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Question => QUESTION_ICON,
            Self::Information => INFORMATION_ICON,
            Self::Warning => WARNING_ICON,
            Self::Error => ERROR_ICON,
            Self::Update => UPDATE_ICON,
        }
    }

    /// Icon lookup handle.
    pub fn named(self) -> Named {
        match self {
            Self::Update => update(),
            Self::Question => question(),
            Self::Information => information(),
            Self::Warning => warning(),
            Self::Error => error(),
        }
    }
}

/// Normalize a freedesktop icon name.
#[must_use]
pub fn symbolic_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.ends_with("-symbolic") {
        trimmed.to_owned()
    } else {
        format!("{trimmed}-symbolic")
    }
}

/// Build symbolic icon lookup.
pub fn symbolic(name: &str) -> Named {
    from_name(symbolic_name(name))
}

/// Question icon lookup.
pub fn question() -> Named {
    symbolic(QUESTION_ICON)
}

/// Info icon lookup.
pub fn information() -> Named {
    symbolic(INFORMATION_ICON)
}

/// Warning icon lookup.
pub fn warning() -> Named {
    symbolic(WARNING_ICON)
}

/// Error icon lookup.
pub fn error() -> Named {
    symbolic(ERROR_ICON)
}

/// Update icon lookup.
pub fn update() -> Named {
    let mut named = symbolic(UPDATE_ICON);
    named.fallback = Some(IconFallback::Names(vec![
        UPDATE_FALLBACK_ICON.into(),
        "system-software-install".into(),
    ]));
    named
}

/// Map --icon override.
pub fn for_override(override_name: &str, default: DialogIcon) -> Named {
    let trimmed = override_name.trim();
    if trimmed.is_empty() {
        default.named()
    } else {
        symbolic(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Debug name helper.
    fn debug_name(named: &Named) -> String {
        format!("{named:?}")
    }

    #[test]
    fn symbolic_adds_suffix_and_preserves() {
        assert_eq!(symbolic_name("dialog-question"), "dialog-question-symbolic");
        assert_eq!(
            symbolic_name("dialog-warning-symbolic"),
            "dialog-warning-symbolic"
        );
        assert_eq!(symbolic_name("  dialog-error  "), "dialog-error-symbolic");

        let base = symbolic("dialog-question");
        assert!(base.symbolic);
        assert!(debug_name(&base).contains("dialog-question-symbolic"));
        let full = symbolic("dialog-warning-symbolic");
        assert!(full.symbolic);
        assert!(debug_name(&full).contains("dialog-warning-symbolic"));
    }

    #[test]
    fn per_dialog_names_use_dialog_set() {
        assert!(debug_name(&question()).contains(QUESTION_ICON));
        assert!(debug_name(&information()).contains(INFORMATION_ICON));
        assert!(debug_name(&warning()).contains(WARNING_ICON));
        assert!(debug_name(&error()).contains(ERROR_ICON));
        assert_eq!(DialogIcon::Question.name(), QUESTION_ICON);
        assert_eq!(DialogIcon::Information.name(), INFORMATION_ICON);
        assert_eq!(DialogIcon::Warning.name(), WARNING_ICON);
        assert_eq!(DialogIcon::Error.name(), ERROR_ICON);
        assert_eq!(DialogIcon::Update.name(), UPDATE_ICON);
    }

    #[test]
    fn update_falls_back_to_install_icon() {
        let named = update();
        assert!(debug_name(&named).contains(UPDATE_ICON));
        assert!(named.symbolic);
        match &named.fallback {
            Some(IconFallback::Names(fallbacks)) => {
                assert!(fallbacks.iter().any(|f| f.as_ref() == UPDATE_FALLBACK_ICON));
            }
            other => panic!("update icon must carry Names fallback, got {other:?}"),
        }
        assert!(debug_name(&DialogIcon::Update.named()).contains(UPDATE_ICON));
    }

    #[test]
    fn override_blank_falls_back_to_default() {
        assert!(debug_name(&for_override("", DialogIcon::Question)).contains(QUESTION_ICON));
        assert!(debug_name(&for_override("   ", DialogIcon::Warning)).contains(WARNING_ICON));
        assert!(
            debug_name(&for_override("custom-icon", DialogIcon::Error))
                .contains("custom-icon-symbolic")
        );
        assert!(
            debug_name(&for_override("custom-icon-symbolic", DialogIcon::Error))
                .contains("custom-icon-symbolic")
        );
    }
}
