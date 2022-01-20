use aws_sdk_dynamodb::SdkError;
use aws_sdk_dynamodb::model::AttributeValue;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum ApplicationError {
    InitError(String),
    ClientError(String),
    InternalError(String),
    SdkError(String),
}

impl std::error::Error for ApplicationError {}

impl fmt::Display for ApplicationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ApplicationError::InitError(msg) => write!(f, "InitError: {}", msg),
      ApplicationError::ClientError(msg) => write!(f, "ClientError: {}", msg),
      ApplicationError::InternalError(msg) => write!(f, "InternalError: {}", msg),
      ApplicationError::SdkError(err) => write!(f, "SdkError: {}", err),
    }
  }
}

impl From<&AttributeValue> for ApplicationError {
    fn from(_: &AttributeValue) -> ApplicationError {
        ApplicationError::InternalError("Invalid value type".to_string())
    }
}

impl<E> From<SdkError<E>> for ApplicationError
where
    E: error::Error,
{
    fn from(value: SdkError<E>) -> ApplicationError {
        ApplicationError::InternalError(format!("AWS Failure: {:?}", value))
    }
}