use core::marker::PhantomData;

use crate::{
    StateManager, Widget, WidgetState,
    themes::{DefaultTheme, Theme},
};
use embedded_graphics::{
    mono_font::{MonoTextStyle, iso_8859_2::FONT_6X10},
    pixelcolor::{BinaryColor, Rgb888},
    prelude::*,
    primitives::{
        CornerRadii, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle,
        StyledDrawable,
    },
};
use embedded_text::{
    TextBox,
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
};

#[derive(Clone, Debug)]
pub struct Button<'a, C>
where
    C: PixelColor + Default + From<Rgb888> + 'static,
{
    ph: PhantomData<C>,
    label: &'a str,
    pos: Point,
    size: Size,
    state_manager: StateManager,
    on_click: Option<fn()>,
}

impl<'a, C> Button<'a, C>
where
    C: PixelColor + Default + From<Rgb888> + 'a,
{
    pub fn new(label: &'a str, pos: Point, size: Size) -> Self {
        Self {
            label,
            pos,
            size,
            ph: PhantomData,
            state_manager: StateManager::default(),
            on_click: None,
        }
    }

    pub fn with_callback(mut self, callback: fn()) -> Self {
        self.on_click = Some(callback);
        self
    }
}

impl<'a, C> Drawable for Button<'a, C>
where
    C: PixelColor + From<Rgb888> + Default + 'a,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        // Use a global default theme or a theme stored in the widget
        let theme = DefaultTheme::<C>::new();
        self.draw_with_theme(target, &theme);
        Ok(())
    }
}
impl<'a, C, D> Widget<C, D> for Button<'a, C>
where
    C: PixelColor + Default + From<Rgb888> + 'a,
    D: DrawTarget<Color = C>,
{
    fn draw_with_theme(&self, target: &mut D, theme: &dyn Theme<C>) -> Result<(), ()> {
        let (background_color, text_color, border_color) = match Widget::<C, D>::get_state(self) {
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
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Center)
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

        let outline = Rectangle::new(self.pos, self.size);

        let _ = outline.draw_styled(&outline_style, target);
        let _ = label.draw(target);
        Ok(())
    }
    fn get_state_manager(&self) -> &StateManager {
        &self.state_manager
    }
    fn get_state_manager_mut(&mut self) -> &mut StateManager {
        &mut self.state_manager
    }
}

//impl<C> Widget for Button<'_, C> where C: PixelColor + From<BinaryColor> {}

impl<C> Transform for Button<'_, C>
where
    C: PixelColor + From<Rgb888> + Default + 'static,
{
    fn translate(&self, by: Point) -> Self {
        let mut new_button = self.clone();
        new_button.pos += by;
        new_button
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.pos += by;
        self
    }
}

impl<C> Dimensions for Button<'_, C>
where
    C: PixelColor + From<Rgb888> + Default + 'static,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.pos, self.size)
    }
}
