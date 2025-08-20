use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::Rectangle};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, Window,
    sdl2::{Keycode, MouseButton},
};
use embedded_layout::{
    align::{horizontal, vertical, Align}, layout::linear::LinearLayout, prelude::Chain, ViewGroup
};
use embui::{
    InputEvent, Response, ThemedWidget, Widget,
    screen::{Draw, Element},
    themes::DefaultTheme,
    widgets::Number,
};
use embui::{Theme, widgets::Button};

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
    #[derive(ViewGroup)]
    struct Model<'a> {
        counter: Number<Message>,
        inc_button: Button<'a, Message>,
        dec_button: Button<'a, Message>,
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

     let chain =  Chain::new(Number::new(Point::new(0, 64), Size::new(64, 64))).append(
        _button: Button::new("+").on_press(Message::Increment))
        dec_button: Button::new("-").on_press(Message::Decrement),

    LinearLayout::vertical(model)
        .with_alignment(horizontal::Center)
        .arrange()
        .align_to_mut(display.bounding_box(), horizontal::Center, vertical::Center)
        .draw();

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
        //model.view().iter().draw_all(&mut display, &theme)?;
        window.update(&display);
    }
    Ok(())
}
