use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, Window,
    sdl2::{Keycode, MouseButton},
};
use embui::widgets::Button;
use embui::{
    InputEvent, Response, Widget,
    screen::{Draw, Element},
    themes::DefaultTheme,
    widgets::Number,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("buttons.rs", &output_settings);

    let theme = DefaultTheme::<Rgb888>::new();
    #[derive(Clone, Copy)]
    enum Message {
        Increment,
        Decrement,
    }
    struct Model<'a> {
        counter: Number<Message>,
        inc_button: Button<'a, Message, Rgb888>,
        dec_button: Button<'a, Message, Rgb888>,
    }
    impl Model<'_> {
        fn handle_event(&mut self, event: InputEvent) {
            if let Response::Changed(Some(msg)) = self.inc_button.handle_event(event) {
                self.update(msg)
            }
            if let Response::Changed(Some(msg)) = self.counter.handle_event(event) {
                self.update(msg)
            }
            if let Response::Changed(Some(msg)) = self.dec_button.handle_event(event) {
                self.update(msg)
            }
        }
        fn update(&mut self, msg: Message) {
            match msg {
                Message::Increment => {
                    self.counter.set(self.counter.get() + 1);
                }
                Message::Decrement => {
                    self.counter.set(self.counter.get() - 1);
                }
            }
        }
        fn view(
            &self,
        ) -> [&dyn Element<Message, SimulatorDisplay<Rgb888>, DefaultTheme<Rgb888>, Rgb888>; 3]
        {
            [&self.inc_button, &self.counter, &self.dec_button]
        }
    }

    let mut model = Model {
        counter: Number::new(Point::new(0, 64), Size::new(64, 64)),
        inc_button: Button::new("+")
            .on_press(Message::Increment)
            .with_position(Point::new(0, 0))
            .with_size(Size::new(32, 32)),
        dec_button: Button::new("-")
            .on_press(Message::Decrement)
            .with_position(Point::new(0, 128))
            .with_size(Size::new(32, 32)),
    };

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
                model.handle_event(ev);
            }
        }
        model.view().iter().draw_all(&mut display, &theme)?;
        window.update(&display);
    }
    Ok(())
}
