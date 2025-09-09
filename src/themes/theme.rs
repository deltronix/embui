use embedded_graphics::mono_font::MonoFont;
use embedded_graphics::prelude::*;

use crate::{Element, Widget, WidgetState, widgets::state::Stateful};

struct SimpleStyle<'a, C: PixelColor> {
    theme: &'a dyn Theme<C>,
    default_size: Size,
}

impl<'a, M, C, D> Style<M, C, D> for SimpleStyle<'a, C>
where
    M: Copy,
    C: PixelColor + Default,
    D: DrawTarget,
{
    fn draw_element(&self, element: &Element<M, C>, target: &mut D) {
        let (widget, theme, state) = (&element.widget, element.theme, element.get_state());
        match element.widget {
            Widget::Button { label } => {}
            Widget::Text { text } => todo!(),
            Widget::Dial => todo!(),
            Widget::Fader => todo!(),
            Widget::Toggle => todo!(),
        }
    }
    fn state_to_colors(&self, state: WidgetState) -> (C, C, C) {
        match state {
            WidgetState::Normal => (
                self.theme.button_normal_bg(),
                self.theme.button_normal_text(),
                self.theme.button_normal_border(),
            ),
            WidgetState::Hovered => (
                self.theme.button_hovered_bg(),
                self.theme.button_hovered_text(),
                self.theme.button_hovered_border(),
            ),
            WidgetState::Pressed => (
                self.theme.button_pressed_bg(),
                self.theme.button_pressed_text(),
                self.theme.button_pressed_border(),
            ),
            WidgetState::Focused => (
                self.theme.button_hovered_bg(),
                self.theme.button_hovered_text(),
                self.theme.button_pressed_bg(),
            ),
            WidgetState::Disabled => todo!(),
        }
    }
}

pub trait Style<M, C, D>
where
    M: Copy,
    C: PixelColor + Default,
    D: DrawTarget,
{
    fn draw_element(&self, element: &Element<M, C>, target: &mut D);
    fn state_to_colors(&self, state: WidgetState) -> (C, C, C);
}

pub trait Theme<C: PixelColor>: core::fmt::Debug {
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
    fn small_font(&self) -> &'static MonoFont<'static>;
    fn normal_font(&self) -> &'static MonoFont<'static>;
    fn large_font(&self) -> &'static MonoFont<'static>;
    fn title_font(&self) -> &'static MonoFont<'static>;

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
