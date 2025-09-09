pub use button::Button;
use embedded_layout::View;
use embedded_layout::view_group::ViewGroup;
pub use number::Number;
pub use state::StateManager;
pub use state::WidgetState;

use crate::Stateful;
pub mod button;
mod number;
pub mod state;
#[derive(Debug)]
pub enum Widget<'a> {
    Button { label: &'a str },
    Text { text: &'a str },
    Dial,
    Fader,
    Toggle,
}

impl<'a> Widget<'a> {
    pub fn button(label: &'a str) -> Self {
        Self::Button { label }
    }
    pub fn text(text: &'a str) -> Self {
        Self::Text { text }
    }
}
