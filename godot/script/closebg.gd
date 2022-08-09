extends TextureButton

var bg
var audio

func _ready(): 
	bg = self.get_node("../../bag")
	audio = self.get_node("../audio")
	
func _pressed():
	bg.visible = !bg.visible
	audio.play()
	

