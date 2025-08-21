use core::marker::PhantomData;

use crate::{
    StateManager, ThemedWidget, Widget, WidgetState,
    screen::Element,
    themes::{DefaultTheme, Theme},
};
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

#[derive(Clone, Debug)]
pub struct Button<'a, M, C>
where
    M: Copy + Clone,
    C: PixelColor,
{
    ph: PhantomData<C>,
    label: &'a str,
    pos: Option<Point>,
    size: Option<Size>,
    on_press: Option<M>,
    state_manager: StateManager,
}

impl<'a, M, C> Button<'a, M, C>
where
    M: Copy + Clone,
    C: PixelColor,
{
    pub fn new(label: &'a str) -> Self {
        Self {
            ph: PhantomData,
            label,
            pos: None,
            size: None,
            on_press: None,
            state_manager: StateManager::default(),
        }
    }

    pub fn on_press(mut self, msg: M) -> Self {
        self.on_press = Some(msg);
        self
    }
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }
    pub fn with_position(mut self, pos: Point) -> Self {
        self.pos = Some(pos);
        self
    }
}

impl<M, C> Widget<M> for Button<'_, M, C>
where
    M: Copy + Clone,
    C: PixelColor,
{
    fn to_message(&self) -> Option<M> {
        match self.get_state() {
            WidgetState::Pressed => self.on_press,
            _ => None,
        }
    }
    fn get_state_manager(&self) -> &StateManager {
        &self.state_manager
    }
    fn get_state_manager_mut(&mut self) -> &mut StateManager {
        &mut self.state_manager
    }
}
impl<D, T, C, M> ThemedWidget<D, T, C> for Button<'_, M, C>
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
        let label = TextBox::with_textbox_style(
            self.label,
            self.bounding_box(),
            character_style,
            textbox_style,
        );

        let outline_style = PrimitiveStyleBuilder::new()
            .fill_color(background_color)
            .stroke_color(border_color)
            .stroke_width(theme.spacing_xs())
            .build();

        let outline = Rectangle::new(
            self.pos.unwrap_or(Point::zero()) + Point::new(2, 2),
            self.size.unwrap_or(Size::new(16, 16)) - Size::new(4, 4),
        );

        outline.draw_styled(&outline_style, target)?;
        label.draw(target)?;
        Ok(())
    }
}
impl<M, C> Drawable for Button<'_, M, C>
where
    M: Copy + Clone,
    C: PixelColor + Default + From<Rgb888>,
{
    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.draw_with_theme(target, &DefaultTheme::new())
    }

    type Color = C;

    type Output = ();
}
impl<M: Copy + Clone, D, T, C> Element<M, D, T, C> for Button<'_, M, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + Default + From<Rgb888>,
    T: Theme<C>,
{
}

impl<M, C> Transform for Button<'_, M, C>
where
    M: Copy + Clone,
    C: PixelColor,
{
    fn translate(&self, by: Point) -> Self {
        let mut new_button = self.clone();
        if let Some(pos) = new_button.pos {
            new_button.pos = Some(pos + by)
        } else {
            new_button.pos = Some(by)
        }
        new_button
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        if let Some(pos) = self.pos {
            self.pos = Some(pos + by)
        } else {
            self.pos = Some(by)
        }

        self
    }
}

impl<M, C> Dimensions for Button<'_, M, C>
where
    M: Copy + Clone,
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        let pos = self.pos.unwrap_or(Point::zero());
        let size = self.size.unwrap_or(Size::new(16, 16));
        Rectangle::new(pos, size)
    }
}
