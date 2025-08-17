use embedded_graphics::prelude::Point;
use embedded_graphics_simulator::{SimulatorEvent, sdl2::Keycode};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEvent {
    Touch(Point),
    TouchRelease(Point),
    KeyPress(char),
    MouseMove(Point),
    MouseDown(Point),
    MouseUp(Point),
}
