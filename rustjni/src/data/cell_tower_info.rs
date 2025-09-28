use jni::objects::{JClass, JObject, JValue, JString};
use serde::{de, Deserialize};

// CellTowerInfo 構造体
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CellTowerInfo {
    #[serde(rename = "type")]
    pub type_: String,
    pub mcc: Option<i32>,
    pub mnc: Option<i32>,
    pub tac: Option<i32>,
    pub cellId: Option<i32>,
    pub pci: Option<i32>,
    pub signalLevel: i32,
}

fn parse_json(json: &str) -> Result<Vec<CellTowerInfo>, serde_json::Error> {
    let towers: Vec<CellTowerInfo> = serde_json::from_str(json)?;
    log::debug!("Cell Tower Info: {:?}", towers);
    Ok(towers)
}
