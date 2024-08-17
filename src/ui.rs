use std::collections::HashMap;

use sdl2::mouse::MouseButton;

use crate::{gui::{button::ClickElement, mouse_over_panel, number_text::NumberTextInput, text::Text, text_input::TextInput, toggle::ToggleGroup, DebugGui, DebugGuiLayout, Panel, RigidBodyDebugGui}, math::vector2::Vector2, panel, physics::{bounding_box::BoundingBox, constraint::FixedJoint, rigidbody::RigidBody, world::PhysicsWorld}, renderer::Renderer, utils::input::Input};

pub enum UIPlaceBodyType {
    Circle,
    Square
}

pub struct AppUI {
    //UI
    main_panel: Panel,
    grab_button: ClickElement,
    place_bodies: ToggleGroup,
    move_button: ClickElement,
    static_body: ClickElement,
    random_number: NumberTextInput,
    
    bodies_panel: Panel,
    bodies_panel_open: bool,
    bodies_id: TextInput,
    place_body_options: ToggleGroup,

    rigidbody_panels: HashMap<String, RigidBodyDebugGui>,
   
    //Interface
    pub new_body_id: String,
    pub static_flag: bool,
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
            place_bodies: ToggleGroup::new(vec![
                ClickElement::new_toggle("Bodies"),
                ClickElement::new_toggle("Fish")
            ]),
            static_body: ClickElement::new_toggle("Static"),
            grab_button: ClickElement::new_button("Grab Bodies"),
            move_button: ClickElement::new_button("Move"),
            random_number: NumberTextInput::new(0.0, "random number"),
            
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
            bodies_id: TextInput::new("", "Body ID"),
            place_body_options: ToggleGroup::new(vec![
                ClickElement::new_toggle("Square"),
                ClickElement::new_toggle("Circle"),
            ]),

            //Interface
            static_flag: false,
            new_body_id: String::new(),
        }
    } 
    
    pub fn render(&mut self, physics_world: &mut PhysicsWorld, renderer: &mut Renderer, input: &Input, delta_time: f64) { 
        let panel_keys: Vec<String> = self.rigidbody_panels.keys().cloned().collect();

        for key in panel_keys {
            if !physics_world.bodies.contains_key(&key) {
                self.rigidbody_panels.remove(&key);
            }
        }

        for (id, body) in physics_world.bodies.iter_mut() {
            if !self.rigidbody_panels.contains_key(id) {
                self.rigidbody_panels.insert(id.clone(), body.debug_gui(Vector2::zero(), &("Rigidbody Panel".to_string() + id)));
            }

            let panel = self.rigidbody_panels.get_mut(id).unwrap();

            panel.render_debug_gui(body, renderer, input, delta_time);

            let (x, y) = input.get_mouse_pos();
            let mouse_pos = Vector2::new(x as f64, y as f64);

            if body.within(mouse_pos) && input.just_pressed_mouse(&MouseButton::Left) {
                panel.get_panel().hidden = !panel.get_panel().hidden;
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
            self.random_number
        );
        
        self.static_flag = self.static_body.on();

        self.bodies_panel_open = false;
        
        if let Some(name) = self.place_bodies.active_toggle() {
            self.bodies_panel_open = name == "Bodies";
        }

        if self.bodies_id.updated() {
            self.new_body_id = self.bodies_id.get_value()
        }

        if self.bodies_panel_open {
            panel!(
                self.bodies_panel,
                renderer,
                input,
                delta_time,
                self.bodies_id,
                self.place_body_options
            );
        }
        
        let (x, y) = input.get_mouse_pos();

        if let None = mouse_over_panel(vec![&mut self.main_panel, &mut self.bodies_panel], x, y) {
            if input.just_pressed_mouse(&MouseButton::Left) {
                let (x, y) = input.get_mouse_pos();
                
                let mut new_id = self.new_body_id.clone();
                while physics_world.bodies.contains_key(&new_id) || new_id.len() == 0 {
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
                    }
                    Some(_) | None => {}
                }   
            };
        }
    }
}

