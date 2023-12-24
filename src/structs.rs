use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Message {
    pub message: &'static str,
}

#[derive(Serialize, Deserialize)]
pub struct Premium {
    pub uuid: String,
    pub active: bool,
    pub ends: i64,
    pub name: String,
    pub starts: i64,
    pub success: bool,
}

#[derive(Deserialize)]
pub struct MojangProfile {
    pub name: String,
    pub id: String,
}
