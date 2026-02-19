extends Control

func _input(event: InputEvent):
	if event.is_action_pressed("next_slide"):
		$PresentationInfo.next_slide()
	if event.is_action_pressed("last_slide"):
		$PresentationInfo.last_slide()
