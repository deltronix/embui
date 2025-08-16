#![no_std]
use embedded_graphics::prelude::*;

#[derive(Debug, Clone)]
pub enum InputEvent {
    Touch(Point),
    KeyPress(),
}

pub mod events;
pub mod themes;
pub mod widgets;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
