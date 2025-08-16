use embedded_graphics::prelude::*;
#[derive(Clone, Copy, Debug)]
pub struct ColorPalette<C: PixelColor> {
    // Primary colors
    pub primary: C,
    pub primary_dark: C,
    pub primary_light: C,

    // Secondary colors
    pub secondary: C,
    pub secondary_dark: C,
    pub secondary_light: C,

    // Neutral colors
    pub background: C,
    pub surface: C,
    pub border: C,

    // Text colors
    pub text_primary: C,
    pub text_secondary: C,
    pub text_disabled: C,

    // Status colors
    pub success: C,
    pub warning: C,
    pub error: C,
    pub info: C,
}
