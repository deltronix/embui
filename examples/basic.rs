use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_9X18},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_layout::{layout::linear::LinearLayout, prelude::*};
use gui::themes::Theme;
use gui::widgets::Button;
use gui::widgets::Widget;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();

    let display_area = display.bounding_box();

    let button = Button::new("test", Point::zero(), Size::new(100, 100));

    let rect = Rectangle::new(Point::zero(), Size::new(100, 100)).into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(2)
            .build(),
    );
    LinearLayout::horizontal(Chain::new(rect).append(button))
        .arrange()
        .align_to(&display_area, horizontal::Center, vertical::Center)
        .draw(&mut display)
        .unwrap();

    Window::new("LinearLayout exmaple", &output_settings).show_static(&display);
    Ok(())
}
