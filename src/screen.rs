use core::slice::{Iter, IterMut};

use crate::themes::DefaultTheme;
use crate::widgets::Button;
use crate::{Box, InputEvent, StateManager, Theme, ThemedWidget, Widget, WidgetState};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

pub trait Element<M, D, T, C>: Widget<M> + ThemedWidget<D, T, C>
where
    M: Copy + Clone,
    D: DrawTarget<Color = C>,
    T: Theme<C>,
    C: PixelColor + Default + From<Rgb888>,
{
}
struct Screen<'a, M, C, D, T>
where
    M: Copy + Clone,
    D: DrawTarget<Color = C>,
    T: Theme<C>,
    C: PixelColor,
{
    widgets: &'a [&'a dyn Element<M, D, T, C>; 32],
    display: D,
}

pub trait Draw<D, T, C>
where
    C: PixelColor + Default + From<Rgb888>,
    T: Theme<C>,
    D: DrawTarget<Color = C>,
{
    fn draw_all(&self, target: &mut D, theme: &T) -> Result<(), D::Error>;
}

impl<M, D, T, C> Draw<D, T, C> for Iter<'_, &dyn Element<M, D, T, C>>
where
    M: Copy + Clone,
    C: PixelColor + Default + From<Rgb888>,
    D: DrawTarget<Color = C>,
    T: Theme<C>,
{
    fn draw_all(&self, target: &mut D, theme: &T) -> Result<(), D::Error> {
        for e in self.as_slice() {
            e.draw_with_theme(target, theme)?
        }
        Ok(())
    }
}

#[test]
fn test_element() {
    //let elements = [Button::new("test", Point::zero(), Size::new(64, 64))];
}
