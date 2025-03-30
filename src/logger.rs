use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Debug)]
pub enum LoggerError {
    InvalidMessage(String),
    InvalidStyle(String),
}

impl std::fmt::Display for LoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoggerError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
            LoggerError::InvalidStyle(msg) => write!(f, "Invalid style: {}", msg),
        }
    }
}

impl std::error::Error for LoggerError {}

impl From<LoggerError> for JsValue {
    fn from(error: LoggerError) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

pub struct Logger;

impl Logger {
    #[cfg(test)]
    pub fn log(message: &str) -> Result<(), LoggerError> {
        if message.is_empty() {
            return Err(LoggerError::InvalidMessage("Message cannot be empty".to_string()));
        }
        console::log_1(&JsValue::from_str(message));
        Ok(())
    }

    #[cfg(test)]
    pub fn warn(message: &str) -> Result<(), LoggerError> {
        if message.is_empty() {
            return Err(LoggerError::InvalidMessage("Message cannot be empty".to_string()));
        }
        console::warn_1(&JsValue::from_str(message));
        Ok(())
    }

    pub fn info(message: &str) -> Result<(), LoggerError> {
        if message.is_empty() {
            return Err(LoggerError::InvalidMessage("Message cannot be empty".to_string()));
        }
        console::info_1(&JsValue::from_str(message));
        Ok(())
    }

    #[cfg(test)]
    pub fn debug(message: &str) -> Result<(), LoggerError> {
        if message.is_empty() {
            return Err(LoggerError::InvalidMessage("Message cannot be empty".to_string()));
        }
        console::debug_1(&JsValue::from_str(message));
        Ok(())
    }

    #[cfg(test)]
    pub fn log_with_style(message: &str, style: &str) -> Result<(), LoggerError> {
        if message.is_empty() {
            return Err(LoggerError::InvalidMessage("Message cannot be empty".to_string()));
        }
        if style.is_empty() {
            return Err(LoggerError::InvalidStyle("Style cannot be empty".to_string()));
        }

        let formatted = format!("%c{}", message);
        let style = JsValue::from_str(style);
        console::log_2(&JsValue::from_str(&formatted), &style);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_functions() {
        assert!(Logger::log("Test log message").is_ok());
        assert!(Logger::warn("Test warning message").is_ok());
        assert!(Logger::debug("Test debug message").is_ok());
        assert!(Logger::log_with_style("Test styled message", "color: red").is_ok());
    }
} 