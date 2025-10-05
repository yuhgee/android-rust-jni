use jni::objects::{JClass, JObject, JValue, JString};
use serde::{de, Deserialize};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SensorData {
    pub timestamp: u64,
}
