use chsl::{math::vector2::Vector2, physics::{bounding_box::BoundingBox, rigidbody::RigidBody}};
use crate::{gui_eng::{button::ClickElement, number_text::NumberTextInput, text::Text, toggle::ToggleGroup, DebugGui, DebugGuiLayout, Panel}, panel, renderer::Renderer, utils::input::Input};

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
