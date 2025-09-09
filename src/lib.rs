#![no_std]

extern crate alloc;

use core::fmt::Debug;
use core::marker::PhantomData;

use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_layout::View;
use embedded_layout::view_group::ViewGroup;

pub mod screen;
pub mod themes;
pub mod widgets;
pub use crate::widgets::Widget;
use heapless::Vec;
pub use themes::Theme;
pub use widgets::state::Stateful;
pub use widgets::{StateManager, WidgetState};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEvent {
    Touch(Point),
    TouchRelease(Point),
    KeyPress(char),
    MouseMove(Point),
    MouseDown(Point),
    MouseUp(Point),
}
#[derive(Debug, Clone, Copy)]
pub enum Response<M: Clone + Copy> {
    Changed(Option<M>),
    NotChanged,
}

pub trait ThemedWidget<D: DrawTarget, T: Theme<C>, C: PixelColor + Default + From<Rgb888>> {
    fn draw_with_theme(&self, target: &mut D, theme: &T) -> Result<(), D::Error>;
}

#[derive(Debug)]
struct Placement(Option<Point>, Option<Size>);

impl Dimensions for Placement {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(
            self.0.unwrap_or(Point::zero()),
            self.1.unwrap_or(Size::zero()),
        )
    }
}

#[derive(Debug)]
struct ElementId(u16);
impl ElementId {
    fn new(id: u16) -> Self {
        Self(id)
    }
}

/// A gui element holds the Widget variant with it's StateManager a reference to the style used to
/// draw the element and an element ID.
#[derive(Debug)]
pub struct Element<'a, M, C> {
    _msg: PhantomData<M>,
    theme: &'a dyn Theme<C>,
    id: ElementId,
    placement: Placement,
    state_manager: StateManager,
    widget: Widget<'a>,
    parent: Option<ElementId>,
}

impl<'a, M, C> Dimensions for Element<'a, M, C> {
    fn bounding_box(&self) -> Rectangle {
        self.placement.bounding_box()
    }
}

impl<'a, M: Copy, C> Stateful<M> for Element<'a, M, C> {
    fn get_state_manager(&self) -> &StateManager {
        &self.state_manager
    }

    fn get_state_manager_mut(&mut self) -> &mut StateManager {
        &mut self.state_manager
    }
}

impl<'a, M, C> View for Element<'a, M, C> {
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        self.placement.translate_impl(by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.placement.bounds()
    }
}

impl<M, C> Drawable for Element<'_, M, C>
where
    C: PixelColor + Default,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        Ok(())
    }
}
impl View for Placement {
    fn translate_impl(&mut self, by: embedded_graphics::prelude::Point) {
        self.0 = Some(self.0.unwrap_or(Point::zero()) + by);
    }

    fn bounds(&self) -> embedded_graphics::primitives::Rectangle {
        Rectangle::new(
            self.0.unwrap_or(Point::zero()),
            self.1.unwrap_or(Size::zero()),
        )
    }
}

impl<'a, M, C> Element<'a, M, C> {
    pub fn new(theme: &'a dyn Theme<C>, widget: Widget<'a>, id: u16) -> Self {
        Self {
            _msg: PhantomData,
            id: ElementId::new(id),
            theme,
            placement: Placement(None, None),
            state_manager: StateManager::new(),
            widget,
            parent: None,
        }
    }
}
#[derive(Debug)]
struct Container<'a, C, M, const N: usize>
where
    C: PixelColor + Default,
    M: Copy + Clone,
{
    default_theme: &'a dyn Theme<C>,
    placement: Placement,
    container_id: ElementId,
    elements: heapless::Vec<Element<'a, M, C>, N>,
}

impl<'a, C, M, const N: usize> Container<'a, C, M, N>
where
    C: PixelColor + Default + Debug,
    M: Copy + Clone + Debug,
{
    fn new(default_theme: &'a dyn Theme<C>, placement: Placement, container_id: ElementId) -> Self {
        Self {
            default_theme,
            placement,
            container_id,
            elements: heapless::Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Widget<'a>) {
        self.elements
            .push(Element::new(
                self.default_theme,
                widget,
                self.elements.len() as u16,
            ))
            .expect("Container full!");
    }
    fn get_element(&'a self, id: u16) -> Option<&'a Element<'a, M, C>> {
        self.elements.get(id as usize)
    }
}
impl<'a, C, M, const N: usize> Drawable for Container<'a, C, M, N>
where
    C: PixelColor + Default + Debug,
    M: Copy + Clone + Debug,
{
    type Color = C;

    type Output = Response<M>;

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        for el in self.elements.iter() {}
        Ok(Response::NotChanged)
    }
}

#[cfg(test)]
mod tests {
    use crate::Container;
    use crate::{Element, Widget, themes::DefaultTheme};
    use embedded_graphics::pixelcolor::Rgb888;

    #[test]
    fn it_works() {
        #[derive(Clone, Copy, Debug)]
        enum Message {
            Test,
        }
        let theme = DefaultTheme::<Rgb888>::new();
        let widget = Widget::button("test");
        let mut container: Container<Rgb888, Message, 16> =
            Container::new(&theme, crate::Placement(None, None), crate::ElementId(0));
        container.add_widget(Widget::button("test"));
        container.add_widget(Widget::button("test2"));

        container.get_element(0);
    }
}
