use embedded_graphics::mono_font::{MonoFont, MonoTextStyle};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;

pub trait Theme<C: PixelColor> {
    // Color palette access
    fn primary_color(&self) -> C;
    fn primary_dark(&self) -> C;
    fn primary_light(&self) -> C;
    fn secondary_color(&self) -> C;
    fn background_color(&self) -> C;
    fn surface_color(&self) -> C;
    fn text_primary(&self) -> C;
    fn text_secondary(&self) -> C;
    fn text_disabled(&self) -> C;
    fn border_color(&self) -> C;
    fn error_color(&self) -> C;
    fn success_color(&self) -> C;
    fn warning_color(&self) -> C;

    // Typography
    fn small_font(&self) -> &'static MonoFont;
    fn normal_font(&self) -> &'static MonoFont;
    fn large_font(&self) -> &'static MonoFont;
    fn title_font(&self) -> &'static MonoFont;

    // Spacing
    fn spacing_xs(&self) -> u32 {
        2
    }
    fn spacing_sm(&self) -> u32 {
        4
    }
    fn spacing_md(&self) -> u32 {
        8
    }
    fn spacing_lg(&self) -> u32 {
        16
    }
    fn spacing_xl(&self) -> u32 {
        24
    }
    fn spacing_xxl(&self) -> u32 {
        32
    }

    // Button-specific styling
    fn button_normal_bg(&self) -> C {
        self.surface_color()
    }
    fn button_normal_text(&self) -> C {
        self.text_primary()
    }
    fn button_normal_border(&self) -> C {
        self.border_color()
    }

    fn button_hovered_bg(&self) -> C {
        self.primary_light()
    }
    fn button_hovered_text(&self) -> C {
        self.text_primary()
    }
    fn button_hovered_border(&self) -> C {
        self.primary_color()
    }

    fn button_pressed_bg(&self) -> C {
        self.primary_color()
    }
    fn button_pressed_text(&self) -> C {
        self.surface_color()
    }
    fn button_pressed_border(&self) -> C {
        self.primary_dark()
    }

    fn button_disabled_bg(&self) -> C {
        self.surface_color()
    }
    fn button_disabled_text(&self) -> C {
        self.text_disabled()
    }
    fn button_disabled_border(&self) -> C {
        self.border_color()
    }

    fn button_border_width(&self) -> u32 {
        1
    }
    fn button_pressed_offset(&self) -> Point {
        Point::new(1, 1)
    }
    fn button_corner_radius(&self) -> u32 {
        0
    }

    // Label-specific styling
    fn label_text_color(&self) -> C {
        self.text_primary()
    }
    fn label_disabled_text_color(&self) -> C {
        self.text_disabled()
    }
    fn label_background_color(&self) -> Option<C> {
        None
    }

    // Panel-specific styling
    fn panel_background_color(&self) -> C {
        self.surface_color()
    }
    fn panel_border_color(&self) -> C {
        self.border_color()
    }
    fn panel_border_width(&self) -> u32 {
        1
    }
    fn panel_padding(&self) -> u32 {
        self.spacing_md()
    }

    // TextBox-specific styling
    fn textbox_background_color(&self) -> C {
        self.surface_color()
    }
    fn textbox_text_color(&self) -> C {
        self.text_primary()
    }
    fn textbox_border_color(&self) -> C {
        self.border_color()
    }
    fn textbox_focused_border_color(&self) -> C {
        self.primary_color()
    }
    fn textbox_cursor_color(&self) -> C {
        self.primary_color()
    }
    fn textbox_selection_color(&self) -> C {
        self.primary_light()
    }
    fn textbox_border_width(&self) -> u32 {
        1
    }
    fn textbox_padding(&self) -> u32 {
        self.spacing_sm()
    }
}
