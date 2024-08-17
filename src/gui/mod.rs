pub mod button;
pub mod text;
pub mod text_input;
pub mod toggle;
pub mod number_text;

use core::f64;
use std::u32;

use button::ClickElement;
use chsl::{math::vector2::Vector2, physics::{bounding_box::BoundingBox, rigidbody::RigidBody}};
use crate::{renderer::Renderer, utils::input::Input};
use number_text::NumberTextInput;
use sdl2::mouse::MouseButton;
use text::Text;
use toggle::ToggleGroup;

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

            $(
                $panel.display(
                    $renderer,
                    $input,
                    $delta_time,
                    &mut $element
                );
            )*

            $panel.drag_start($input);
            $panel.drag_end($input);
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
        Panel { bounds, hidden: false, drag_origin: None, y_offset: PADDING as i32 / 2, name: name.to_string() }
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

pub struct RigidBodyDebugGui {
    panel: Panel,
    pub id_text: Text,
    position_x: NumberTextInput,
    position_y: NumberTextInput,
    velocity_x: NumberTextInput,
    velocity_y: NumberTextInput,
    angular_velocity: NumberTextInput,
    rotation: NumberTextInput,
    body_type: ToggleGroup,
    mass: NumberTextInput,
    inertia: NumberTextInput,
    scale_x: NumberTextInput,
    scale_y: NumberTextInput,
    gravity_scale: NumberTextInput,
    editing: ClickElement,
    hide: ClickElement,
    delete: ClickElement,
}

impl RigidBodyDebugGui {
    fn get_id(&self) -> String {
        self.id_text.content.clone()
    }
}

impl DebugGuiLayout<RigidBody> for RigidBodyDebugGui {
    fn get_panel(&mut self) -> &mut Panel {
        &mut self.panel
    }
    fn render_debug_gui(&mut self, body: &mut RigidBody, renderer: &mut Renderer, input: &Input, delta_time: f64) {
        panel!(
            self.panel,
            renderer,
            input,
            delta_time,
            self.id_text,
            self.position_x,
            self.position_y,
            self.rotation,
            self.scale_x,
            self.scale_y,
            self.velocity_x,
            self.velocity_y,
            self.angular_velocity,
            self.body_type,
            self.mass,
            self.inertia,
            self.gravity_scale,
            self.editing,
            self.hide,
            self.delete
        );

        if self.editing.on() {
            body.position = Vector2::new(self.position_x.get_value(), self.position_y.get_value());
            body.scale = Vector2::new(self.scale_x.get_value(), self.scale_y.get_value());
            body.inv_mass = 1.0 / self.mass.get_value();
            body.rotation = self.rotation.get_value();
            //TODO
        } else {
            self.position_x.set_value(body.position.x);
            self.position_y.set_value(body.position.y);
            self.scale_x.set_value(body.scale.x);
            self.scale_y.set_value(body.scale.y);
            self.mass.set_value(1.0 / body.inv_mass);
            self.rotation.set_value(body.rotation);
        }

        if self.hide.just_clicked() {
            self.panel.hidden = true;
        }

        body.deleted = self.delete.just_clicked();
    }
}

impl DebugGui for RigidBody {
    type Layout = RigidBodyDebugGui;

    fn debug_gui(&mut self, position: Vector2, name: &str) -> Self::Layout {
        let mut result = Self::Layout {
            angular_velocity: NumberTextInput::new(self.ang_velocity, "Angular Velocity"),
            editing: ClickElement::new_toggle("Editing"),
            id_text: Text::new(2, &("ID: ".to_string() + name)),
            panel: Panel::new(
                BoundingBox { 
                    x: position.x, 
                    y: position.y, 
                    width: 200.0, 
                    height: 0.0 
                }, 
                name
            ),
            body_type: ToggleGroup::new(vec![
                ClickElement::new_toggle("Static"),
                ClickElement::new_toggle("Dynamic"),
            ]),
            velocity_x: NumberTextInput::new(self.velocity.x, "X Velocity"),
            velocity_y: NumberTextInput::new(self.velocity.y, "Y Velocity"),
            position_x: NumberTextInput::new(self.position.x, "X Position"),
            position_y: NumberTextInput::new(self.position.y, "Y Position"),
            rotation: NumberTextInput::new(self.rotation, "Rotation"),
            mass: NumberTextInput::new(1.0 / self.inv_mass, "Mass"),
            inertia: NumberTextInput::new(1.0 / self.inv_inertia, "Inertia"),
            scale_x: NumberTextInput::new(self.scale.x, "Scale X"),
            scale_y: NumberTextInput::new(self.scale.y, "Scale Y"),
            gravity_scale: NumberTextInput::new(self.gravity_scale, "Gravity Scale"),
            hide: ClickElement::new_button("Hide"),
            delete: ClickElement::new_button("Delete"),
        };
        result.panel.hidden = true;
        result
    }
}
