use embedded_graphics::mono_font::MonoFont;
use embedded_graphics::mono_font::ascii::{FONT_4X6, FONT_6X10, FONT_8X13, FONT_10X20};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

pub mod colors;
pub mod theme;

pub use theme::Theme;

pub struct DefaultTheme<C: PixelColor> {
    _phantom: core::marker::PhantomData<C>,
}

impl<C: PixelColor> DefaultTheme<C> {
    pub const fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<C: PixelColor> Default for DefaultTheme<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> Theme<C> for DefaultTheme<C>
where
    C: PixelColor + From<Rgb888>,
{
    fn primary_color(&self) -> C {
        C::from(Rgb888::CSS_BLUE)
    }
    fn primary_dark(&self) -> C {
        C::from(Rgb888::CSS_DARK_BLUE)
    }
    fn primary_light(&self) -> C {
        C::from(Rgb888::CSS_LIGHT_BLUE)
    }
    fn secondary_color(&self) -> C {
        C::from(Rgb888::CSS_GRAY)
    }
    fn background_color(&self) -> C {
        C::from(Rgb888::CSS_WHITE)
    }
    fn surface_color(&self) -> C {
        C::from(Rgb888::CSS_LIGHT_GRAY)
    }
    fn text_primary(&self) -> C {
        C::from(Rgb888::CSS_BLACK)
    }
    fn text_secondary(&self) -> C {
        C::from(Rgb888::CSS_DIM_GRAY)
    }
    fn text_disabled(&self) -> C {
        C::from(Rgb888::CSS_LIGHT_GRAY)
    }
    fn border_color(&self) -> C {
        C::from(Rgb888::CSS_GRAY)
    }
    fn error_color(&self) -> C {
        C::from(Rgb888::CSS_RED)
    }
    fn success_color(&self) -> C {
        C::from(Rgb888::CSS_GREEN)
    }
    fn warning_color(&self) -> C {
        C::from(Rgb888::CSS_ORANGE)
    }

    fn small_font(&self) -> &'static MonoFont<'static> {
        &FONT_4X6
    }
    fn normal_font(&self) -> &'static MonoFont<'static> {
        &FONT_6X10
    }
    fn large_font(&self) -> &'static MonoFont<'static> {
        &FONT_8X13
    }
    fn title_font(&self) -> &'static MonoFont<'static> {
        &FONT_10X20
    }
}
