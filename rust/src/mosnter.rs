use std::ops::Deref;
use std::ops::DerefMut;

use gdnative::api::*;
use gdnative::prelude::*;

use crate::man_base;
use crate::man_base::ManBase;
use crate::tools;

/// The Mosnter "class"
#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_builder)]
pub struct Mosnter {
    man: ManBase,
    #[property]
    timer_idel: f64,
    #[property]
    timer_run: f64,
    #[property]
    timer_attack: f64,
    timer_flg: f64,
    #[property]
    move_speed: f32,
    //偏移坐标
    #[property]
    shadow_offset_x: f32,
    #[property]
    shadow_offset_y: f32,
    //是否在碰撞中
    //is_block: bool,
    step_nums: [u8; 3],
    timer_tick: f64,
    sum: u8,
}
impl Deref for Mosnter {
    type Target = ManBase;

    fn deref(&self) -> &ManBase {
        &self.man
    }
}

impl DerefMut for Mosnter {
    fn deref_mut(&mut self) -> &mut ManBase {
        &mut self.man
    }
}
#[methods]
impl Mosnter {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Mosnter builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &Area2D) -> Self {
        Mosnter {
            man: ManBase::new(),
            timer_idel: 0.2,
            timer_attack: 0.2,
            timer_run: 0.2,
            timer_flg: 0.2,
            move_speed: 100.0,
            //偏移量xxxx
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            // is_block: false,
            step_nums: [20, 8, 10],
            timer_tick: 0.0,
            sum: 0,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: &Area2D) {
        self.timer_flg = self.timer_idel;
        //加载素材
        self.load_assets(
            "res://assets/monster/FC.json",
            "res://assets/monster/FC.png",
        );
        //获取精灵节点
        let s = _owner
            .get_node_as("Sprite")
            .and_then(|f: TRef<Sprite>| f.cast::<Sprite>())
            .unwrap();
        //保存精灵节点
        self.play_sprite = Some(s.claim());
        //获取影子节点
        let shadow = _owner
            .get_node_as("shadow")
            .and_then(|f: TRef<Sprite>| f.cast::<Sprite>())
            .unwrap();
        //保存精灵节点
        self.shadow_sprite = Some(shadow.claim());
        //裁剪图集
        let json_name = self.json_data.get("0_stand_0.png").unwrap();

        //更新图片
        s.set_texture(tools::get_texture::get_img_by_name(
            self.img_assets.as_ref().unwrap(),
            json_name,
        ));
        shadow.set_texture(tools::get_texture::get_img_by_name(
            self.img_assets.as_ref().unwrap(),
            json_name,
        ));
    }

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&mut self, _owner: &Area2D, delta: f64) {
        //轮图
        self.timer_tick += delta;
        if self.timer_tick > self.timer_flg {
            let index = match self.state {
                man_base::Action::Idle(a) => a,
                man_base::Action::Run(b) => b,
                man_base::Action::Attack(c) => c,
            };
            self.timer_tick = 0.0;
            self.sum %= self.step_nums[index as usize];

            //裁剪图集
            let n = (&self.anim_name).to_string() + &self.sum.to_string() + ".png";
            let json_name = self.json_data.get(&n).unwrap();

            self.render_sprite_and_shadow(json_name, self.shadow_offset_x, self.shadow_offset_y);
            self.sum += 1;
        }
    }
}
