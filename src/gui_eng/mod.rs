pub mod button;
pub mod text;
pub mod text_input;
pub mod toggle;
pub mod number_text;

use core::f64;
use std::u32;

use chsl::{math::vector2::Vector2, physics::bounding_box::BoundingBox};
use crate::{renderer::Renderer, utils::input::Input};
use sdl2::mouse::MouseButton;

const PADDING: u32 = 20;

pub trait Element {
    fn render(&mut self, renderer: &mut Renderer, input: &Input, panel: &mut Panel, delta_time: f64);
    fn bounds(&mut self) -> (i32, i32);
}

pub struct Panel {
    pub name: String,
    bounds: BoundingBox,
    y_offset: i32,
    drag_origin: Option<Vector2>,
    pub hidden: bool,
}

#[macro_export]
macro_rules! panel {
    (
        $panel:expr,
        $renderer:expr,
        $input:expr,
        $delta_time:expr,
        $($element:expr),*
    ) => {
        {
            $panel.render($renderer, $input);
            $panel.drag_start($input);
            $panel.drag_end($input);

            $(
                $panel.display(
                    $renderer,
                    $input,
                    $delta_time,
                    &mut $element
                );
            )*
        }
    };
}

impl Panel {
    pub fn display<T: Element>(&mut self, renderer: &mut Renderer, input: &Input, delta_time: f64, element: &mut T) -> &mut Self {
        if self.hidden { return self };
        element.render(renderer, input, self, delta_time);
        self.y_offset += element.bounds().0 + PADDING as i32 / 2;
        if self.y_offset as f64 > self.bounds.height {
            self.bounds.height = self.y_offset as f64; 
        }
        self
    }

    pub fn render(&mut self, renderer: &mut Renderer, input: &Input) {
        if self.hidden { return };
        self.y_offset = PADDING as i32 / 2;
        
        let offset = if let Some(origin) = self.drag_origin {
            let (x, y) = input.get_mouse_pos();
            let pos = Vector2::new(x as f64, y as f64);
            pos - origin
        } else {
            Vector2::zero()
        };
        
        renderer.set_color(40, 40, 40, 255);
        renderer.fill_rect(
            (self.bounds.x + offset.x) as i32, 
            (self.bounds.y + offset.y) as i32, 
            self.bounds.width as i32, 
            self.bounds.height as i32
        );
    }

    pub fn new(bounds: BoundingBox, name: &str) -> Panel {
        Panel { bounds, hidden: true, drag_origin: None, y_offset: PADDING as i32 / 2, name: name.to_string() }
    }

    pub fn drag_start(&mut self, input: &Input) {
        let (x, y) = input.get_mouse_pos();
        let origin = Vector2::new(x as f64, y as f64);

        if self.bounds.point_within(origin) && input.just_pressed_mouse(&MouseButton::Left) {
            self.drag_origin = Some(origin);
        }
    }

    pub fn drag_end(&mut self, input: &Input) {
        if let Some(origin) = self.drag_origin {
            if input.just_released_mouse(&MouseButton::Left) {
                let (x, y) = input.get_mouse_pos();
                let pos = Vector2::new(x as f64, y as f64);
                let offset = pos - origin;

                self.bounds.x += offset.x;
                self.bounds.y += offset.y;

                self.drag_origin = None;
            }
        } 
    }
}

pub fn mouse_over_panel(panels: Vec<&mut Panel>, x: i32, y: i32) -> Option<String> {
    for panel in panels.iter() {
        if panel.bounds.point_within(Vector2::new(x as f64, y as f64)) {
            return Some(panel.name.clone()); 
        }
    }
    None
}

pub trait DebugGui: Sized {
    fn debug_gui(&mut self, position: Vector2, name: &str) -> Self::Layout;
    type Layout: DebugGuiLayout<Self>;
}

pub trait DebugGuiLayout<T> {
    fn get_panel(&mut self) -> &mut Panel;
    fn render_debug_gui(&mut self, data: &mut T, renderer: &mut Renderer, input: &Input, delta_time: f64);
}
