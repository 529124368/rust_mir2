# Info

The game demo written by godot rust implements simple websocket communication, most of the game logic is developed using rust, and the websocket client and server are developed based on tokio .



# rust_mir2
A Legend 2 (Mir2) demo written with rust+godot Gdnative


# Installation Process
### . Godot Engine version 3.4 is required
### . Install the rust development environment


# Demo Server Code (websocket serve code) written in rust, download link:

https://github.com/529124368/rust_websocket_demo

Compilation command: `cargo build --release`


# Client Code (game client code)

The code includes rust script code + godot project code

First download the code, and execute the compilation command below as well:

Compilation command: `cargo build --release`

Then open the Godot engine, use the Godot engine to open the `project.godot` file in the project to enter the game editor screen, and then click the run button within the engine to run the game.

![image](https://user-images.githubusercontent.com/22612129/184534556-d8801ed2-0d42-45b3-932a-39778ffde83b.png)


Server side

![image](https://user-images.githubusercontent.com/22612129/184534572-e15dd599-68e3-4d16-b634-2dd24be0c9bf.png)

Client side

![image](https://user-images.githubusercontent.com/22612129/184534580-f8a4ee37-f33e-42ee-87d8-6d3f463a6309.png)


Implemented character movement, multiplayer movement on the same screen, online chat, and nothing else.

Game Controls

Left-click to move, right-click to use skill + attack

Press F1 to switch skill -> Half Moon Scimitar
Press F2 to switch skill -> Flaming Sword Technique
