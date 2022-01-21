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

impl From<serde_json::error::Error> for ApplicationError {
    fn from(value: serde_json::error::Error) -> ApplicationError {
        ApplicationError::ClientError(format!("Cannot convert to stirng {}", value))
    }
}

impl From<&aws_sdk_dynamodb::model::AttributeValue> for ApplicationError {
    fn from(value: &aws_sdk_dynamodb::model::AttributeValue) -> ApplicationError {
        ApplicationError::InternalError(format!("{:?}", value))
    }
}

impl<E> From<aws_sdk_dynamodb::SdkError<E>> for ApplicationError
where
    E: error::Error,
{
    fn from(value: aws_sdk_dynamodb::SdkError<E>) -> ApplicationError {
        ApplicationError::InternalError(format!("{:?}", value))
    }
}