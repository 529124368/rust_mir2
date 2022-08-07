extends TextureButton


# Called when the node enters the scene tree for the first time.
func _pressed():
	var minibar = self.get_node("../../minibar")
	self.get_node("../minipanel").visible=true
	minibar.visible=false
	self.visible = false
		#移动minibar位置
	pass

