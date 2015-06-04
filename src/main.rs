// Imports
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::Button::{ Keyboard, Mouse };
use piston::input::Input;
use piston::input::keyboard::Key;
use piston::input::Motion::MouseCursor;

const windowX: f64 = 800.0;
const windowY: f64 = 800.0;
const windowSize: [f32; 2] = [800.0, 800.0];
pub struct Body {
    position: (f64, f64),
    velocity: (f64, f64),
    acceleration: (f64, f64),

    radius: f64,
    density: f64,
    
    color: (f32, f32, f32, f32),

}

impl Body {
    fn new(position: (f64, f64), radius: f64, density: f64) -> Body {
       Body {
           position: position,
           velocity: (0.0, 0.0),
           acceleration: (0.0, 0.0),

           radius: radius,
           density: density,

           color: (1.0, 1.0, 1.0, 1.0),

       }
    }
}

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    body_count: i32,
    current_cursor_position: (f64, f64),
    bodies: Vec<Body>
}

impl App {
     fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Color Constants  
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // Create the square
        let square = rectangle::square(0.0, 0.0, 50.0);
        let circle = rectangle::square(0.0, 0.0, 20.0);
        let rotation = self.rotation;
        let body_count = self.body_count;
        let (x, y) = ((args.width/2) as f64, (args.height/2) as f64);
        
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the scene after all bodies are drawn
            clear(WHITE, gl);
        });

        for b in &self.bodies {
            // Iterate through all the bodies

            self.gl.draw(args.viewport(), |c, gl| {
                let circle = rectangle::square(b.position.0, b.position.1, b.radius);

                let transform = c.transform.trans(x, y).rot_rad(rotation).trans(-25.0,-25.0);
                
//                rectangle(BLACK, square, transform, gl);
                ellipse(BLACK, circle, c.transform, gl);                          

               // for b in self.bodies {
    //                let circle = rectangle::square(b.
                 //   ellipse(WHITE, circle, c.transform, gl); 
             //   }
            });
         } 
        }
       
     
    
    fn update(&mut self, args: &UpdateArgs){
        use std::num::*;
        use std::f64::*;
        // Insert gravity code here
        // Force = G * m1 * m2 / r^2
        // ma = G * m1 * m2 / r^2
        // a = G * m1 * m2 / r^2 / m1
        // m = pi * r^2 * density
        const G: f64  = 6.67384 * 1e-11;
        for a in &mut self.bodies {

            let r1 = a.radius as f64;
            let area = consts::PI * (r1 * r1);
            let m1  = area * a.density;

            for b in &self.bodies.clone() {

                let r2 = b.radius as f64;
                let area = consts::PI * (r2 * r2);
                let m2  = area * a.density;


                let displacement = (b.position.0 - a.position.0, b.position.1 - a.position.1);
                let distance = (displacement.0.powf(2.0) + displacement.1.powf(2.0)).powf(0.5);
                


            
                let accel_magnitude = G * m2 / distance / distance;

                println!("AM:{}", accel_magnitude);


            }
            



        }



    }

    fn handleInput (&mut self, i:Input ) {
        
        match i {
            Input::Release(Mouse(left)) => { 
                println!("Button Released");
                println!("{:?}",i.mouse_cursor_args());
                

                println!("{:?}", self.current_cursor_position);
                
                // Adds a new body to the stage

                let b: Body = Body::new(self.current_cursor_position, 10.0, 10.0);
                self.bodies.push(b);

            },

            Input::Move(MouseCursor(x, y)) => {
               // self.current_cursor_position = (x - windowX/2.0, y - windowY/2.0);
                self.current_cursor_position = (x, y);
            }

            _=> {}
            


        }

    }

}

fn main() {

    setup_graphics();
    println!("Hello, world!");
}


fn setup_graphics() {
    let opengl = OpenGL::_3_2;

    // Create a Glutin window.

    let window = Window::new(
        opengl,
        WindowSettings::new(
            "N-body Gravity Simulation by Wilson Zhao",
            [800,800]
        )
        .exit_on_esc(true)
    );
    
    // Create a new app and run it
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        body_count: 1,
        current_cursor_position: (0.0,0.0),
        bodies: Vec::new(),

    };

    for e in window.events() {
        match e {
            
            Event::Input(i) => {
                app.handleInput(i.clone());

            }

            Event::Render(_) => {
                if let Some(r) = e.render_args() {
                    app.render(&r);
                }
            }

            Event::Update(_) => {
                if let Some(u) = e.update_args() {
                    app.update(&u);
                }
            }
            _ => {}
        } 
    }
}
    
