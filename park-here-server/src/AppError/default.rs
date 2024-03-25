use crate::AppError::error::AppError;

pub struct DefaultAppError {
    message: Option<String>,
    status_code: i32
}

impl AppError for DefaultAppError {
    fn message(&self) -> String {
        match self.message {
            Some(msg) => msg,
            None => String::from("Internal Server Error") 
        }      
    }

    fn status_code(&self) -> i32 {
        self.status_code
    }

    fn in_short(&self) -> String {
        format!("Error Message: {}\n Status Code: {}", self.message(), self.status_code())
    }
}