#![no_std]

extern crate alloc;

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

pub mod screen;
pub mod themes;
pub mod widgets;
use heapless::Vec;
pub use themes::Theme;
pub use widgets::{StateManager, WidgetState};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEvent {
    Touch(Point),
    TouchRelease(Point),
    KeyPress(char),
    MouseMove(Point),
    MouseDown(Point),
    MouseUp(Point),
}
#[derive(Debug, Clone, Copy)]
pub enum Response<M: Clone + Copy> {
    Changed(Option<M>),
    NotChanged,
}
/// A Widget handles events.
pub trait Widget<M: Clone + Copy>: Dimensions {
    fn to_message(&self) -> Option<M>;
    fn handle_event(&mut self, event: InputEvent) -> Response<M> {
        let contains_point = match event {
            InputEvent::Touch(point)
            | InputEvent::TouchRelease(point)
            | InputEvent::MouseMove(point)
            | InputEvent::MouseDown(point)
            | InputEvent::MouseUp(point) => self.bounding_box().contains(point),
            _ => false,
        };

        let state_changed = self
            .get_state_manager_mut()
            .handle_event(event, contains_point);

        if state_changed {
            Response::Changed(self.to_message())
        } else {
            Response::NotChanged
        }
    }

    fn get_state(&self) -> WidgetState {
        self.get_state_manager().current_state()
    }

    fn set_state(&mut self, state: WidgetState) -> bool {
        self.get_state_manager_mut().set_state(state)
    }

    fn get_state_manager(&self) -> &StateManager;
    fn get_state_manager_mut(&mut self) -> &mut StateManager;
}
pub trait ThemedWidget<D: DrawTarget, T: Theme<C>, C: PixelColor + Default + From<Rgb888>> {
    fn draw_with_theme(&self, target: &mut D, theme: &T) -> Result<(), D::Error>;
}


enum Parameter {

}

pub struct Element<'a, M, C> {
    theme: &'a dyn Theme<C>,
    widget: &'a mut dyn Widget<M>,
    parent: Option<&'a Element<'a, M, C>>,
    children: heapless::Vec<&'a Element<'a, M, C>, 8>,
}

impl<M, C> Drawable for Element<'_, M, C>
where
    C: PixelColor + Default,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Ok(())
    }
}

impl<'a, M, C> Element<'a, M, C> {
    pub fn new(theme: &'a dyn Theme<C>, widget: &'a mut dyn Widget<M>) -> Self {
        Self {
            theme,
            widget,
            parent: None,
            children: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Element, themes::DefaultTheme, widgets::Button};
    use embedded_graphics::pixelcolor::Rgb888;

    #[test]
    fn it_works() {
        #[derive(Clone, Copy)]
        enum Message {
            Test,
        }
        let theme = DefaultTheme::<Rgb888>::new();
        let mut widget = Button::<Message, Rgb888>::new("test");
        let n = Element::new(&theme, &mut widget);
    }
}
