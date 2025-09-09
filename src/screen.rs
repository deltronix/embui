pub(crate) use core::slice::Iter;

use crate::{Stateful, Theme, ThemedWidget};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

pub trait Element<M, D, T, C>: Stateful<M> + ThemedWidget<D, T, C>
where
    M: Copy + Clone,
    D: DrawTarget<Color = C>,
    T: Theme<C>,
    C: PixelColor + Default + From<Rgb888>,
{
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
