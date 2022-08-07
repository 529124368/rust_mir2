extends TextureButton


func _pressed():
	#按钮切换
	var minibar = self.get_node("../../minibar")
	self.get_node("../minipanel2").visible=true
	minibar.visible=true
	pass
