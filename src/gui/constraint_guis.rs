use core::f64;

use chsl::{math::vector2::Vector2, physics::{bounding_box::BoundingBox, constraint::Constraint}};

use crate::{gui_eng::{button::ClickElement, number_text::NumberTextInput, text_input::TextInput, DebugGui, DebugGuiLayout, Panel}, panel};

pub enum ConstraintGuiType {
    Slider {
        position_x: NumberTextInput,
        position_y: NumberTextInput,
        rotation: NumberTextInput,
        body: TextInput,
        strength: NumberTextInput,
    },
    Fixed {
        position_x: NumberTextInput,
        position_y: NumberTextInput,
        rotation: NumberTextInput,
        body: TextInput,
        strength: NumberTextInput,
    }
}

pub struct ConstraintDebugGui {
    panel: Panel,
    editing: ClickElement,
    constraint_type: ConstraintGuiType,
}

impl DebugGuiLayout<Constraint> for ConstraintDebugGui {
    fn get_panel(&mut self) -> &mut crate::gui_eng::Panel {
        &mut self.panel
    }
    fn render_debug_gui(&mut self, data: &mut Constraint, renderer: &mut crate::renderer::Renderer, input: &crate::utils::input::Input, delta_time: f64) {
        match &mut self.constraint_type {
            ConstraintGuiType::Slider { position_x, position_y, rotation: angle, body, strength } => {
                panel!(
                    self.panel,
                    renderer,
                    input,
                    delta_time,
                    body,
                    position_x,
                    position_y,
                    angle,
                    strength,
                    &mut self.editing
                );
            }
            ConstraintGuiType::Fixed { position_x, position_y, rotation: angle, body, strength } => {
                panel!(
                    self.panel,
                    renderer,
                    input,
                    delta_time,
                    body,
                    position_x,
                    position_y,
                    angle,
                    strength,
                    &mut self.editing
                );
            }
        }

        if let Constraint::SlideJoint { body, position, rotation, strength } = data {
            let max = renderer.size.0.max(renderer.size.1) as f64;

            renderer.set_color(255, 0, 0, 255);
            renderer.line(
                (position.x + rotation.cos() * *rotation) as i32, 
                (position.y - rotation.sin() * *rotation) as i32, 
                (position.x - rotation.cos() * *rotation) as i32, 
                (position.y + rotation.sin() * *rotation) as i32, 
            );
        }

        if self.editing.just_clicked() {
            if self.editing.on() {
                match (data, &mut self.constraint_type) {
                    (
                        Constraint::SlideJoint { body, position, rotation, strength }, 
                        ConstraintGuiType::Slider { position_x, position_y, rotation: angle_text, body: body_text, strength: strength_text }
                    ) => {
                        body_text.set_value(body.to_string());
                        position_x.set_value(position.x);
                        position_y.set_value(position.y);
                        angle_text.set_value(*rotation);
                        strength_text.set_value(*strength);
                    }
                    (
                        Constraint::FixedJoint { body, position, rotation, strength },
                        ConstraintGuiType::Fixed { position_x, position_y, rotation: rotation_text, body: body_text, strength: strength_text }
                    ) => {
                        body_text.set_value(body.to_string());
                        position_x.set_value(position.x);
                        position_y.set_value(position.y);
                        rotation_text.set_value(*rotation);
                        strength_text.set_value(*strength);
                    }
                    _ => {}
                }
            } else {
                match (data, &mut self.constraint_type) {
                    (
                        Constraint::SlideJoint { body, position, rotation, strength }, 
                        ConstraintGuiType::Slider { position_x, position_y, rotation: rotation_text, body: body_text, strength: strength_text }
                    ) => {
                        *body = body_text.get_value();
                        position.x = position_x.get_value();
                        position.y = position_y.get_value();
                        *rotation = rotation_text.get_value();
                        *strength = strength_text.get_value();
                    }
                    (
                        Constraint::FixedJoint { body, position, rotation, strength }, 
                        ConstraintGuiType::Fixed { position_x, position_y, rotation: rotation_text, body: body_text, strength: strength_text }
                    ) => {
                        *body = body_text.get_value();
                        position.x = position_x.get_value();
                        position.y = position_y.get_value();
                        *rotation = rotation_text.get_value();
                        *strength = strength_text.get_value();
                    }
                    _ => {}
                }
            }
        }
    }
}

impl DebugGui for Constraint {
    type Layout = ConstraintDebugGui;

    fn debug_gui(&mut self, position: Vector2, name: &str) -> Self::Layout {
        ConstraintDebugGui {
            constraint_type: match self {
                Constraint::SlideJoint { body, position, rotation, strength } => {
                    ConstraintGuiType::Slider { 
                        strength: NumberTextInput::new(*strength, "Strength"),
                        rotation: NumberTextInput::new(*rotation, "Angle"),
                        body: TextInput::new(&body, "Body ID"),
                        position_x: NumberTextInput::new(position.x, "Position X"),
                        position_y: NumberTextInput::new(position.y, "Position Y"),
                    }
                }
                Constraint::FixedJoint { body, position, rotation, strength } => {
                    ConstraintGuiType::Fixed { 
                        strength: NumberTextInput::new(*strength, "Strength"),
                        rotation: NumberTextInput::new(*rotation, "Angle"),
                        body: TextInput::new(&body, "Body ID"),
                        position_x: NumberTextInput::new(position.x, "Position X"),
                        position_y: NumberTextInput::new(position.y, "Position Y"),
                    }
                }
                _ => {
                    panic!("Invalid Gui Type")
                }
            },
            panel: Panel::new(
                BoundingBox { 
                    x: position.x, 
                    y: position.y, 
                    width: 200.0, 
                    height: 0.0, 
                }, 
                &("SlideJoint Panel ".to_string() + name),
            ),
            editing: ClickElement::new_toggle("Edit"),
        }
    }
}
