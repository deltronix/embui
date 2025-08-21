use core::{fmt::Write, marker::PhantomData};

use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, StyledDrawable},
};
use embedded_text::{
    TextBox,
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::{HeightMode, TextBoxStyleBuilder, VerticalOverdraw},
};

use crate::{StateManager, Theme, ThemedWidget, Widget, WidgetState, screen::Element};
#[derive(Clone)]
pub struct Number<M>
where
    M: Copy + Clone,
{
    ph: PhantomData<M>,
    number: i32,
    state_manager: StateManager,
    pos: Point,
    size: Size,
}

impl<M: Copy> Number<M> {
    pub fn new(pos: Point, size: Size) -> Self {
        Self {
            ph: PhantomData,
            number: 0,
            state_manager: StateManager::new(),
            pos,
            size,
        }
    }
    pub fn set(&mut self, number: i32) {
        self.number = number;
    }
    pub fn get(&self) -> i32 {
        self.number
    }
}
impl<M: Copy + Clone> Transform for Number<M> {
    fn translate(&self, by: Point) -> Self {
        let mut new_number = self.clone();
        new_number.pos += by;
        new_number
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.pos += by;
        self
    }
}
impl<M: Copy> Widget<M> for Number<M> {
    fn to_message(&self) -> Option<M> {
        None
    }

    fn get_state_manager(&self) -> &super::StateManager {
        &self.state_manager
    }

    fn get_state_manager_mut(&mut self) -> &mut super::StateManager {
        &mut self.state_manager
    }
}

impl<D, T, C, M> ThemedWidget<D, T, C> for Number<M>
where
    C: PixelColor + Default + From<Rgb888>,
    D: DrawTarget<Color = C>,
    T: Theme<C>,
    M: Copy,
{
    fn draw_with_theme(&self, target: &mut D, theme: &T) -> Result<(), <D as DrawTarget>::Error> {
        let (background_color, text_color, border_color) = match Widget::get_state(self) {
            WidgetState::Normal => (
                theme.button_normal_bg(),
                theme.button_normal_text(),
                theme.button_normal_border(),
            ),
            WidgetState::Hovered => (
                theme.button_hovered_bg(),
                theme.button_hovered_text(),
                theme.button_hovered_border(),
            ),
            WidgetState::Pressed => (
                theme.button_pressed_bg(),
                theme.button_pressed_text(),
                theme.button_pressed_border(),
            ),
            WidgetState::Focused => (
                theme.button_hovered_bg(),
                theme.button_hovered_text(),
                theme.primary_color(), // Different border for focus
            ),
            WidgetState::Disabled => (
                theme.button_disabled_bg(),
                theme.button_disabled_text(),
                theme.button_disabled_border(),
            ),
        };
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::Exact(VerticalOverdraw::Hidden))
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .paragraph_spacing(6)
            .build();
        let character_style = MonoTextStyle::new(theme.normal_font(), text_color);

        let mut text: heapless::String<8> = heapless::String::new();

        write!(&mut text, "{}", self.number).unwrap();
        let label = TextBox::with_textbox_style(
            text.as_str(),
            self.bounding_box(),
            character_style,
            textbox_style,
        );

        let outline_style = PrimitiveStyleBuilder::new()
            .fill_color(background_color)
            .stroke_color(border_color)
            .stroke_width(theme.spacing_xs())
            .build();

        let outline = Rectangle::new(self.pos + Point::new(2, 2), self.size - Size::new(4, 4));

        outline.draw_styled(&outline_style, target)?;
        label.draw(target)?;
        Ok(())
    }
}

impl<M: Copy + Clone, D, T, C> Element<M, D, T, C> for Number<M>
where
    M: Copy + Clone,
    D: DrawTarget<Color = C>,
    C: PixelColor + Default + From<Rgb888>,
    T: Theme<C>,
{
}
impl<M> Dimensions for Number<M>
where
    M: Copy + Clone,
{
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        Rectangle::new(self.pos, self.size)
    }
}
