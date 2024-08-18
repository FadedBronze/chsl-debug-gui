use chsl::{math::vector2::Vector2, physics::{bounding_box::BoundingBox, constraint::SlideJoint}};

use crate::{gui_eng::{number_text::NumberTextInput, text_input::TextInput, DebugGui, DebugGuiLayout, Panel}, panel};

pub struct SlideJointDebugGui {
    panel: Panel,
    position_x: NumberTextInput,
    position_y: NumberTextInput,
    angle: NumberTextInput,
    body: TextInput,
}

impl DebugGuiLayout<SlideJoint> for SlideJointDebugGui {
    fn get_panel(&mut self) -> &mut crate::gui_eng::Panel {
        &mut self.panel
    }
    fn render_debug_gui(&mut self, data: &mut SlideJoint, renderer: &mut crate::renderer::Renderer, input: &crate::utils::input::Input, delta_time: f64) {
        panel!(
            self.panel,
            renderer,
            input,
            delta_time,
            self.body,
            self.position_x,
            self.position_y,
            self.angle
        )
    }
}

impl DebugGui for SlideJoint {
    type Layout = SlideJointDebugGui;

    fn debug_gui(&mut self, position: Vector2, name: &str) -> Self::Layout {
        SlideJointDebugGui {
            panel: Panel::new(
                BoundingBox { 
                    x: position.x, 
                    y: position.y, 
                    width: 200.0, 
                    height: 0.0, 
                }, 
                &("SlideJoint Panel ".to_string() + name),
            ),
            angle: NumberTextInput::new(self.angle, "Angle"),
            body: TextInput::new(&self.body, "Body ID"),
            position_x: NumberTextInput::new(self.position.x, "Position X"),
            position_y: NumberTextInput::new(self.position.y, "Position Y"),
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
