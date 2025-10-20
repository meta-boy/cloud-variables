use serde_json::Value;

use crate::error::{AppError, Result};

/// Validate that JSON data is valid and within size limits
pub fn validate_json_data(data: &Value, max_size_mb: i32) -> Result<()> {
    // Calculate size in bytes
    let json_string = serde_json::to_string(data)?;
    let size_bytes = json_string.len();
    let size_mb = (size_bytes / (1024 * 1024)) as i32;

    if size_mb > max_size_mb {
        return Err(AppError::Validation(format!(
            "JSON data size ({} MB) exceeds maximum allowed size ({} MB)",
            size_mb, max_size_mb
        )));
    }

    Ok(())
}

/// Calculate the size of JSON data in bytes
pub fn calculate_json_size(data: &Value) -> usize {
    serde_json::to_string(data)
        .map(|s| s.len())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_json_data() {
        let small_data = json!({"key": "value"});
        assert!(validate_json_data(&small_data, 10).is_ok());

        // Create a large JSON object (approximately 2MB)
        let large_data = json!({
            "data": "x".repeat(2 * 1024 * 1024)
        });
        assert!(validate_json_data(&large_data, 1).is_err());
    }

    #[test]
    fn test_calculate_json_size() {
        let data = json!({"key": "value"});
        let size = calculate_json_size(&data);
        assert!(size > 0);
    }
}
