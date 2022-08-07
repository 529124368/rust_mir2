extends AnimatedSprite

func _ready():
	self.play("default")
	
func _process(delta):
	if self.frame == 9:
		get_tree().change_scene("res://scenes/game/start.tscn")
