use gdnative::api::*;
use gdnative::prelude::*;
use lsz_macro::lszMacro;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Mul;

use crate::man_base;
use crate::man_base::Action;
use crate::man_base::ManBase;
use crate::tools;
static mut SUM: u8 = 0;
static mut TIMER_TICK: f64 = 0.0;
static mut TARGETS: Vector2 = Vector2 { x: 0.0, y: 0.0 };
static mut SPEED: Vector2 = Vector2 { x: 0.0, y: 0.0 };
static mut CENTER: Vector2 = Vector2 { x: 400.0, y: 300.0 };
static mut PRESS: bool = false;
/// The Player "class"
#[derive(NativeClass, lszMacro)]
#[inherit(Area2D)]
#[register_with(Self::register_builder)]
pub struct Player {
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
    //是否在碰撞中
    is_block: bool,
    //各个动作的动画帧数
    step_nums: [u8; 3],
}

#[methods]
impl Player {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        _builder.signal("enter").done();
    }

    /// The "constructor" of the class.
    fn new(_owner: &Area2D) -> Self {
        Player {
            sprite_name: "man".to_string(),
            man: ManBase::new(),
            timer_idel: 0.2,
            timer_attack: 0.2,
            timer_run: 0.2,
            timer_flg: 0.2,
            move_speed: 100.0,
            is_block: false,
            step_nums: [4, 6, 6],
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: &Area2D) {
        //碰撞信号注册
        self.bind_signal_method(_owner, "body_entered", "_on_player_enter");
        //碰撞退出信号注册
        self.bind_signal_method(_owner, "body_exited", "_on_player_exit");
        TARGETS = _owner.position();
        //加载素材
        let a = self.sprite_name.clone();
        self.load_assets_for_mir(&a);
        //获取精灵节点
        let w = _owner
            .get_node_as("Sprite")
            .and_then(|f: TRef<Sprite>| f.cast::<Sprite>())
            .unwrap();
        //保存精灵节点
        self.play_sprite.push(Some(w.claim()));
        //获取武器节点
        let w = _owner
            .get_node_as("weapon")
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

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&mut self, _owner: &Area2D, delta: f64) {
        let input = Input::godot_singleton();
        //轮图
        TIMER_TICK += delta;
        if TIMER_TICK > self.timer_flg {
            let action = match self.state {
                man_base::Action::Idle(a) => a,
                man_base::Action::Run(b) => b,
                man_base::Action::Attack(c) => c,
            };
            TIMER_TICK = 0.0;
            SUM %= self.step_nums[action as usize];

            //裁剪图集
            self.render_sprite(action, SUM);
            //
            if action == 2 {
                self.render_skill(SUM);
            }
            SUM += 1;
        }

        //移动
        SPEED = _owner.position().direction_to(TARGETS) * self.move_speed;
        if _owner.position().distance_to(TARGETS) > 3.0 && !self.is_block {
            self.state = man_base::Action::Run(1);
            //修改播放速度
            self.timer_flg = self.timer_run;
            self.anim_name = self.dir.to_string() + "_run_";
            let pos = SPEED.mul(delta as f32);
            let newx = _owner.position().x + pos.x;
            let newy = _owner.position().y + pos.y;
            _owner.set_position(Vector2 { x: newx, y: newy });
        } else if self.state != man_base::Action::Attack(2) {
            self.state = man_base::Action::Idle(0);
            //修改播放速度
            self.timer_flg = self.timer_idel;
            self.anim_name = self.dir.to_string() + "_stand_";
        }

        //鼠标右键点击 攻击
        if Input::is_mouse_button_pressed(&input, 2) && self.state == Action::Idle(0) {
            let audio = _owner
                .get_node_as("audio")
                .and_then(|t: TRef<AudioStreamPlayer2D>| t.cast::<AudioStreamPlayer2D>())
                .unwrap();
            if !audio.is_playing() {
                audio.play(0.0);
            }
            if self.state != man_base::Action::Attack(2) {
                SUM = 0;
            }
            self.state = man_base::Action::Attack(2);
            //修改播放速度
            self.timer_flg = self.timer_attack;
            self.anim_name = self.dir.to_string() + "_attack_";
        }
    }

    #[export]
    unsafe fn _unhandled_input(&mut self, _owner: &Area2D, _event: Ref<InputEvent>) {
        if _event
            .assume_safe()
            .is_action_pressed("mouse_left", false, false)
            || PRESS
        {
            PRESS = true;
            if let Some(viewport) = _owner.get_viewport().map(|f| f.assume_safe()) {
                let target = viewport.get_mouse_position();
                self.dir = tools::math::cal_d(tools::math::cal_dir(CENTER, target));
                TARGETS = tools::math::add(_owner.position(), tools::math::sub(CENTER, target));
                let state2d = _owner
                    .get_world_2d()
                    .unwrap()
                    .assume_safe()
                    .direct_space_state()
                    .unwrap()
                    .assume_safe()
                    .intersect_ray(
                        _owner.position(),
                        TARGETS,
                        VariantArray::new_shared(),
                        2147483647,
                        true,
                        false,
                    );
                if state2d.is_empty() {
                    self.is_block = false;
                }
            }
        }
        if _event.assume_safe().is_action_released("mouse_left", false) {
            PRESS = false;
        }
    }

    #[export]
    unsafe fn _on_player_enter(&mut self, _owner: &Area2D, _data: Variant) {
        //godot_print!("发生了碰撞");
        TARGETS = _owner.position();
        self.is_block = true;
        _owner.emit_signal("enter", &[]);
    }

    #[export]
    unsafe fn _on_player_exit(&mut self, _owner: &Area2D, _data: Variant) {
        self.is_block = false;
        //godot_print!("退出碰撞");
    }

    // //绑定其他节点信号
    // unsafe fn bind_signal_method_by_path(
    //     &self,
    //     _owner: &Area2D,
    //     node_path: &str,
    //     signal: &str,
    //     method: &str,
    // ) {
    //     let emit = _owner.get_node(node_path).unwrap().assume_safe();
    //     emit.connect(
    //         signal,
    //         _owner.assume_shared(),
    //         method,
    //         VariantArray::new_shared(),
    //         0,
    //     )
    //     .unwrap();
    // }

    //绑定本身的信号
    unsafe fn bind_signal_method(&self, _owner: &Area2D, signal: &str, method: &str) {
        _owner
            .connect(
                signal,
                _owner.assume_shared(),
                method,
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
    }
}
