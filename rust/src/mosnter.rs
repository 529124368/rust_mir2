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
    unsafe fn _ready(&mut self, _owner: &Area2D) {}

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&mut self, _owner: &Area2D, delta: f64) {
        //轮图
    }
}
