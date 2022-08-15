# Info

The game demo written by godot rust implements simple websocket communication, most of the game logic is developed using rust, and the websocket client and server are developed based on tokio .




# rust_mir2
rust+godot Gdnative 写的传奇2 demo


# 安装流程
### .需要安装 Godot引擎 3.4 版本
### .安装 rust 开发环境


# demo的服务器端代码(websocket serve code) rust写，下载地址

https://github.com/529124368/rust_websocket_demo

编译命令 ： cargo build --release


# 客户端代码 (game client code)

代码包含rust脚本代码 + godot工程代码

先下载代码，同样执行下面的编译命令

编译命令 ： cargo build --release

然后打开godot引擎，用godot引擎打开工程里的
project.godot文件，即进入游戏编辑器画面，然后点击引擎内的运行按钮，就可以运行游戏。

![image](https://user-images.githubusercontent.com/22612129/184534556-d8801ed2-0d42-45b3-932a-39778ffde83b.png)


服务器端

![image](https://user-images.githubusercontent.com/22612129/184534572-e15dd599-68e3-4d16-b634-2dd24be0c9bf.png)

客户端

![image](https://user-images.githubusercontent.com/22612129/184534580-f8a4ee37-f33e-42ee-87d8-6d3f463a6309.png)


实现了人物移动 ，同屏多人移动，在线聊天，其他没了。

游戏操作

左键移动，右键放技能+攻击

F1按一下切换技能 -》 半月弯刀
F2按一下切换技能 -》 烈火剑法

