use chrono::Local;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &str, field_value: String, err: &str) -> Self {
        let current_time = Local::now();
        let formatted_date = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
        FormError {
            form_values: (field_name.to_string(), field_value),
            date: formatted_date,
            err: err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty".to_string()));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password".to_string(),
                self.password.clone(),
                "Password should be at least 8 characters long".to_string(),
            ));
        }

        let has_alpha = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| c.is_ascii_punctuation());

        if !(has_alpha && has_digit && has_symbol) {
            return Err(FormError::new(
                "password".to_string(),
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols".to_string(),
            ));
        }

        Ok(())
    }
}