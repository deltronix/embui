use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::Rectangle};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, Window,
    sdl2::{Keycode, MouseButton},
};
use embui::widgets::Button;
use embui::{InputEvent, screen::Screen, themes::DefaultTheme};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("buttons.rs", &output_settings);

    let theme = DefaultTheme::new();
    let mut counter: i32 = 0;
    let button: Box<Button<Rgb888>> = Box::new(
        Button::new("test", Point::zero(), Size::new(100, 100)).with_callback(|| {
            println!("Pressed button 1");
        }),
    );

    let button2: Box<Button<Rgb888>> = Box::new(
        Button::new("test2", Point::new(100, 100), Size::new(100, 100))
            .with_callback(|| println!("Pressed button 2")),
    );

    let mut screen: Screen<Rgb888, SimulatorDisplay<Rgb888>> =
        Screen::new(Rectangle::new(Point::zero(), Size::new(320, 240)));
    let top = screen
        .add_container(Rectangle::new(Point::zero(), Size::new(320, 100)))
        .unwrap();
    screen.add_widget_to_parent(top, button).unwrap();
    screen.add_widget_to_parent(top, button2).unwrap();

    window.update(&display);
    'running: loop {
        for event in window.events() {
            if let Some(ev) = match event {
                embedded_graphics_simulator::SimulatorEvent::KeyUp { .. } => None,
                embedded_graphics_simulator::SimulatorEvent::KeyDown { keycode, .. } => {
                    if keycode == Keycode::Q {
                        break 'running;
                    } else {
                        None
                    }
                }
                embedded_graphics_simulator::SimulatorEvent::MouseButtonUp { mouse_btn, point } => {
                    if mouse_btn == MouseButton::Left {
                        Some(embui::InputEvent::MouseUp(point))
                    } else {
                        None
                    }
                }
                embedded_graphics_simulator::SimulatorEvent::MouseButtonDown {
                    mouse_btn,
                    point,
                } => {
                    if mouse_btn == MouseButton::Left {
                        Some(InputEvent::MouseDown(point))
                    } else {
                        None
                    }
                }
                embedded_graphics_simulator::SimulatorEvent::MouseWheel { .. } => None,
                embedded_graphics_simulator::SimulatorEvent::MouseMove { point } => {
                    Some(embui::InputEvent::MouseMove(point))
                }
                embedded_graphics_simulator::SimulatorEvent::Quit => {
                    break 'running;
                }
            } {
                screen.handle_event(ev);
            }
        }
        screen.draw_with_theme(&mut display, &theme)?;
        window.update(&display);
    }
    Ok(())
}
