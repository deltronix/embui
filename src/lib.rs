#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use embedded_graphics::{prelude::*, primitives::Rectangle};

pub mod themes;
pub mod widgets;
use heapless::{FnvIndexMap, Vec};
pub use themes::Theme;
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

/// Unique identifier for widgets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(pub(crate) u16);
impl WidgetId {
    pub const ROOT: WidgetId = WidgetId(0);
    pub const INVALID: WidgetId = WidgetId(u16::MAX);

    pub fn new(id: u16) -> Self {
        WidgetId(id)
    }
}

pub struct WidgetNode<C: PixelColor> {
    id: WidgetId,
    parent: WidgetId,
    children: Vec<WidgetId, 8>,
    widget: Option<Box<dyn Widget<C>>>,
    visible: bool,
    pub bounds: Rectangle,
}
impl<C: PixelColor> WidgetNode<C> {
    fn new(id: WidgetId, parent: WidgetId) -> Self {
        Self {
            id,
            parent,
            children: Vec::new(),
            widget: None,
            visible: true,
            bounds: Rectangle::zero(),
        }
    }
    fn with_widget(id: WidgetId, parent: WidgetId, widget: Box<dyn Widget<C>>) -> Self {
        let bounds = widget.bounding_box();
        Self {
            id,
            parent,
            children: Vec::new(),
            widget: Some(widget),
            visible: true,
            bounds,
        }
    }
    /// Get absolute bounds (including parent transforms)
    pub fn absolute_bounds(&self, tree: &WidgetTree<C>) -> Rectangle {
        if self.parent == WidgetId::INVALID {
            return self.bounds;
        }

        let mut absolute_pos = self.bounds.top_left;
        let mut current_parent = self.parent;

        // Walk up the tree to accumulate transforms
        while current_parent != WidgetId::INVALID {
            if let Some(parent_node) = tree.get_node(current_parent) {
                absolute_pos += parent_node.bounds.top_left;
                current_parent = parent_node.parent;
            } else {
                break;
            }
        }

        Rectangle::new(absolute_pos, self.bounds.size)
    }
}
/// Widget tree for managing hierarchical UI
pub struct WidgetTree<C: PixelColor> {
    nodes: Vec<WidgetNode<C>, 32>, // Max 32 widgets per screen
    next_id: u16,
    root_id: WidgetId,
}

impl<C: PixelColor> WidgetTree<C> {
    pub fn new() -> Self {
        let mut tree = Self {
            nodes: Vec::new(),
            next_id: 1,
            root_id: WidgetId::ROOT,
        };

        // Create root node
        let root_node = WidgetNode::new(WidgetId::ROOT, WidgetId::INVALID);
        tree.nodes.push(root_node).ok();

        tree
    }

    /// Add a widget to the tree
    pub fn add_widget(
        &mut self,
        parent_id: WidgetId,
        widget: Box<dyn Widget<C>>,
    ) -> Result<WidgetId, ()> {
        if self.nodes.is_full() {
            return Err(());
        }

        let widget_id = WidgetId::new(self.next_id);
        self.next_id += 1;

        // Create the widget node
        let widget_node = WidgetNode::with_widget(widget_id, parent_id, widget);

        // Add to parent's children list
        if let Some(parent) = self.get_node_mut(parent_id) {
            if parent.children.push(widget_id).is_err() {
                return Err(()); // Parent is full
            }
        } else {
            return Err(()); // Parent doesn't exist
        }

        // Add node to tree
        self.nodes.push(widget_node).map_err(|_| ())?;

        Ok(widget_id)
    }

    /// Add a container (invisible widget for grouping)
    pub fn add_container(
        &mut self,
        parent_id: WidgetId,
        bounds: Rectangle,
    ) -> Result<WidgetId, ()> {
        if self.nodes.is_full() {
            return Err(());
        }

        let container_id = WidgetId::new(self.next_id);
        self.next_id += 1;

        let mut container_node = WidgetNode::new(container_id, parent_id);
        container_node.bounds = bounds;

        // Add to parent's children list
        if let Some(parent) = self.get_node_mut(parent_id) {
            if parent.children.push(container_id).is_err() {
                return Err(()); // Parent is full
            }
        } else {
            return Err(()); // Parent doesn't exist
        }

        // Add node to tree
        self.nodes.push(container_node).map_err(|_| ())?;

        Ok(container_id)
    }

