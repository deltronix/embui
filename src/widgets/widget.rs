#![no_std]

use crate::themes::Theme;
use embedded_graphics::prelude::*;

pub trait Widget<C: PixelColor>: Dimensions + Transform + Drawable {
    fn draw_with_theme<D, T>(&self, target: &mut D, theme: &T) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
        T: Theme<C>;
    fn handle_event() {}
    fn is_enabled() -> bool;
    fn set_enabled(&mut self, enabled: bool);
}
