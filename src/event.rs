use std::time::Duration;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Event {
	Unknown,
	Quit,
	Key(KeyEvent),
	Window(WindowEvent),
	Settings(SettingsEvent),
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum KeyEvent {
	Up,
	Down,
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum WindowEvent {
	Minimized,
	Maximized,
	Restore
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum SettingsEvent {
	FrameTime(Duration),
}