    /// Remove a widget and all its children
    pub fn remove_widget(&mut self, widget_id: WidgetId) -> bool {
        if widget_id == WidgetId::ROOT {
            return false; // Can't remove root
        }

        // Remove from parent's children list first
        if let Some(node) = self.get_node(widget_id) {
            let parent_id = node.parent;
            if let Some(parent) = self.get_node_mut(parent_id) {
                if let Some(pos) = parent.children.iter().position(|&id| id == widget_id) {
                    parent.children.swap_remove(pos);
                }
            }
        }

        // Recursively remove children
        if let Some(node) = self.get_node(widget_id) {
            let children: Vec<WidgetId, 8> = node.children.clone();
            for &child_id in &children {
                self.remove_widget(child_id);
            }
        }

        // Remove the node itself
        if let Some(pos) = self.nodes.iter().position(|n| n.id == widget_id) {
            self.nodes.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Get a node by ID
    pub fn get_node(&self, id: WidgetId) -> Option<&WidgetNode<C>> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, id: WidgetId) -> Option<&mut WidgetNode<C>> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Get widget by ID
    pub fn get_widget(&self, id: WidgetId) -> Option<&dyn Widget<C>> {
        self.get_node(id)?.widget.as_ref().map(|w| w.as_ref())
    }

    /// Get mutable widget by ID
    pub fn get_widget_mut(&mut self, id: WidgetId) -> Option<&mut dyn Widget<C>> {
        self.get_node_mut(id)?.widget.as_mut().map(|w| w.as_mut())
    }

    /// Set widget visibility
    pub fn set_visible(&mut self, id: WidgetId, visible: bool) -> bool {
        if let Some(node) = self.get_node_mut(id) {
            node.visible = visible;
            true
        } else {
            false
        }
    }

    /// Check if widget is visible (including parents)
    pub fn is_visible(&self, id: WidgetId) -> bool {
        let mut current_id = id;

        while current_id != WidgetId::INVALID {
            if let Some(node) = self.get_node(current_id) {
                if !node.visible {
                    return false;
                }
                current_id = node.parent;
            } else {
                return false;
            }
        }

        true
    }

    /// Find widget at point (top-most visible widget)
    pub fn hit_test(&self, point: Point) -> Option<WidgetId> {
        self.hit_test_recursive(self.root_id, point)
    }

    fn hit_test_recursive(&self, node_id: WidgetId, point: Point) -> Option<WidgetId> {
        if let Some(node) = self.get_node(node_id) {
            if !node.visible {
                return None;
            }

            let absolute_bounds = node.absolute_bounds(self);

            if !absolute_bounds.contains(point) {
                return None;
            }

            // Check children first (they're on top)
            for &child_id in node.children.iter().rev() {
                // Reverse for top-to-bottom
                if let Some(hit) = self.hit_test_recursive(child_id, point) {
                    return Some(hit);
                }
            }

            // If no children hit and this node has a widget, it's the hit
            if node.widget.is_some() {
                Some(node_id)
            } else {
                None // Container nodes don't capture hits
            }
        } else {
            None
        }
    }
}

pub struct Container<C: PixelColor> {
    widgets: heapless::FnvIndexMap<WidgetId, Box<dyn Widget<C>>, 32>,
}

impl<C: PixelColor> Container<C> {
    fn new() -> Self {
        Self {
            widgets: FnvIndexMap::new(),
        }
    }
}

pub trait Widget<C: PixelColor>: Dimensions {
    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);

    fn handle_event(&mut self, event: InputEvent) -> bool {
        let contains_point = match event {
            InputEvent::Touch(point)
            | InputEvent::TouchRelease(point)
            | InputEvent::MouseMove(point)
            | InputEvent::MouseDown(point)
            | InputEvent::MouseUp(point) => self.bounding_box().contains(point),
            _ => false,
        };

        let state_changed = self
            .get_state_manager_mut()
            .handle_event(event, contains_point);

        let widget_handled = self.handle_event_impl(event, contains_point);

        state_changed || widget_handled
    }

    fn get_state(&self) -> WidgetState {
        self.get_state_manager().current_state()
    }

    fn set_state(&mut self, state: WidgetState) -> bool {
        self.get_state_manager_mut().set_state(state)
    }

    fn get_state_manager(&self) -> &StateManager;
    fn get_state_manager_mut(&mut self) -> &mut StateManager;

    fn handle_event_impl(&mut self, _event: InputEvent, _contains_point: bool) -> bool {
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
