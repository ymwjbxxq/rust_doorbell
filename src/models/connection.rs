use crate::error::ApplicationError;
use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Connection {
  pub connection_id: String,
  pub ttl_expire_at: i64,
}

enum ValueType {
  S,
  N,
}

impl Connection {
  pub fn to_dynamodb(&self) -> HashMap<String, AttributeValue> {
    let mut retval = HashMap::new();
    retval.insert("connection_id".to_owned(), AttributeValue::S(self.connection_id.clone()));
    retval.insert("ttl_expire_at".to_owned(), AttributeValue::N(format!("{:}", self.ttl_expire_at)));

    retval
  }

  pub fn from_dynamodb(value: HashMap<String, AttributeValue>) -> Result<Connection, ApplicationError> {
    Ok(Connection {
      connection_id: Connection::get_key("connection_id", ValueType::S, &value)?,
      ttl_expire_at: Connection::get_key("ttl_expire_at", ValueType::N, &value)?
        .parse::<i64>()
        .unwrap(),
    })
  }

  fn get_key(key: &str, t: ValueType, item: &HashMap<String, AttributeValue>) -> Result<String, ApplicationError> {
    let v = item
      .get(key)
      .ok_or_else(|| ApplicationError::InternalError(format!("Missing '{}'", key)))?;

    Ok(match t {
      ValueType::N => v.as_n()?.to_owned(),
      ValueType::S => v.as_s()?.to_owned(),
    })
  }
}
