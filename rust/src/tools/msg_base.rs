use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MsgBase {
    pub mes_type: u8,
    pub send_id: u32,
    pub message: String,
}

impl MsgBase {
    pub fn new(mes_type: u8, send_id: u32, message: String) -> Self {
        Self {
            mes_type,
            send_id,
            message,
        }
    }
}
