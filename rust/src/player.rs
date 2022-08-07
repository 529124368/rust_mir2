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
impl Player {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Player builder is registered!");
        _builder.signal("enter").done();
    }

    /// The "constructor" of the class.
    fn new(_owner: &Area2D) -> Self {
        Player {
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
            step_nums: [16, 8, 15],
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: &Area2D) {
        //碰撞信号注册
        self.bind_signal_method(_owner, "body_entered", "_on_player_enter");
        //碰撞退出信号注册
        self.bind_signal_method(_owner, "body_exited", "_on_player_exit");
        //UI信号接受
        // self.bind_signal_method_by_path(
        //     _owner,
        //     "../../CanvasLayer/minibar",
        //     "move_mini_panel_to_left",
        //     "_move_panel_to_left",
        // );
        // self.bind_signal_method_by_path(
        //     _owner,
        //     "../../CanvasLayer/minibar",
        //     "move_mini_panel_to_right",
        //     "_move_panel_to_right",
        // );
        TARGETS = _owner.position();
        //加载素材
        let a = self.sprite_name.clone();
        self.load_assets(
            &("res://assets/man/".to_string() + &a + ".json"),
            &("res://assets/man/".to_string() + &a + ".png"),
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
        let input = Input::godot_singleton();
        //轮图
        TIMER_TICK += delta;
        if TIMER_TICK > self.timer_flg {
            let index = match self.state {
                man_base::Action::Idle(a) => a,
                man_base::Action::Run(b) => b,
                man_base::Action::Attack(c) => c,
            };
            TIMER_TICK = 0.0;
            SUM %= self.step_nums[index as usize];

            //裁剪图集
            let n = (&self.anim_name).to_string() + &SUM.to_string() + ".png";
            let json_name = self.json_data.get(&n).unwrap();

            self.render_sprite_and_shadow(json_name, self.shadow_offset_x, self.shadow_offset_y);
            SUM += 1;
        }

        //鼠标右键点击 攻击
        if Input::is_mouse_button_pressed(&input, 2) && self.state == Action::Idle(0) {
            if self.state != man_base::Action::Attack(2) {
                SUM = 0;
            }
            self.state = man_base::Action::Attack(2);
            //修改播放速度
            self.timer_flg = self.timer_attack;
            self.anim_name = self.dir.to_string() + "_attack_";
        }

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
                self.dir = tools::math::cal_d(tools::math::cal_dir(CENTER, target)) as u8;
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

    // #[export]
    // unsafe fn _move_panel_to_left(&mut self, _owner: &Area2D) {
    //     let camera = _owner
    //         .get_node("Camera2D")
    //         .unwrap()
    //         .assume_safe()
    //         .cast::<Camera2D>()
    //         .unwrap();
    //     camera.set_position(Vector2 {
    //         x: camera.position().x + 155.0,
    //         y: camera.position().y,
    //     });
    //     CENTER = Vector2 {
    //         x: CENTER.x - 200.0,
    //         y: CENTER.y,
    //     };
    // }

    // #[export]
    // unsafe fn _move_panel_to_right(&mut self, _owner: &Area2D) {
    //     let camera = _owner
    //         .get_node("Camera2D")
    //         .unwrap()
    //         .assume_safe()
    //         .cast::<Camera2D>()
    //         .unwrap();
    //     camera.set_position(Vector2 {
    //         x: camera.position().x - 155.0,
    //         y: camera.position().y,
    //     });
    //     CENTER = Vector2 {
    //         x: CENTER.x + 200.0,
    //         y: CENTER.y,
    //     };
    // }

    //重新加载资源 (godot脚本调用的外部接口)
    #[export]
    unsafe fn _reload_asset(&mut self, _owner: &Area2D) {
        let name = self.sprite_name.clone();
        self.load_assets(
            &("res://assets/man/".to_string() + &name + ".json"),
            &("res://assets/man/".to_string() + &name + ".png"),
        );
    }
    //绑定其他节点信号
    unsafe fn bind_signal_method_by_path(
        &self,
        _owner: &Area2D,
        node_path: &str,
        signal: &str,
        method: &str,
    ) {
        let emit = _owner.get_node(node_path).unwrap().assume_safe();
        emit.connect(
            signal,
            _owner.assume_shared(),
            method,
            VariantArray::new_shared(),
            0,
        )
        .unwrap();
    }

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
