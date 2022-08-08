extends TextureButton

var eq 

func _ready(): 
	eq = self.get_node("../../eq")
func _pressed():
	eq.visible = !eq.visible
	


