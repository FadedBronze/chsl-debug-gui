use core::f64;

use chsl::{math::vector2::Vector2, physics::{bounding_box::BoundingBox, constraint::Constraint}};

use crate::{gui_eng::{button::ClickElement, number_text::NumberTextInput, text_input::TextInput, DebugGui, DebugGuiLayout, Panel}, panel};

pub struct ConstraintDebugGui {
    panel: Panel,
    position_x: NumberTextInput,
    position_y: NumberTextInput,
    angle: NumberTextInput,
    body: TextInput,
    editing: ClickElement,
    strength: NumberTextInput,
}

impl DebugGuiLayout<Constraint> for ConstraintDebugGui {
    fn get_panel(&mut self) -> &mut crate::gui_eng::Panel {
        &mut self.panel
    }
    fn render_debug_gui(&mut self, data: &mut Constraint, renderer: &mut crate::renderer::Renderer, input: &crate::utils::input::Input, delta_time: f64) {
        panel!(
            self.panel,
            renderer,
            input,
            delta_time,
            self.body,
            self.position_x,
            self.position_y,
            self.angle,
            self.strength,
            self.editing
        );

        if let Constraint::SlideJoint { body, position, angle, strength } = data {
            let max = renderer.size.0.max(renderer.size.1) as f64;

            renderer.set_color(255, 0, 0, 255);
            renderer.line(
                (position.x + angle.cos() * max) as i32, 
                (position.y - angle.sin() * max) as i32, 
                (position.x - angle.cos() * max) as i32, 
                (position.y + angle.sin() * max) as i32, 
            );
        }

        if self.editing.just_clicked() {
            if self.editing.on() {
                match data {
                    Constraint::SlideJoint { body, position, angle, strength } => {
                        self.body.set_value(body.to_string());
                        self.position_x.set_value(position.x);
                        self.position_y.set_value(position.y);
                        self.angle.set_value(*angle);
                        self.strength.set_value(*strength);
                    }
                    _ => {}
                }
            } else {
                match data {
                    Constraint::SlideJoint { body, position, angle, strength } => {
                        *body = self.body.get_value();
                        position.x = self.position_x.get_value();
                        position.y = self.position_y.get_value();
                        *angle = self.angle.get_value();
                        *strength = self.strength.get_value();
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
        match self {
            Constraint::SlideJoint { angle, body, position: pos, strength } => {
                ConstraintDebugGui {
                    strength: NumberTextInput::new(*strength, "Strength"),
                    panel: Panel::new(
                        BoundingBox { 
                            x: position.x, 
                            y: position.y, 
                            width: 200.0, 
                            height: 0.0, 
                        }, 
                        &("SlideJoint Panel ".to_string() + name),
                    ),
                    angle: NumberTextInput::new(*angle, "Angle"),
                    body: TextInput::new(&body, "Body ID"),
                    position_x: NumberTextInput::new(pos.x, "Position X"),
                    position_y: NumberTextInput::new(pos.y, "Position Y"),
                    editing: ClickElement::new_toggle("Edit"),
                }
            }
            _ => {
                panic!("hi")
            }
        }
    }
}

//struct SlideJointLayout {
//    panel: Panel,
//}
//
//impl DebugGuiLayout<SlideJoint> for SlideJointLayout {
//    fn get_panel(&mut self) -> &mut crate::gui_eng::Panel {
//        &mut self.panel
//    }
//    fn render_debug_gui(&mut self, data: &mut T, renderer: &mut crate::renderer::Renderer, input: &crate::utils::input::Input, delta_time: f64) {
//        
//    }
//}
//
//impl DebugGui for SlideJoint {
//    type Layout = SlideJointLayout;
//
//    fn debug_gui(&mut self, position: chsl::math::vector2::Vector2, name: &str) -> Self::Layout {
//        
//    }
//}
