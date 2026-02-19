extends Control

func _input(event: InputEvent):
	if event.is_action_pressed("next_slide"):
		$PresentationInfo.next_slide()
	if event.is_action_pressed("last_slide"):
		$PresentationInfo.last_slide()
	if event.is_action_pressed("fullscreen"):
		var mode := DisplayServer.window_get_mode()
		var is_window = mode != DisplayServer.WINDOW_MODE_FULLSCREEN
		DisplayServer.window_set_mode(DisplayServer.WINDOW_MODE_FULLSCREEN if is_window else DisplayServer.WINDOW_MODE_WINDOWED)
	if event.is_action_pressed("exit"):
		get_tree().quit(0)
