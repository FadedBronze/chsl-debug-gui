mod gui_eng;
mod gui;
mod utils;
mod renderer;

use utils::input::Input;
use renderer::Renderer;
use gui::AppUI;
use utils::TimerUtil;
use std::{f64::consts::PI, f64, time::{SystemTime, UNIX_EPOCH}};
use chsl::{math::{matrix::Matrix, vector2::Vector2}, physics::{bounding_box::BoundingBox, rigidbody::Collider, world::PhysicsWorld}};

pub fn loop_with_dt<F: FnMut(f64) -> bool>(mut tick: F) {
    let mut last_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    loop { 
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let dt = now - last_time;
        last_time = now;

        if !tick(dt.as_secs_f64()) {
            return;
        }
    }
}

trait DebugRender {
    fn debug_render(&mut self, renderer: &mut Renderer);
}

impl DebugRender for PhysicsWorld {
    fn debug_render(&mut self, renderer: &mut Renderer) {
        for (_, body) in self.all_bodies().iter_mut() {
            let mut points = vec![];

            match &body.collider {
                Collider::Circle { radius } => {
                    let mut angle: f64 = 0.0;
                    let iterations: f64 = 16.0;

                    for _ in 0..iterations as usize {
                        angle += PI * 2.0 / iterations;
                        points.push(Vector2::new(angle.sin() * radius, angle.cos() * radius))
                    }
                }

                Collider::Polygon { vertices } => {
                    points.append(&mut vertices.clone()); 
                }
            }

            let transform = Matrix::new().scale(body.scale).rot(body.rotation);
            
            for point in points.iter_mut() {
                *point = transform.vec_mul(point);
            }
            
            let mut last = &points[points.len()-1];
            
            for i in 0..points.len() {
                let current = &points[i];
                
                renderer.set_color(0, 0, 0, 255);

                let a = *last + body.position;
                let b = *current + body.position;

                renderer.line(a.x as i32, a.y as i32, b.x as i32, b.y as i32);

                last = current;
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();  

    let mut input = Input::new(sdl_context.event_pump().unwrap());

    let mut physics_world = PhysicsWorld::new(BoundingBox {
        x: 0.0,
        y: 0.0,
        width: 1240.0,
        height: 880.0,
    });
    
    let mut ui = AppUI::new();

    let mut renderer = Renderer::new();

    let mut timer_util = TimerUtil::new();

    loop_with_dt(| delta_time | {
        renderer.clear(255, 255, 255, 255);

        if input.close_button() {
            return false;
        }

        input.update();

        timer_util.start("UI");
        ui.render(&mut physics_world, &mut renderer, &input, delta_time);
        timer_util.stop_log_secs("UI");
        
        timer_util.start("physics");
        physics_world.update(delta_time, 1);
        timer_util.stop_log_secs("physics");

        physics_world.debug_render(&mut renderer);
       
        renderer.update();

        true
    })
}
