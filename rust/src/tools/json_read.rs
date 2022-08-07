use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub frames: HashMap<String, T>,
    pub meta: Meta,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct T {
    pub frame: Frame,
    pub rotated: bool,
    pub trimmed: bool,
    pub spriteSourceSize: SpriteSourceSize,
    pub sourceSize: SourceSize,
    pub pivot: Pivot,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    pub size: Size,
    pub scale: String,
    pub smartupdate: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    pub w: i64,
    pub h: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Pivot {
    pub x: f64,
    pub y: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceSize {
    pub w: i64,
    pub h: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteSourceSize {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub fn getjson(s: &str) -> Root {
    let v: Root = serde_json::from_str(s).unwrap();
    v
}
