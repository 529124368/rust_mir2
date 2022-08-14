use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MsgBase {
    pub mes_type: u8,
    pub send_id: u32,
    pub message: String,
    pub position: Vector2,
}

impl MsgBase {
    pub fn new(mes_type: u8, send_id: u32, message: String) -> Self {
        Self {
            mes_type,
            send_id,
            message,
            position: Vector2::new(0.0, 0.0),
        }
    }
}
