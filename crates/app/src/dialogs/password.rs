//! Password dialog.

/// Password dialog config.
#[derive(Clone, Debug)]
pub struct Password {
    /// Window title.
    pub title: String,
    /// Field label.
    pub text: String,
    /// Show username field?
    pub username: bool,
    /// Username buffer.
    pub username_value: String,
    /// Secret buffer.
    pub secret: String,
}

impl Password {
    /// Create password dialog.
    #[must_use]
    pub fn new(title: String, text: String, username: bool) -> Self {
        Self {
            title,
            text,
            username,
            username_value: String::new(),
            secret: String::new(),
        }
    }

    /// Set username.
    pub fn set_username(&mut self, username: String) {
        self.username_value = username;
    }

    /// Set secret.
    pub fn set_secret(&mut self, secret: String) {
        self.secret = secret;
    }

    /// Masked display form.
    #[must_use]
    pub fn display_secret(&self) -> String {
        "•".repeat(self.secret.chars().count())
    }

    /// OK payload.
    #[must_use]
    pub fn output(&self) -> String {
        if self.username {
            format!("{}|{}", self.username_value, self.secret)
        } else {
            self.secret.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn username_output_is_user_pipe_secret() {
        let mut pw = Password::new("T".to_owned(), "Password:".to_owned(), true);
        pw.set_username("alice".to_owned());
        pw.set_secret("s3cret".to_owned());
        assert_eq!(pw.output(), "alice|s3cret");
        assert_eq!(pw.display_secret(), "••••••");
    }

    #[test]
    fn plain_password_outputs_secret_only() {
        let mut pw = Password::new("T".to_owned(), String::new(), false);
        pw.set_secret("hunter2".to_owned());
        assert_eq!(pw.output(), "hunter2");
    }
}
