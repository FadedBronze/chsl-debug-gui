mod rigidbody_gui;
mod constraint_guis;

use std::collections::HashMap;

use chsl::{math::{unique_id::{self, unique_id}, vector2::Vector2}, physics::{bounding_box::BoundingBox, constraint::Constraint, rigidbody::RigidBody, world::PhysicsWorld}};
use constraint_guis::ConstraintDebugGui;
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
    bodies_id: TextInput,
    place_body_options: ToggleGroup,

    rigidbody_panels: HashMap<String, RigidBodyDebugGui>,

    constraints_panel: Panel,
    constraints_panel_toggle: ClickElement,
    constraints_debug_guis: HashMap<String, ConstraintDebugGui>,
    constraints_debug_guis_toggles: HashMap<String, ClickElement>,
    add_constraint_type: ToggleGroup,
    add_constraint: ClickElement,
    constraining_body_id_a: TextInput,
    constraining_body_id_b: TextInput,
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
            bodies_id: TextInput::new(&unique_id(), "Body ID"),
            place_body_options: ToggleGroup::new(vec![
                ClickElement::new_toggle("Square"),
                ClickElement::new_toggle("Circle"),
            ]),
            constraints_panel: Panel::new(
                BoundingBox {
                    x: 460.0, 
                    y: 50.0, 
                    width: 200.0, 
                    height: 0.0
                },
                "constraints_panel",
            ),
            constraints_debug_guis_toggles: HashMap::new(),
            constraints_debug_guis: HashMap::new(),
            constraints_panel_toggle: ClickElement::new_toggle("Constraints"),
            constraining_body_id_a: TextInput::new("", "Body A"),
            constraining_body_id_b: TextInput::new("", "Body B"),
            add_constraint_type: ToggleGroup::new(vec![
               ClickElement::new_toggle("Look"), 
               ClickElement::new_toggle("Distance"), 
               ClickElement::new_toggle("Fixed"), 
               ClickElement::new_toggle("Slide"), 
            ]),
            add_constraint: ClickElement::new_button("Add Constraint"),
        }
    } 

    fn render_rigidbody_panels(&mut self, physics_world: &mut PhysicsWorld, renderer: &mut Renderer, input: &Input, delta_time: f64) {
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
    }

    fn render_constraint_panels(&mut self, physics_world: &mut PhysicsWorld, renderer: &mut Renderer, input: &Input, delta_time: f64) {
        let panel_keys: Vec<String> = self.constraints_debug_guis.keys().cloned().collect();
       
        //if world doesn't have a body for our panel remove
        for key in panel_keys {
            if !physics_world.all_constraints().contains_key(&key) {
                self.constraints_debug_guis.remove(&key);
                self.constraints_debug_guis_toggles.remove(&key);
            }
        }
       
        //add a panel for the body into a list if one doesn't exist
        //render the panel
        for (id, constraint) in physics_world.all_constraints().iter_mut() {
            if !self.constraints_debug_guis.contains_key(id) {
                self.constraints_debug_guis_toggles.insert(
                    id.clone(),
                    ClickElement::new_toggle(&("Constraint Panel ".to_string() + id))
                );
                self.constraints_debug_guis.insert(
                    id.clone(), 
                    constraint.debug_gui(Vector2::zero(), &("Constraint Panel ".to_string() + id))
                );
            }

            let panel = self.constraints_debug_guis.get_mut(id).unwrap();

            panel.render_debug_gui(constraint, renderer, input, delta_time);
        }

        self.constraints_panel.hidden = !self.constraints_panel_toggle.on();
       
        //render toggles
        panel!(
            self.constraints_panel,
            renderer,
            input,
            delta_time,
            self.add_constraint_type,
            self.add_constraint,
            self.constraining_body_id_a,
            self.constraining_body_id_b
        );

        if self.add_constraint.just_clicked() {
            match self.add_constraint_type.active_toggle().as_deref() {
                Some("Look") => {

                }
                Some("Distance") => {

                }
                Some("Fixed") => {

                }
                Some("Slide") => {
                    physics_world.add_constraint(&unique_id(), Constraint::SlideJoint {
                        body: self.constraining_body_id_a.get_value(),
                        angle: 0.0,
                        position: Vector2::new(500.0, 500.0),
                        strength: 0.01,
                    })
                }
                Some(_) | None => {
                    panic!("this shouldn't have happened")
                }
            }
        }

        for (k, toggle) in self.constraints_debug_guis_toggles.iter_mut() {
            self.constraints_panel.display(renderer, input, delta_time, toggle);
            self.constraints_debug_guis.get_mut(k).unwrap().get_panel().hidden = !toggle.on();
        }
    }
    
    pub fn render(&mut self, physics_world: &mut PhysicsWorld, renderer: &mut Renderer, input: &Input, delta_time: f64) { 
        self.render_rigidbody_panels(physics_world, renderer, input, delta_time);
        self.render_constraint_panels(physics_world, renderer, input, delta_time);

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
            self.constraints_panel_toggle,
            self.hide
        );

        if self.hide.just_clicked() {
            self.main_panel.hidden = true;
        }

        if input.just_pressed(&Keycode::S) {
            self.main_panel.hidden = !self.main_panel.hidden;
        }

        panel!(
            self.bodies_panel,
            renderer,
            input,
            delta_time,
            self.bodies_id,
            self.place_body_options
        );
        
        if let Some(name) = self.place_bodies.active_toggle() {
            self.bodies_panel.hidden = name != "Bodies";
        } else {
            self.bodies_panel.hidden = true;
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

