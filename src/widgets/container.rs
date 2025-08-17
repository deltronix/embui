use crate::themes::DefaultTheme;
use crate::{Box, InputEvent, StateManager, Theme, Widget, WidgetId, WidgetState, WidgetTree};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

pub struct Screen<C: PixelColor, D: DrawTarget<Color = C>> {
    widget_tree: WidgetTree<C, D>,
    bounds: Rectangle,
    background_color: Option<C>,
    state_manager: StateManager,
}

impl<C: PixelColor, D: DrawTarget<Color = C>> Screen<C, D> {
    pub fn new(bounds: Rectangle) -> Self {
        Self {
            widget_tree: WidgetTree::new(),
            bounds,
            background_color: None,
            state_manager: StateManager::new(),
        }
    }

    pub fn with_background(mut self, color: C) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Add a widget to the screen (to root container)
    pub fn add_widget(&mut self, widget: Box<dyn Widget<C, D>>) -> Result<WidgetId, &str> {
        self.widget_tree.add_widget(WidgetId::ROOT, widget)
    }

    /// Add a widget to a specific parent
    pub fn add_widget_to_parent(
        &mut self,
        parent_id: WidgetId,
        widget: Box<dyn Widget<C, D>>,
    ) -> Result<WidgetId, &str> {
        self.widget_tree.add_widget(parent_id, widget)
    }

    /// Add a container for grouping widgets
    pub fn add_container(&mut self, bounds: Rectangle) -> Result<WidgetId, ()> {
        self.widget_tree.add_container(WidgetId::ROOT, bounds)
    }

    /// Add a container to a specific parent
    pub fn add_container_to_parent(
        &mut self,
        parent_id: WidgetId,
        bounds: Rectangle,
    ) -> Result<WidgetId, ()> {
        self.widget_tree.add_container(parent_id, bounds)
    }

    /// Remove a widget
    pub fn remove_widget(&mut self, widget_id: WidgetId) -> bool {
        self.widget_tree.remove_widget(widget_id)
    }

    /// Get widget by ID
    pub fn get_widget(&self, id: WidgetId) -> Option<&dyn Widget<C, D>> {
        self.widget_tree.get_widget(id)
    }

    /// Get mutable widget by ID
    pub fn get_widget_mut(&mut self, id: WidgetId, f: &dyn Fn(&mut dyn Widget<C, D>)) {
        match self.widget_tree.get_node_mut(id) {
            None => {}
            Some(node) => {
                if let Some(w) = node.widget.as_mut() {
                    f(w.as_mut());
                }
            }
        }
        //self.get_node_mut(id)?.widget.as_mut().map(|w| w.as_mut())
    }

    /// Set widget visibility
    pub fn set_visible(&mut self, id: WidgetId, visible: bool) -> bool {
        self.widget_tree.set_visible(id, visible)
    }

    /// Handle input event
    pub fn handle_event(&mut self, event: InputEvent) -> bool {
        self.widget_tree.handle_event(event);
        true
    }

    /// Draw the entire screen
    pub fn draw(&mut self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
        C: From<embedded_graphics::pixelcolor::Rgb888>,
    {
        let theme = DefaultTheme::<C>::new();
        self.draw_with_theme(target, &theme)
    }

    /// Draw the screen with a custom theme
    pub fn draw_with_theme(&mut self, target: &mut D, theme: &dyn Theme<C>) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        // Clear background if specified
        if let Some(bg_color) = self.background_color {
            use embedded_graphics::primitives::{
                PrimitiveStyle, Rectangle as DrawRect, StyledDrawable,
            };
            let bg_style = PrimitiveStyle::with_fill(bg_color);
            DrawRect::new(self.bounds.top_left, self.bounds.size).draw_styled(&bg_style, target)?;
        }

        // Draw all widgets
        self.widget_tree.draw_all(target, theme);
        Ok(())
    }

    /// Get screen bounds
    pub fn bounds(&self) -> Rectangle {
        self.bounds
    }

    /// Find widget at point
    pub fn hit_test(&self, point: Point) -> Option<WidgetId> {
        if self.bounds.contains(point) {
            self.widget_tree.hit_test(point)
        } else {
            None
        }
    }

    /// Get access to the widget tree
    pub fn tree(&self) -> &WidgetTree<C, D> {
        &self.widget_tree
    }

    /// Get mutable access to the widget tree
    pub fn tree_mut(&mut self) -> &mut WidgetTree<C, D> {
        &mut self.widget_tree
    }
}
impl<C, D> Dimensions for Screen<C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C, Error = ()>,
{
    fn bounding_box(&self) -> Rectangle {
        self.bounds
    }
}
