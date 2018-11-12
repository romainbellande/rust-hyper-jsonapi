use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
pub use std::collections::HashMap;

use super::api::*;

pub trait JApiModel: Serialize
where
    for<'de> Self: Deserialize<'de>,
{
    fn get_type(&self) -> String;

    fn get_id(&self) -> String;

    fn serialize(&self) -> Document {
        if let Value::Object(json_attributes) = to_value(self).unwrap() {
            let attrs = json_attributes
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .filter(|(key, _value)| {
                    if key == "id" {
                        return false;
                    }
                    true
                })
                .collect();

            let attributes = attrs;

            let resource = Resource {
                id: self.get_id(),
                _type: self.get_type(),
                attributes,
            };

            let jsonapi = DocumentVersion {
                version: "1.0".to_string(),
            };

            let document = Document {
                jsonapi,
                data: resource,
            };

            document
        } else {
            panic!(format!("{} is not a Value::Object", self.get_type()))
        }
    }
}

#[macro_export]
macro_rules! japi_model {
    ($model:ty; $type:expr) => {
        impl JApiModel for $model {
            fn get_id(&self) -> String {
                self.id.to_string()
            }
            fn get_type(&self) -> String {
                $type.to_string()
            }
        }
    };
}
