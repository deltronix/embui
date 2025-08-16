use core::marker::PhantomData;

use crate::{
    themes::{DefaultTheme, Theme},
    widgets::Widget,
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

#[derive(Clone, Copy, Debug)]
pub struct Button<'a, C: PixelColor> {
    ph: PhantomData<C>,
    label: &'a str,
    pos: Point,
    size: Size,
}

impl<'a, C> Button<'a, C>
where
    C: PixelColor,
{
    pub fn new(label: &'a str, pos: Point, size: Size) -> Self {
        Self {
            label,
            pos,
            size,
            ph: PhantomData,
        }
    }
}

impl<'a, C> Drawable for Button<'a, C>
where
    C: PixelColor + From<Rgb888> + From<BinaryColor> + Default + 'static,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        // Use a global default theme or a theme stored in the widget
        let theme = DefaultTheme::<C>::new();
        self.draw_with_theme(target, &theme)
    }
}
impl<'a, C> Widget<C> for Button<'a, C>
where
    C: PixelColor + Default + From<BinaryColor> + From<Rgb888> + 'static,
{
    fn draw_with_theme<D, T>(&self, target: &mut D, theme: &T) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
        T: Theme<C>,
    {
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Center)
            .paragraph_spacing(6)
            .build();
        let character_style = MonoTextStyle::new(theme.normal_font(), theme.button_normal_text());
        let label = TextBox::with_textbox_style(
            self.label,
            self.bounding_box(),
            character_style,
            textbox_style,
        );

        let outline_style = PrimitiveStyleBuilder::new()
            .fill_color(theme.button_normal_bg())
            .stroke_color(theme.button_normal_border())
            .stroke_width(theme.spacing_xs())
            .build();

        let outline = RoundedRectangle::new(
            Rectangle::new(self.pos, self.size),
            CornerRadii::new(Size::new(3, 3)),
        );

        outline.draw_styled(&outline_style, target)?;
        label.draw(target)?;
        Ok(())
    }

    fn is_enabled() -> bool {
        todo!()
    }

    fn set_enabled(&mut self, enabled: bool) {
        todo!()
    }
}
//impl<C> Widget for Button<'_, C> where C: PixelColor + From<BinaryColor> {}

impl<C> Transform for Button<'_, C>
where
    C: PixelColor,
{
    fn translate(&self, by: Point) -> Self {
        let mut new_button = *self;
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
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.pos, self.size)
    }
}
