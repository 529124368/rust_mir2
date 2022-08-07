use gdnative::{
    api::{AtlasTexture, Texture},
    core_types::{Rect2, Vector2},
    object::{AsArg, Ref},
    prelude::Unique,
};

use super::json_read::T;

pub fn get_img_by_name(atlas: impl AsArg<Texture>, json_name: &T) -> Ref<AtlasTexture, Unique> {
    let boxs = AtlasTexture::new();
    AtlasTexture::set_atlas(&boxs, atlas);
    AtlasTexture::set_region(
        &boxs,
        Rect2 {
            position: Vector2 {
                x: json_name.frame.x,
                y: json_name.frame.y,
            },
            size: Vector2 {
                x: json_name.frame.w,
                y: json_name.frame.h,
            },
        },
    );
    boxs
}
