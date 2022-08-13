extends LineEdit
var msg:RichTextLabel

func _ready():
	msg = self.get_node("../msg")
	
func _on_input_text_entered(new_text):
	msg.append_bbcode("\n[color=green]"+new_text+"[/color]")
	self.text = ""
