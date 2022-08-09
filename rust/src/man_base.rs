use gdnative::{
    api::File,
    prelude::{Ref, ResourceLoader, Sprite, Texture, Vector2},
};
use std::collections::HashMap;

use crate::tools::{
    self,
    json_read::{Offset, Skill, T},
};

#[derive(PartialEq, Eq)]
pub enum Action {
    Idle(u8),
    Run(u8),
    Attack(u8),
}

//人物基类
pub struct ManBase {
    //json缓存
    pub json_data: Vec<HashMap<String, T>>,
    pub json_offset: Vec<Offset>,
    //图集缓存
    pub img_assets: Vec<Option<Ref<Texture>>>,
    pub play_sprite: Vec<Option<Ref<Sprite>>>,
    pub dir: u8,
    pub anim_name: String,
    pub state: Action,
    pub nodes: [String; 2],
    //技能相关
    pub skill_offset: Skill,
    pub skill_json: HashMap<String, T>,
    pub skill_img_asset: Option<Ref<Texture>>,
    pub skill_sprite: Option<Ref<Sprite>>,
    pub skill_name: String,
}

impl ManBase {
    pub fn new() -> Self {
        ManBase {
            json_data: Vec::new(),
            json_offset: Vec::new(),
            img_assets: Vec::new(),
            play_sprite: Vec::new(),
            //当前状态
            state: Action::Idle(0),
            dir: 0,
            anim_name: "0_stand_".to_string(),
            nodes: ["man".to_string(), "weapon".to_string()],
            //技能相关
            skill_offset: Skill::default(),
            skill_json: HashMap::new(),
            skill_img_asset: None,
            skill_sprite: None,
            skill_name: "liehuo".to_string(),
        }
    }

    //渲染精灵 传奇用
    pub unsafe fn render_sprite(&self, state: u8, frame_num: u8) {
        for i in 0..self.nodes.len() {
            //获取精灵
            let s = self.play_sprite[i].unwrap().assume_safe();
            //更新图片
            let n = (self.anim_name).to_string() + &frame_num.to_string() + ".png";
            s.set_texture(tools::get_texture::get_img_by_name(
                self.img_assets[i].as_ref().unwrap(),
                self.json_data[i].get(&n).unwrap(),
            ));
            //更新偏移
            let ss = match state {
                state if state == 0 => self.json_offset[i]
                    .stand
                    .get(self.dir as usize)
                    .unwrap()
                    .get(frame_num as usize)
                    .unwrap(),
                state if state == 1 => self.json_offset[i]
                    .run
                    .get(self.dir as usize)
                    .unwrap()
                    .get(frame_num as usize)
                    .unwrap(),
                state if state == 2 => self.json_offset[i]
                    .attack
                    .get(self.dir as usize)
                    .unwrap()
                    .get(frame_num as usize)
                    .unwrap(),
                _ => todo!(),
            };
            let res: Vec<&str> = ss.split('_').collect();
            let res: Vec<f32> = res.iter().map(|x| x.parse::<f32>().unwrap()).collect();
            s.set_position(Vector2 {
                x: res[0],
                y: res[1],
            });
        }
    }

    //渲染技能 传奇用
    pub unsafe fn render_skill(&self, frame_num: u8) {
        //获取精灵
        let s = self.skill_sprite.unwrap().assume_safe();
        //更新图片
        let n =
            (self.dir).to_string() + "_" + &self.skill_name + "_" + &frame_num.to_string() + ".png";
        s.set_texture(tools::get_texture::get_img_by_name(
            self.skill_img_asset.as_ref().unwrap(),
            self.skill_json.get(&n).unwrap(),
        ));
        //更新偏移
        let ss = match &self.skill_name {
            skill if skill == "liehuo" => self
                .skill_offset
                .liehuo
                .get(self.dir as usize)
                .unwrap()
                .get(frame_num as usize)
                .unwrap(),
            skill if skill == "banyue" => self
                .skill_offset
                .banyue
                .get(self.dir as usize)
                .unwrap()
                .get(frame_num as usize)
                .unwrap(),
            _ => todo!(),
        };
        let res: Vec<&str> = ss.split('_').collect();
        let res: Vec<f32> = res.iter().map(|x| x.parse::<f32>().unwrap()).collect();
        s.set_position(Vector2 {
            x: res[0],
            y: res[1],
        });
    }
    //加载资源 传奇用
    pub fn load_assets_for_mir(&mut self, name: &str) {
        //加载技能
        self.load_assets_for_skill(name);
        //加载玩家 武器
        for i in 0..self.nodes.len() {
            //人物
            let json_path = &("res://assets/".to_string() + &self.nodes[i] + "/" + name + ".json");
            let image_path = &("res://assets/".to_string() + &self.nodes[i] + "/" + name + ".png");
            let json_offset = &("res://assets/".to_string() + &self.nodes[i] + "/data.json");
            //json 加载
            let res = self.common_load(json_path, image_path, json_offset);
            //加载偏移json offset
            self.json_data.push(res.0);
            //加载资源
            self.img_assets.push(res.1);
            self.json_offset.push(res.2);
        }
    }

    //加载资源 传奇用
    fn load_assets_for_skill(&mut self, name: &str) {
        //人物
        let json_path = &("res://assets/skill/".to_string() + name + ".json");
        let image_path = &("res://assets/skill/".to_string() + name + ".png");
        let json_offset = "res://assets/skill//data.json";
        //json 加载
        let res = self.common_load_skill(json_path, image_path, json_offset);
        //加载偏移json offset
        self.skill_json = res.0;
        //加载资源
        self.skill_img_asset = res.1;
        self.skill_offset = res.2;
    }

    //共通方法
    fn common_load(
        &self,
        json_path: &str,
        image_path: &str,
        json_offset: &str,
    ) -> (HashMap<String, T>, Option<Ref<Texture>>, Offset) {
        let json_file = File::new();
        //json 加载
        File::open(&json_file, json_path, File::READ).unwrap();
        let s = json_file.get_as_text();
        let s = tools::json_read::getjson(&s.to_string());
        let frames = s.frames;
        //加载资源
        let im = ResourceLoader::godot_singleton();
        let asset =
            ResourceLoader::load(im, image_path, "", false).and_then(|s| s.cast::<Texture>());
        //加载偏移json offset
        File::open(&json_file, json_offset, File::READ).unwrap();
        let s = json_file.get_as_text();
        let offset = tools::json_read::getjson_offset(&s.to_string());
        File::close(&json_file);
        (frames, asset, offset)
    }

    fn common_load_skill(
        &self,
        json_path: &str,
        image_path: &str,
        json_offset: &str,
    ) -> (HashMap<String, T>, Option<Ref<Texture>>, Skill) {
        let json_file = File::new();
        //json 加载
        File::open(&json_file, json_path, File::READ).unwrap();
        let s = json_file.get_as_text();
        let s = tools::json_read::getjson(&s.to_string());
        let frames = s.frames;
        //加载资源
        let im = ResourceLoader::godot_singleton();
        let asset =
            ResourceLoader::load(im, image_path, "", false).and_then(|s| s.cast::<Texture>());
        //加载偏移json offset
        File::open(&json_file, json_offset, File::READ).unwrap();
        let s = json_file.get_as_text();
        let offset = tools::json_read::getjson_skill(&s.to_string());
        File::close(&json_file);
        (frames, asset, offset)
    }
    //改变技能
    pub fn change_skill(&mut self, name: String) {
        self.skill_name = name;
    }
}
