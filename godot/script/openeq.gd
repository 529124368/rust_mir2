extends TextureButton

var eq 
var audio
func _ready(): 
	eq = self.get_node("../../eq")
	audio = self.get_node("../audio")
func _pressed():
	eq.visible = !eq.visible
	audio.play()
	


