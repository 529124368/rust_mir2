use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Mul;

use gdnative::api::*;
use gdnative::prelude::*;
use lsz_macro::lszMacro;

use crate::man_base;
use crate::man_base::ManBase;

/// The OtherPlayer "class"
#[derive(NativeClass, lszMacro)]
#[inherit(Area2D)]
#[register_with(Self::register_builder)]
pub struct OtherPlayer {
    man: ManBase,
    #[property]
    timer_idel: f64,
    #[property]
    sprite_name: String,
    #[property]
    timer_run: f64,
    #[property]
    timer_attack: f64,
    #[property]
    move_speed: f32,
    #[property]
    disection_diy: u8,
    timer_flg: f64,
    step_nums: [u8; 3],
    timer_tick: f64,
    sum: u8,
    speed: Vector2,
    target: Vector2,
}

#[methods]
impl OtherPlayer {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("OtherPlayer builder is registered!");
    }

    fn new(_owner: &Area2D) -> Self {
        OtherPlayer {
            disection_diy: 0,
            sprite_name: "man".to_string(),
            man: ManBase::new(),
            timer_idel: 0.2,
            timer_attack: 0.2,
            timer_run: 0.2,
            timer_flg: 0.2,
            move_speed: 100.0,
            step_nums: [4, 6, 6], //idle run attack 的图片个数
            timer_tick: 0.0,
            sum: 0,
            speed: Vector2 { x: 0.0, y: 0.0 },
            target: Vector2 { x: 0.0, y: 0.0 },
        }
    }

    #[godot]
    unsafe fn _ready(&mut self, #[base] _owner: &Area2D) {
        //初始化状态
        self.target = _owner.position();
        self.dir = self.disection_diy;
        //加载素材
        let a = self.sprite_name.to_string();
        self.load_assets_for_mir(&a, "zhanshi", "tulong");
        //获取精灵节点
        let w = _owner
            .get_node_as("YSort/Sprite")
            .and_then(|f: TRef<Sprite>| f.cast::<Sprite>())
            .unwrap();
        //保存精灵节点
        self.play_sprite.push(Some(w.claim()));
        //获取武器节点
        let w = _owner
            .get_node_as("YSort/weapon")
            .and_then(|f: TRef<Sprite>| f.cast::<Sprite>())
            .unwrap();
        //保存武器节点
        self.play_sprite.push(Some(w.claim()));
        //获取技能节点
        let w = _owner
            .get_node_as("skill")
            .and_then(|f: TRef<Sprite>| f.cast::<Sprite>())
            .unwrap();
        self.skill_sprite = Some(w.claim());
    }

    #[godot]
    unsafe fn _process(&mut self, #[base] _owner: &Area2D, delta: f64) {
        //轮图
        self.timer_tick += delta;
        if self.timer_tick > self.timer_flg {
            let action = match self.state {
                man_base::Action::Idle(a) => a,
                man_base::Action::Run(b) => b,
                man_base::Action::Attack(c) => c,
            };
            self.timer_tick = 0.0;
            self.sum %= self.step_nums[action as usize];

            //裁剪图集
            self.render_sprite(action, self.sum);
            //
            if action == 2 {
                self.render_skill(self.sum);
            } else {
                self.skill_sprite
                    .unwrap()
                    .assume_safe()
                    .set_texture(Texture::null());
            }
            self.sum += 1;
        }

        //移动
        self.speed = _owner.position().direction_to(self.target) * self.move_speed;
        if _owner.position().distance_to(self.target) > 3.0 {
            self.state = man_base::Action::Run(1);
            //修改播放速度
            self.timer_flg = self.timer_run;
            self.anim_name = self.dir.to_string() + "_run_";
            let pos = self.speed.mul(delta as f32);
            let newx = _owner.position().x + pos.x;
            let newy = _owner.position().y + pos.y;
            _owner.set_position(Vector2 { x: newx, y: newy });
        } else if self.state != man_base::Action::Attack(2) {
            self.state = man_base::Action::Idle(0);
            //修改播放速度
            self.timer_flg = self.timer_idel;
            self.anim_name = self.dir.to_string() + "_stand_";
        }
    }
}
