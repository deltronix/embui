use crate::widgets::state::{StateManager, WidgetState};
use crate::{InputEvent, themes::Theme};
use embedded_graphics::prelude::*;

pub trait Widget<C: PixelColor>: Dimensions + Transform + Drawable {
    fn draw_with_theme<D, T>(&self, target: &mut D, theme: &T) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
        T: Theme<C>;
    //fn handle_event(&mut self, event: InputEvent) -> OutputEvent {}
    fn is_enabled() -> bool;
    fn set_enabled(&mut self, enabled: bool);

    fn handle_event(&mut self, event: InputEvent) -> bool {
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

        let widget_handled = self.handle_event_impl(event, contains_point);

        state_changed || widget_handled
    }

    fn get_state(&self) -> WidgetState {
        self.get_state_manager().current_state()
    }

    fn set_state(&mut self, state: WidgetState) -> bool {
        self.get_state_manager_mut().set_state(state)
    }

    fn get_state_manager(&self) -> &StateManager;
    fn get_state_manager_mut(&mut self) -> &mut StateManager;

    fn handle_event_impl(&mut self, _event: InputEvent, _contains_point: bool) -> bool {
        false
    }
}
