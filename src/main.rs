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
use piston::input::Motion::MouseRelative;
pub struct App {
    gl: GlGraphics,
    rotation: f64,
    body_count: i32,
}

impl App {
     fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Color Constants  
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // Create the square
        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let body_count = self.body_count;
        let (x, y) = ((args.width/2) as f64, (args.height/2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y).rot_rad(rotation).trans(-25.0,-25.0);

            rectangle(WHITE, square, transform, gl);
        });
     }
    fn update(&mut self, args: &UpdateArgs){

        // Insert gravity code here
        self.rotation += 2.0 * args.dt;

    }

    fn handleInput (&mut self, i:Input ) {
        match i {
            _=> { println!("input");}


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
            [500,500]
        )
        .exit_on_esc(true)
    );
    
    // Create a new app and run it
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        body_count: 1

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
    
