use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_layout::{
    align::{Align, horizontal, vertical},
    layout::linear::LinearLayout,
    prelude::Views,
};
use embui::widgets::Button;
use embui::{ThemedWidget, themes::DefaultTheme};

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

    let b: Button<Message, Rgb888> = Button::new("test").with_size(Size::new(32, 32));
    let mut v = [
        b.clone().on_press(Message::Increment),
        b.clone().on_press(Message::Decrement),
        b.clone(),
        b.clone(),
        b.clone(),
        b.clone(),
        b.clone(),
        b.clone(),
        b.clone(),
        b.clone(),
    ];
    //let n = Number::<Message>::new(Point::new(0, 0), Size::new(32, 32));
    let views = Views::new(&mut v);

    LinearLayout::horizontal(views)
        .arrange()
        .align_to(&display.bounding_box(), horizontal::Center, vertical::Top)
        .arrange()
        .inner()
        .iter()
        .for_each(|l| l.draw_with_theme(&mut display, &theme).expect("draw error"));

    window.show_static(&display);
    Ok(())
}
