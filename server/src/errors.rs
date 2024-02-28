use std::backtrace::Backtrace;

use tracing::error;

#[derive(Debug)]
pub enum AppErrors {
    CommonError(String, Backtrace),
}

impl From<&str> for AppErrors {
    fn from(err: &str) -> Self {
        error!("str err: {err}");
        AppErrors::CommonError(err.to_string(), Backtrace::capture())
    }
}

impl From<String> for AppErrors {
    fn from(err: String) -> Self {
        error!("string err: {err}");
        AppErrors::CommonError(err, Backtrace::capture())
    }
}
