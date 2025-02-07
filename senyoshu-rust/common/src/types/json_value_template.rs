use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum JsonValueTemplate {
    Option(Box<JsonValueTemplate>),
    Number,
    Boolean,
    Array(Box<JsonValueTemplate>),
    //object的顺序是有意义的
    Object(HashMap<String, JsonValueTemplate>),
    String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DynTypeTemplateWithConstraint {
    pub template: JsonValueTemplate,
    pub constraint: Option<String>,
}

impl JsonValueTemplate {
    pub fn check(&self, object: &JsonValue) -> bool {
        match self {
            JsonValueTemplate::Option(template) => {
                object == &JsonValue::Null || template.check(object)
            }
            JsonValueTemplate::Number => {
                if let JsonValue::Number(_) = object {
                    true
                } else {
                    false
                }
            }
            JsonValueTemplate::Boolean => {
                if let JsonValue::Bool(_) = object {
                    true
                } else {
                    false
                }
            }
            JsonValueTemplate::Array(array_template) => {
                if let JsonValue::Array(array_object) = object {
                    array_object.into_iter().all(|it| array_template.check(it))
                } else {
                    false
                }
            }
            JsonValueTemplate::Object(map_template) => {
                if let JsonValue::Object(map_object) = object {
                    map_template
                        .into_iter()
                        .map(|(key, template)| {
                            let obj = map_object.get(key).unwrap_or(&JsonValue::Null);
                            template.check(obj)
                        })
                        .all(|it| it)
                } else {
                    false
                }
            }
            JsonValueTemplate::String => {
                if let JsonValue::String(_) = object {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn trim(&self, object: &mut JsonValue) {
        match self {
            JsonValueTemplate::Option(template) => {
                if !template.check(object) {
                    *object = JsonValue::Null;
                }
            }
            JsonValueTemplate::Number => {
                if let JsonValue::Number(_) = object {} else {
                    *object = JsonValue::Null;
                }
            }
            JsonValueTemplate::Boolean => {
                if let JsonValue::Bool(_) = object {} else {
                    *object = JsonValue::Null;
                }
            }
            JsonValueTemplate::Array(array_template) => {
                if let JsonValue::Array(array_object) = object {
                    array_object.retain(|it| array_template.check(it));
                    if array_object.len() == 0 {
                        *object = JsonValue::Null;
                    }
                } else {
                    *object = JsonValue::Null;
                }
            }
            JsonValueTemplate::Object(map_template) => {
                if let Value::Object(map_object) = object {
                    map_object.retain(|key, value| {
                        if let Some(template) = map_template.get(key) {
                            template.trim(value);
                            template.check(value)
                        } else {
                            false
                        }
                    });
                    if map_object.len() == 0 {
                        *object = JsonValue::Null;
                    }
                } else {
                    *object = JsonValue::Null;
                }
            }
            JsonValueTemplate::String => {
                if let JsonValue::String(_) = object {} else {
                    *object = JsonValue::Null;
                }
            }
        }
    }
}
