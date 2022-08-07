use gdnative::{
    api::File,
    prelude::{Ref, ResourceLoader, Sprite, Texture, Vector2},
};
use std::collections::HashMap;

use crate::tools::{self, json_read::T};

#[derive(PartialEq)]
pub enum Action {
    Idle(u8),
    Run(u8),
    Attack(u8),
}

//人物基类
pub struct ManBase {
    //json缓存
    pub json_data: HashMap<String, tools::json_read::T>,
    //图集缓存
    pub img_assets: Option<Ref<Texture>>,
    pub play_sprite: Option<Ref<Sprite>>,
    pub shadow_sprite: Option<Ref<Sprite>>,
    pub dir: u8,
    pub anim_name: String,
    pub state: Action,
}

impl ManBase {
    pub fn new() -> Self {
        ManBase {
            json_data: HashMap::new(),
            img_assets: None,
            play_sprite: None,
            //当前状态
            state: Action::Idle(0),
            shadow_sprite: None,
            dir: 0,
            anim_name: "0_stand_".to_string(),
        }
    }
    //加载资源
    pub fn load_assets(&mut self, json_path: &str, image_path: &str) {
        //json 加载
        let json_file = File::new();
        File::open(&json_file, json_path, File::READ).unwrap();
        let s = json_file.get_as_text();
        let s = tools::json_read::getjson(&s.to_string());
        self.json_data = s.frames;
        File::close(&json_file);

        //加载资源
        let im = ResourceLoader::godot_singleton();
        self.img_assets =
            ResourceLoader::load(&im, image_path, "", false).and_then(|s| s.cast::<Texture>());
    }
    //渲染精灵和影子
    pub unsafe fn render_sprite_and_shadow(
        &self,
        json_name: &T,
        shadow_offset_x: f32,
        shadow_offset_y: f32,
    ) {
        //获取精灵
        let s = self.play_sprite.unwrap().assume_safe();
        //更新图片
        s.set_texture(tools::get_texture::get_img_by_name(
            self.img_assets.as_ref().unwrap(),
            json_name,
        ));
        //更新偏移
        if self.state == Action::Attack(2) {
            s.set_position(Vector2 {
                x: json_name.spriteSourceSize.x - 45.0,
                y: json_name.spriteSourceSize.y - 15.0,
            });
        } else {
            s.set_position(Vector2 {
                x: json_name.spriteSourceSize.x,
                y: json_name.spriteSourceSize.y,
            });
        }
        //获取影子
        let sh = self.shadow_sprite.unwrap().assume_safe();
        //更新图片
        sh.set_texture(tools::get_texture::get_img_by_name(
            self.img_assets.as_ref().unwrap(),
            json_name,
        ));
        //更新偏移
        if self.state == Action::Attack(2) {
            sh.set_position(Vector2 {
                x: json_name.spriteSourceSize.x + shadow_offset_x,
                y: json_name.spriteSourceSize.y + shadow_offset_y,
            });
        } else {
            sh.set_position(Vector2 {
                x: json_name.spriteSourceSize.x,
                y: json_name.spriteSourceSize.y,
            });
        }
    }
}
