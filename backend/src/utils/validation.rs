use validator::ValidationError;

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("Password must be at least 8 characters long"));
    }

    if !password.chars().any(|c| c.is_numeric()) {
        return Err(ValidationError::new("Password must contain at least one number"));
    }

    if !password.chars().any(|c| c.is_alphabetic()) {
        return Err(ValidationError::new("Password must contain at least one letter"));
    }

    Ok(())
}

pub fn validate_variable_key(key: &str) -> Result<(), ValidationError> {
    if key.is_empty() {
        return Err(ValidationError::new("Variable key cannot be empty"));
    }

    if key.len() > 255 {
        return Err(ValidationError::new("Variable key cannot exceed 255 characters"));
    }

    // Only allow alphanumeric, underscore, hyphen, and dot
    if !key
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        return Err(ValidationError::new(
            "Variable key can only contain alphanumeric characters, underscore, hyphen, and dot",
        ));
    }

    Ok(())
}

pub fn validate_api_key_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::new("API key name cannot be empty"));
    }

    if name.len() > 100 {
        return Err(ValidationError::new("API key name cannot exceed 100 characters"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password() {
        assert!(validate_password("password123").is_ok());
        assert!(validate_password("short").is_err());
        assert!(validate_password("nodigits").is_err());
        assert!(validate_password("12345678").is_err());
    }

    #[test]
    fn test_validate_variable_key() {
        assert!(validate_variable_key("my_variable").is_ok());
        assert!(validate_variable_key("my-variable.v1").is_ok());
        assert!(validate_variable_key("").is_err());
        assert!(validate_variable_key("invalid key!").is_err());
    }

    #[test]
    fn test_validate_api_key_name() {
        assert!(validate_api_key_name("My API Key").is_ok());
        assert!(validate_api_key_name("").is_err());
        assert!(validate_api_key_name(&"x".repeat(101)).is_err());
    }
}
