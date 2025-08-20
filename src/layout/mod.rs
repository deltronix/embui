use core::marker::PhantomData;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_layout::View;
use embedded_layout::layout::linear::LinearLayout;
use embedded_layout::prelude::Chain;
use embedded_layout::view_group::ViewGroup;

use crate::screen::{Draw, Element};
use crate::widgets::Button;
use crate::{StateManager, Theme, ThemedWidget, Widget};

pub struct Container<C>
where
    C: PixelColor,
{
    ph: PhantomData<C>,
    pos: Point,
    size: Size,
    state_manager: StateManager,
}

impl<C> Container<C>
where
    C: PixelColor,
{
    pub fn new(pos: Point, size: Size) -> Self {
        Self {
            ph: PhantomData,
            pos,
            size,
            state_manager: StateManager::new(),
        }
    }
}

impl<C> Dimensions for Container<C>
where
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.pos, self.size)
    }
}
impl<D, T, C> ThemedWidget<D, T, C> for Container<C>
where
    C: PixelColor + Default + From<Rgb888>,
    D: DrawTarget<Color = C>,
    T: Theme<C>,
{
    fn draw_with_theme(&self, target: &mut D, theme: &T) -> Result<(), D::Error> {
        Ok(())
    }
}
impl<C, M> Widget<M> for Container<C>
where
    C: PixelColor,
    M: Copy + Clone,
{
    fn to_message(&self) -> Option<M> {
        None
    }

    fn get_state_manager(&self) -> &crate::StateManager {
        &self.state_manager
    }

    fn get_state_manager_mut(&mut self) -> &mut crate::StateManager {
        &mut self.state_manager
    }
}
impl<M, D, T, C> Element<M, D, T, C> for Container<C>
where
    M: Copy + Clone,
    D: DrawTarget<Color = C>,
    C: PixelColor + Default + From<Rgb888>,
    T: Theme<C>,
{
}
pub struct Window<C, E>
where
    C: PixelColor + Default + From<Rgb888>,
{
    ph: PhantomData<C>,
    element_chain: Chain<E>,
}

impl<C, E> Window<C, E>
where
    C: PixelColor + Default + From<Rgb888>,
    E: View,
{
    fn from(chain: Chain<E>) -> Self {
        Self {
            ph: PhantomData,
            element_chain: chain,
        }
    }
}

#[test]
fn layout() {
    let b: Button<Rgb888> = Button::new("test");
    let b2: Button<Rgb888> = Button::new("test2");

    Chain::new(b).append(b2);
}
