#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::Rectangle};

pub mod screen;
pub mod themes;
pub mod widgets;
use heapless::{FnvIndexMap, Vec};
pub use themes::Theme;
pub use widgets::{StateManager, WidgetState};

use crate::{themes::DefaultTheme, widgets::state};

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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
