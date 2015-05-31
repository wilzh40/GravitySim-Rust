// Imports
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    body_count: i32,
}

impl App {
     fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let body_count = self.body_count;
        let (x, y) = ((args.width/2) as f64, (args.height/2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y).rot_rad(rotation).trans(-25.0,-25.0);

            rectangle(RED, square, transform, gl);
        });
     }
    fn update(&mut self, args: &UpdateArgs){
        self.rotation += 2.0 * args.dt;

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
            "spinning-square",
            [200,200]
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
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
    

}
    
