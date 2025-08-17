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
use embui::{
    InputEvent,
    themes::{DefaultTheme, Theme},
    widgets::container::Screen,
};
use embui::{Widget, WidgetTree};
use embui::{WidgetId, widgets::Button};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("buttons.rs", &output_settings);

    let theme = DefaultTheme::new();
    let button: Box<Button<Rgb888>> =
        Box::new(Button::new("test", Point::zero(), Size::new(100, 100)));

    let button2: Box<Button<Rgb888>> = Box::new(Button::new(
        "test2",
        Point::new(100, 0),
        Size::new(100, 100),
    ));

    let rect = Rectangle::new(Point::zero(), Size::new(100, 100)).into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(2)
            .build(),
    );

    let mut screen: Screen<Rgb888, SimulatorDisplay<Rgb888>> =
        Screen::new(Rectangle::new(Point::zero(), Size::new(320, 240)));
    //let mut wt: WidgetTree<Rgb888, SimulatorDisplay<Rgb888>> = WidgetTree::new();
    screen.add_widget(button).unwrap();
    screen.add_widget(button2).unwrap();

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
                        screen.handle_event(embui::InputEvent::MouseUp(point));
                    }
                }
                embedded_graphics_simulator::SimulatorEvent::MouseButtonDown {
                    mouse_btn,
                    point,
                } => {
                    if mouse_btn == MouseButton::Left {
                        screen.handle_event(InputEvent::MouseDown(point));
                    }
                }
                embedded_graphics_simulator::SimulatorEvent::MouseWheel { .. } => {}
                embedded_graphics_simulator::SimulatorEvent::MouseMove { point } => {
                    screen.handle_event(embui::InputEvent::MouseMove(point));
                }
                embedded_graphics_simulator::SimulatorEvent::Quit => {
                    break 'running;
                }
            }
        }
        //button.draw(&mut display)?;
        screen.draw_with_theme(&mut display, &theme)?;
        window.update(&display);
    }
    Ok(())
}
