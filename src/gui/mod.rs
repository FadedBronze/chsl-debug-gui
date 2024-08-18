mod rigidbody_gui;
mod constraint_guis;

use std::collections::HashMap;

use chsl::{math::{unique_id::{self, unique_id}, vector2::Vector2}, physics::{bounding_box::BoundingBox, rigidbody::RigidBody, world::PhysicsWorld}};
use rigidbody_gui::RigidBodyDebugGui;
use sdl2::{keyboard::Keycode, mouse::MouseButton};

use crate::{gui_eng::{button::ClickElement, mouse_over_panel, text_input::TextInput, toggle::ToggleGroup, DebugGui, DebugGuiLayout, Panel}, panel, renderer::Renderer, utils::input::Input};

pub struct AppUI {
    //UI
    main_panel: Panel,
    grab_button: ClickElement,
    place_bodies: ToggleGroup,
    move_button: ClickElement,
    view_debug_body: ClickElement,
    static_body: ClickElement,
    hide: ClickElement,
    
    bodies_panel: Panel,
    bodies_panel_open: bool,
    bodies_id: TextInput,
    place_body_options: ToggleGroup,

    rigidbody_panels: HashMap<String, RigidBodyDebugGui>,
}

impl AppUI {
    pub fn new() -> Self { 
        Self {
            //UI
            rigidbody_panels: HashMap::new(),
            main_panel: Panel::new(
                BoundingBox {
                    x: 50.0, 
                    y: 50.0, 
                    width: 200.0, 
                    height: 0.0
                },
                "main_panel",
            ),
            view_debug_body: ClickElement::new_toggle("Debug Body"),
            place_bodies: ToggleGroup::new(vec![
                ClickElement::new_toggle("Bodies"),
                ClickElement::new_toggle("Fish")
            ]),
            static_body: ClickElement::new_toggle("Static"),
            grab_button: ClickElement::new_button("Grab Bodies"),
            move_button: ClickElement::new_button("Move"),
            hide: ClickElement::new_button("Hide"),
            
            bodies_panel: Panel::new(
                BoundingBox {
                    x: 255.0, 
                    y: 50.0, 
                    width: 200.0, 
                    height: 0.0
                },
                "bodies_panel",
            ),
            bodies_panel_open: false,
            bodies_id: TextInput::new(&unique_id(), "Body ID"),
            place_body_options: ToggleGroup::new(vec![
                ClickElement::new_toggle("Square"),
                ClickElement::new_toggle("Circle"),
            ]),
        }
    } 
    
    pub fn render(&mut self, physics_world: &mut PhysicsWorld, renderer: &mut Renderer, input: &Input, delta_time: f64) { 
        let panel_keys: Vec<String> = self.rigidbody_panels.keys().cloned().collect();

        for key in panel_keys {
            if !physics_world.all_bodies().contains_key(&key) {
                self.rigidbody_panels.remove(&key);
            }
        }

        for (id, body) in physics_world.all_bodies().iter_mut() {
            if !self.rigidbody_panels.contains_key(id) {
                self.rigidbody_panels.insert(
                    id.clone(), 
                    body.debug_gui(Vector2::zero(), &("Rigidbody Panel".to_string() + id))
                );
            }

            let panel = self.rigidbody_panels.get_mut(id).unwrap();

            panel.render_debug_gui(body, renderer, input, delta_time);

            let (x, y) = input.get_mouse_pos();
            let mouse_pos = Vector2::new(x as f64, y as f64);

            if self.view_debug_body.on() {
                if body.within(mouse_pos) && input.just_pressed_mouse(&MouseButton::Left) {
                    panel.get_panel().hidden = false;
                }
            }
        }

        panel!(
            self.main_panel,
            renderer,
            input,
            delta_time,
            self.grab_button,
            self.move_button,
            self.static_body,
            self.place_bodies,
            self.view_debug_body,
            self.hide
        );

        if self.hide.just_clicked() {
            self.main_panel.hidden = true;
        }

        if input.just_pressed(&Keycode::S) {
            self.main_panel.hidden = !self.main_panel.hidden;
        }
        
        if let Some(name) = self.place_bodies.active_toggle() {
            if name == "Bodies" {
                panel!(
                    self.bodies_panel,
                    renderer,
                    input,
                    delta_time,
                    self.bodies_id,
                    self.place_body_options
                );
            }
        }
        
        let (x, y) = input.get_mouse_pos();

        if let None = mouse_over_panel(vec![&mut self.main_panel, &mut self.bodies_panel], x, y) {
            if input.just_pressed_mouse(&MouseButton::Left) {
                let (x, y) = input.get_mouse_pos();
                
                let mut new_id = self.bodies_id.get_value();
                while physics_world.all_bodies().contains_key(&new_id) || new_id.len() == 0 {
                    new_id = new_id + " Copy";
                }

                let new_position = Vector2::new(x as f64, y as f64);

                match self.place_body_options.active_toggle().as_deref() {
                    Some("Circle") => {
                        physics_world.add_body(
                            &new_id, 
                            RigidBody::new_circle(
                                new_position,
                                0.0, 
                                30.0,
                                self.static_body.on(),
                            )
                        );
                        self.bodies_id.set_value(unique_id());
                    }
                    Some("Square") => {
                        physics_world.add_body(
                            &new_id, 
                            RigidBody::new_square(
                                new_position,
                                0.0, 
                                Vector2::new(30.0, 30.0),
                                self.static_body.on(),
                            )
                        );
                        self.bodies_id.set_value(unique_id());
                    }
                    Some(_) | None => {}
                }   
            };
        }
    }
}

