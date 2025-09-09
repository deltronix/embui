use crate::{InputEvent, Response};

use embedded_graphics::prelude::Dimensions;

/// A Widget handles events.
pub trait Stateful<M: Clone + Copy>: Dimensions {
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

        Response::NotChanged
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum WidgetState {
    #[default]
    Normal,
    Hovered,
    Pressed,
    Focused,
    Disabled,
}

impl WidgetState {
    /// Returns true if the state allows user interaction
    pub fn is_interactive(self) -> bool {
        !matches!(self, WidgetState::Disabled)
    }

    /// Returns true if the state indicates the widget is being pressed
    pub fn is_pressed(self) -> bool {
        matches!(self, WidgetState::Pressed)
    }

    /// Returns true if the state indicates the widget has focus
    pub fn is_focused(self) -> bool {
        matches!(self, WidgetState::Focused)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StateManager {
    current_state: WidgetState,
    previous_state: WidgetState,
    enabled: bool,
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            current_state: WidgetState::default(),
            previous_state: WidgetState::default(),
            enabled: true,
        }
    }

    /// Get current state
    pub fn current_state(&self) -> WidgetState {
        self.current_state
    }

    /// Get previous state
    pub fn previous_state(&self) -> WidgetState {
        self.previous_state
    }

    /// Check if widget is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    /// Set enabled state
    pub fn set_enabled(&mut self, enabled: bool) -> bool {
        if self.enabled == enabled {
            return false;
        }

        self.enabled = enabled;
        if enabled {
            // Re-enable: go back to normal unless we were focused
            let target = if self.previous_state == WidgetState::Focused {
                WidgetState::Focused
            } else {
                WidgetState::Normal
            };
            self.set_state(target)
        } else {
            // Disable: remember current state and go to disabled
            self.set_state(WidgetState::Disabled)
        }
    }
    pub fn set_state(&mut self, new_state: WidgetState) -> bool {
        // Don't allow changes from disabled unless enabling
        if self.current_state == WidgetState::Disabled
            && new_state != WidgetState::Disabled
            && !self.enabled
        {
            return false;
        }

        if self.current_state == new_state {
            return false;
        }

        self.previous_state = self.current_state;
        self.current_state = new_state;
        true
    }
    // Handle input event and potentially transition state
    pub fn handle_event(&mut self, event: InputEvent, contains_point: bool) -> bool {
        if !self.enabled {
            return false;
        }

        let new_state = match (self.current_state, event) {
            // Mouse/touch interactions
            (WidgetState::Normal, InputEvent::MouseMove(_)) if contains_point => {
                Some(WidgetState::Hovered)
            }
            (WidgetState::Hovered, InputEvent::MouseMove(_)) if !contains_point => {
                Some(WidgetState::Normal)
            }
            (WidgetState::Hovered, InputEvent::MouseDown(_)) if contains_point => {
                Some(WidgetState::Pressed)
            }
            (WidgetState::Normal, InputEvent::MouseDown(_)) if contains_point => {
                Some(WidgetState::Pressed)
            }
            (WidgetState::Pressed, InputEvent::MouseUp(_)) => {
                if contains_point {
                    Some(WidgetState::Hovered)
                } else {
                    Some(WidgetState::Normal)
                }
            }
            (WidgetState::Normal, InputEvent::Touch(_)) if contains_point => {
                Some(WidgetState::Pressed)
            }
            (WidgetState::Pressed, InputEvent::TouchRelease(_)) => {
                Some(WidgetState::Normal) // Touch usually doesn't hover
            }

            // Focused state handling - simple approach
            (WidgetState::Focused, InputEvent::MouseMove(_)) if contains_point => {
                Some(WidgetState::Hovered)
            }
            (WidgetState::Focused, InputEvent::MouseMove(_)) if !contains_point => {
                Some(WidgetState::Normal)
            }

            _ => None,
        };

        if let Some(state) = new_state {
            self.set_state(state)
        } else {
            false
        }
    }
    /// Reset to normal state
    pub fn reset(&mut self) {
        self.current_state = if self.enabled {
            WidgetState::Normal
        } else {
            WidgetState::Disabled
        };
        self.previous_state = self.current_state;
    }

    /// Check if state has changed since last check
    pub fn state_changed(&self) -> bool {
        self.current_state != self.previous_state
    }
}

#[cfg(test)]
mod tests {
    use embedded_graphics::prelude::Point;

    use super::*;

    #[test]
    fn state_manager_basic() {
        let mut sm = StateManager::new();
        assert_eq!(sm.current_state(), WidgetState::Normal);
        assert!(sm.is_enabled());

        // Test state transition
        assert!(sm.set_state(WidgetState::Hovered));
        assert_eq!(sm.current_state(), WidgetState::Hovered);
        assert_eq!(sm.previous_state(), WidgetState::Normal);

        // Test disable
        assert!(sm.set_enabled(false));
        assert_eq!(sm.current_state(), WidgetState::Disabled);
        assert!(!sm.is_enabled());
    }

    #[test]
    fn state_manager_events() {
        let mut sm = StateManager::new();

        // Mouse enter should hover
        assert!(sm.handle_event(InputEvent::MouseMove(Point::zero()), true));
        assert_eq!(sm.current_state(), WidgetState::Hovered);

        // Mouse down should press
        assert!(sm.handle_event(InputEvent::MouseDown(Point::zero()), true));
        assert_eq!(sm.current_state(), WidgetState::Pressed);

        // Mouse up should hover (if still over widget)
        assert!(sm.handle_event(InputEvent::MouseUp(Point::zero()), true));
        assert_eq!(sm.current_state(), WidgetState::Hovered);
    }
}
