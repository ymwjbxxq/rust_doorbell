use aws_sdk_dynamodb::model::AttributeValue;
use std::collections::HashMap;

pub trait AttributeValuesExt {
    fn get_string(&self, key: &str) -> Option<String>;
    fn get_number(&self, key: &str) -> Option<f64>;
    fn get_bool(&self, key: &str) -> Option<bool>;
}

impl AttributeValuesExt for HashMap<String, AttributeValue> {
  fn get_string(&self, key: &str) -> Option<String> {
      Some(self.get(key)?.as_s().ok()?.to_owned())
  }

  fn get_number(&self, key: &str) -> Option<f64> {
    self.get(key)?.as_n().ok()?.parse::<f64>().ok()
  }

  fn get_bool(&self, key: &str) -> Option<bool> {
    Some(self.get(key)?.as_bool().ok()?.to_owned())
  }
}