extends Node2D

func _on_websocket_create_role():
	print("收到信号")
	var role = load("res://scenes/otherPlayer.tscn").instance()
	add_child(role)
