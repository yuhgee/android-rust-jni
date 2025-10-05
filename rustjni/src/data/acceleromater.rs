use jni::objects::{JClass, JObject, JValue, JString};
use serde::{de, Deserialize};

use crate::data::SensorData;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Accelerometer {
    #[serde(flatten)]
    pub sensor_data: SensorData,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


// ジェネリックにしてもいいね
pub fn parse_json(json: &str) -> Result<Accelerometer, serde_json::Error> {
    let value: Accelerometer = serde_json::from_str(json)?;
    log::debug!("parse_json: {:?}", value);
    Ok(value)
}
