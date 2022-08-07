extends Control

signal move_mini_panel_to_left
signal move_mini_panel_to_right
#全局变量
var offset
var rightSide
var leftSide

func _ready():
	offset = get_viewport_rect().size.x /6.5
	rightSide = self.get_node("../rightSide")
	leftSide = self.get_node("../leftSide")

func _on_TextureButton2_pressed():
	#显示装备栏
	if rightSide.visible:
		rightSide.visible=false
		#移动minibar位置
		self.rect_position.x += offset 
		emit_signal("move_mini_panel_to_right")
	else:
		rightSide.visible=true
		#移动minibar位置
		self.rect_position.x -= offset
		emit_signal("move_mini_panel_to_left")
		
		
func _on_TextureButton_pressed():
		#显示装备栏
	if leftSide.visible:
		leftSide.visible=false
		#移动minibar位置
		self.rect_position.x -= offset
		emit_signal("move_mini_panel_to_left")
	else:
		leftSide.visible=true
		#移动minibar位置
		self.rect_position.x += offset
		emit_signal("move_mini_panel_to_right") 
		


func _on_closeleft_pressed():
	emit_signal("move_mini_panel_to_left")
		#显示装备栏
	#移动minibar位置
	self.rect_position.x -= offset 
	leftSide.visible = false


func _on_closeright_pressed():
	emit_signal("move_mini_panel_to_right")
	#移动minibar位置
	self.rect_position.x += offset 
	rightSide.visible = false
	pass


func _on_TextureButton3_pressed():
	var playerNode = self.get_node("../../TileMap/Player")
	if playerNode.sprite_name == "ba":
		playerNode.sprite_name = "ba2"
	else:
		playerNode.sprite_name = "ba"
	playerNode._reload_asset()
	
	pass # Replace with function body.
