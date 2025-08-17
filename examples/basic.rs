use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_9X18},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, Window,
    sdl2::{Keycode, MouseButton},
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*};
use embui::Widget;
use embui::widgets::Button;
use embui::{InputEvent, themes::Theme};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("buttons.rs", &output_settings);

    let display_area = display.bounding_box();

    let mut button: Button<Rgb888> = Button::new("test", Point::zero(), Size::new(100, 100));

    let rect = Rectangle::new(Point::zero(), Size::new(100, 100)).into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(2)
            .build(),
    );

    button.draw(&mut display)?;
    window.update(&display);
    'running: loop {
        for event in window.events() {
            match event {
                embedded_graphics_simulator::SimulatorEvent::KeyUp { .. } => {}
                embedded_graphics_simulator::SimulatorEvent::KeyDown { keycode, .. } => {
                    if keycode == Keycode::Q {
                        break 'running;
                    }
                }
                embedded_graphics_simulator::SimulatorEvent::MouseButtonUp { mouse_btn, point } => {
                    if mouse_btn == MouseButton::Left {
                        button.handle_event(embui::InputEvent::MouseUp(point));
                    }
                }
                embedded_graphics_simulator::SimulatorEvent::MouseButtonDown {
                    mouse_btn,
                    point,
                } => {
                    if mouse_btn == MouseButton::Left {
                        button.handle_event(InputEvent::MouseDown(point));
                    }
                }
                embedded_graphics_simulator::SimulatorEvent::MouseWheel { .. } => {}
                embedded_graphics_simulator::SimulatorEvent::MouseMove { point } => {
                    button.handle_event(embui::InputEvent::MouseMove(point));
                }
                embedded_graphics_simulator::SimulatorEvent::Quit => {
                    break 'running;
                }
            }
        }
        button.draw(&mut display)?;
        window.update(&display);
    }
    Ok(())
}
