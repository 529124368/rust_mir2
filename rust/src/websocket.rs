use futures_util::{future, pin_mut, StreamExt};
use gdnative::api::*;
use gdnative::prelude::*;
use std::sync::{Arc, RwLock};
use std::thread;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// The Websocket "class"
#[derive(NativeClass, Clone)]
#[inherit(Node)]
#[register_with(Self::register_builder)]
pub struct Websocket {
    url: String,
    send_channel: Arc<RwLock<Option<UnboundedSender<String>>>>,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Websocket {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Websocket builder is registered!");
    }
    /// The "constructor" of the class.
    fn new(_owner: &Node) -> Self {
        Websocket {
            url: "ws:127.0.0.1/chat".to_string(),
            send_channel: Arc::new(RwLock::new(None)),
        }
    }

    #[godot]
    unsafe fn _ready(&mut self, #[base] _owner: &Node) {
        //输入信号注册
        self.bind_signal_method_by_path(
            _owner,
            "../../CanvasLayer/message/input",
            "text_entered",
            "_on_input_enter",
        );
        //todo
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

    // #[godot]

    // unsafe fn _process(&self, #[base] _owner: &Node, delta: f64) {}

    //链接
    async fn conn(&mut self, mut chanel2: UnboundedReceiver<String>) {
        godot_print!("进入了监听");
        let url = url::Url::parse(&self.url).unwrap();
        //消息传送管道
        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();

        tokio::task::spawn(async move {
            while let Some(s) = chanel2.recv().await {
                godot_print!("获取到消息了在巡乱队列里:{}", s);
                stdin_tx
                    .unbounded_send(Message::binary(s.into_bytes()))
                    .unwrap();
            }
        });

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        let (write, read) = ws_stream.split();

        //往服务器发送消息
        let stdin_to_ws = stdin_rx.map(Ok).forward(write);

        //读取服务器消息
        let ws_to_stdout = read.for_each(|message| async {
            let res = match message {
                Ok(s) => s,
                Err(_) => {
                    panic!("掉线了");
                }
            };
            let data = res.into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        });
        //

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
        godot_print!("结束了监听");
    }

    #[godot]
    unsafe fn _on_input_enter(&mut self, _data: Variant) {
        let mess = _data.to_string();
        godot_print!("输入消息为:{}", mess);
        self.send_mesg(mess);
    }

    fn send_mesg(&self, msg: String) {
        let _ = self
            .send_channel
            .write()
            .unwrap()
            .as_ref()
            .unwrap()
            .send(msg);
    }

    // //绑定其他节点信号
    unsafe fn bind_signal_method_by_path(
        &self,
        _owner: &Node,
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
}
