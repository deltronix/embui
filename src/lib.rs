#![no_std]
use embedded_graphics::prelude::*;

pub mod events;
pub mod themes;
pub mod widgets;
pub use events::input::InputEvent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
