use crate::tools;
use futures_util::{future, pin_mut, StreamExt};
use gdnative::api::*;
use gdnative::prelude::*;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time;
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// The Websocket "class"

#[derive(NativeClass, Clone)]
#[inherit(Node2D)]
#[register_with(Self::register_builder)]
pub struct Websocket {
    url: String,
    send_channel: Arc<RwLock<Option<UnboundedSender<String>>>>,
    msg_input: Option<Ref<RichTextLabel>>,
    my_node: Option<Ref<Node2D>>,
}
#[methods]
impl Websocket {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Websocket builder is registered!");
    }

    fn new(_owner: &Node2D) -> Self {
        Websocket {
            url: "ws:127.0.0.1/chat".to_string(),
            send_channel: Arc::new(RwLock::new(None)),
            msg_input: None,
            my_node: None,
        }
    }

    #[godot]
    unsafe fn _ready(&mut self, #[base] _owner: TRef<Node2D>) {
        self.my_node = Some(_owner.claim());
        //获取输入节点
        let w = _owner
            .get_node_as("../../CanvasLayer/message/msg")
            .and_then(|f: TRef<RichTextLabel>| f.cast::<RichTextLabel>())
            .unwrap();
        self.msg_input = Some(w.claim());
        //绑定输入信号注册
        self.bind_signal_method_by_path(
            _owner.as_ref(),
            "../../CanvasLayer/message/input",
            "text_entered",
            "_on_input_enter",
        );
        //绑定玩家移动信号
        self.bind_signal_method_by_path(_owner.as_ref(), "../Player", "move", "_on_move");

        //消息管道
        let (chane1, chanel2) = tokio::sync::mpsc::unbounded_channel::<String>();
        self.send_channel = Arc::new(RwLock::new(Some(chane1)));
        let mut ws = self.clone();
        //websocket服务器链接 start
        thread::spawn(move || {
            let a = tokio::runtime::Runtime::new().unwrap();
            a.block_on(async move {
                ws.conn(chanel2).await;
            });
        });
    }

    //链接
    async fn conn(&mut self, mut chanel2: UnboundedReceiver<String>) {
        let url = url::Url::parse(&self.url).unwrap();
        //消息传送管道
        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();

        tokio::task::spawn(async move {
            while let Some(s) = chanel2.recv().await {
                stdin_tx
                    .unbounded_send(Message::binary(s.into_bytes()))
                    .unwrap();
            }
        });

        //心跳检测
        let wss = self.clone();
        tokio::task::spawn(async move {
            let mut t = time::interval(Duration::from_secs(50));
            loop {
                t.tick().await;
                let msg = tools::msg_base::MsgBase::new(3, 0, "".to_string());
                let msg = serde_json::to_string(&msg).unwrap();
                wss.send_mesg(msg);
            }
        });

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        let (write, read) = ws_stream.split();

        //往服务器发送消息
        let stdin_to_ws = stdin_rx.map(Ok).forward(write);

        //读取服务器消息
        let ws_to_stdout = read.for_each(|message| async {
            match message {
                Ok(s) => {
                    let data = s.into_data();
                    let msg = std::str::from_utf8(&data).unwrap();
                    //传入handle
                    unsafe {
                        self.do_handle(msg);
                    }
                }
                Err(_) => {
                    panic!()
                }
            };
        });
        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    }

    #[godot]
    //聊天消息
    unsafe fn _on_input_enter(&mut self, _data: Variant) {
        let msg = tools::msg_base::MsgBase::new(1, 0, _data.to_string());
        let msg = serde_json::to_string(&msg).unwrap();
        self.send_mesg(msg);
    }

    #[godot]
    //玩家移动信号
    unsafe fn _on_move(&mut self, _data: Variant) {
        let reee: Vector2 = _data.try_to().unwrap();
        let mut msg = tools::msg_base::MsgBase::new(2, 0, "move".to_string());
        msg.position = tools::msg_base::Vector2::new(reee.x, reee.y);
        let msg = serde_json::to_string(&msg).unwrap();
        self.send_mesg(msg);
    }

    //发送消息
    fn send_mesg(&self, msg: String) {
        if let Ok(s) = self.send_channel.write() {
            if let Some(b) = s.as_ref() {
                let _ = b.send(msg);
            }
        }
    }

    //绑定其他节点信号
    unsafe fn bind_signal_method_by_path(
        &self,
        _owner: &Node2D,
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

    unsafe fn load_role(&self, id: u32) {
        let role = ResourceLoader::godot_singleton().load(
            "res://scenes/otherPlayer.tscn",
            "PackedScene",
            false,
        );
        if let Some(s) = role.and_then(|f| f.cast::<PackedScene>()) {
            let instance = s.assume_safe().instance(0);
            if let Some(instance) = instance {
                instance.assume_safe().set_name(id.to_string());
                self.my_node
                    .unwrap()
                    .assume_safe()
                    .add_child(instance, false);
            }
        }
        let nodes = self
            .my_node
            .unwrap()
            .assume_safe()
            .get_node_as(id.to_string())
            .and_then(|f: TRef<Area2D>| f.cast::<Area2D>())
            .unwrap();
        nodes.set_position(Vector2 { x: 0.0, y: 0.0 });
        nodes.set("target", Vector2 { x: 0.0, y: 0.0 });
    }

    //处理服务器消息
    unsafe fn do_handle(&self, msg: &str) {
        let p: tools::msg_base::MsgBase = serde_json::from_str(msg).unwrap();
        match p {
            p if p.mes_type == 0 => {
                if p.message.contains("上线") {
                    //加载玩家
                    self.load_role(p.send_id);
                }
                let inp = self.msg_input.unwrap().assume_safe();
                let _ = inp.append_bbcode("\n[color=red]".to_string() + &p.message + "[/color]");
            }
            p if p.mes_type == 1 => {
                let inp = self.msg_input.unwrap().assume_safe();
                let _ = inp.append_bbcode("\n".to_string() + &p.message);
            }
            p if p.mes_type == 2 => {
                //移动其他玩家位置
                if p.message.contains("move") {
                    let nodes = self
                        .my_node
                        .unwrap()
                        .assume_safe()
                        .get_node_as(p.send_id.to_string())
                        .and_then(|f: TRef<Area2D>| f.cast::<Area2D>())
                        .unwrap();
                    nodes.set(
                        "target",
                        Vector2 {
                            x: p.position.x,
                            y: p.position.y,
                        },
                    );
                }
            }
            tools::msg_base::MsgBase { .. } => todo!(),
        }
    }
}
