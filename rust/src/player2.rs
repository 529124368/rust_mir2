use crate::man_base::ManBase;
use gdnative::api::*;
use gdnative::prelude::*;
use lsz_macro::lszMacro;
use std::ops::Deref;
use std::ops::DerefMut;

/// The Player2 "class"
#[derive(NativeClass, lszMacro)]
#[inherit(Area2D)]
#[register_with(Self::register_builder)]
pub struct Player2 {
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
    #[property]
    sprite_name: String,
    //影子偏移坐标
    #[property]
    shadow_offset_x: f32,
    #[property]
    shadow_offset_y: f32,
    //是否在碰撞中
    is_block: bool,
    //各个动作的动画帧数
    step_nums: [u8; 3],
}

#[methods]
impl Player2 {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Player2 builder is registered!");
    }
    fn new(_owner: &Area2D) -> Self {
        godot_print!("Player2 is created!");
        Player2 {
            sprite_name: "".to_string(),
            man: ManBase::new(),
            timer_idel: 0.2,
            timer_attack: 0.2,
            timer_run: 0.2,
            timer_flg: 0.2,
            move_speed: 100.0,
            //偏移量
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            is_block: false,
            step_nums: [4, 6, 6],
        }
    }
    #[export]
    unsafe fn _ready(&mut self, _owner: &Area2D) {}

    #[export]
    unsafe fn _process(&self, _owner: &Area2D, delta: f64) {}
}
