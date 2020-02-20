extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateArgs, UpdateEvent}; // RenderArgs,
use piston::window::{WindowSettings};
use rand;
use graphics::*;

const WINDOW_X: f64 = 639.0;
const WINDOW_Y: f64 = 639.0;
const MAX_X: u32 = 40;
const MAX_Y: u32 = 40;

#[derive(Clone)]
struct Boxes {
    x: u32,
    y: u32,
    color: [f32; 4],
    max_x: u32,
    max_y: u32,
    last_box: u32,
}

impl Boxes {
    fn new(max_x: u32, max_y: u32) -> Boxes {
        Boxes{
            x: rand::random::<u32>() % max_x,
            y: rand::random::<u32>() % max_y,
            color: [rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0],
            max_x,
            max_y,
            last_box: 4,
        }
    }

    fn update(&mut self) {
        loop {
            match rand::random::<u32>() % 4 {
                0 => { if self.x < self.max_x - 1 && self.last_box != 0 { self.x += 1; self.last_box = 1; return; } }
                1 => { if self.x > 0 && self.last_box != 1 { self.x -= 1; self.last_box = 0; return; } }
                2 => { if self.y < self.max_y - 1 && self.last_box != 2 { self.y += 1; self.last_box = 3; return; } }
                3 => { if self.y > 0 && self.last_box != 3 { self.y -= 1; self.last_box = 2; return; } }
                _ => {}
            }
        }
    }
}

struct Grid {
    pos_x: f64,
    pos_y: f64,
    line_width: f64,
    grid_width: f64,
    grid_height: f64,
    grid_count_horizontal: u32,
    grid_count_vertical: u32,
    //color: [f32; 4],
    boxes: [Boxes; 5],
    past_boxes: std::collections::HashMap<(u32, u32), Boxes>,
}

impl Grid {
    fn box_width(&mut self) -> f64 {
        (self.grid_width - (self.line_width * (self.grid_count_horizontal - 1) as f64)) / self.grid_count_horizontal as f64
    }
    fn box_height(&mut self) -> f64 {
        (self.grid_height - (self.line_width * (self.grid_count_vertical - 1) as f64)) / self.grid_count_vertical as f64
    }
    fn update(&mut self) {
        for i in 0..self.boxes.len() {
            self.boxes[i].update();

            self.past_boxes.insert((self.boxes[i].x, self.boxes[i].y), self.boxes[i].clone());
        }
    }
}

struct Walker {
    grid: Grid,
    dt_aggr: f64,
}

impl Walker {
    fn new() -> Walker {
        let s = Walker {
            grid: Grid{
                pos_x: 0.0,
                pos_y: 0.0,
                line_width: 1.0,
                grid_width: WINDOW_X,
                grid_height: WINDOW_Y,
                grid_count_horizontal: MAX_X,
                grid_count_vertical: MAX_Y,
                //color: [1.0, 1.0, 1.0, 0.2],
                boxes: [
                    Boxes::new(MAX_X, MAX_Y),
                    Boxes::new(MAX_X, MAX_Y),
                    Boxes::new(MAX_X, MAX_Y),
                    Boxes::new(MAX_X, MAX_Y),
                    Boxes::new(MAX_X, MAX_Y),
                ],
                past_boxes: std::collections::HashMap::new(),
            },
            dt_aggr: 0.0,
        };

        s
    }

    fn render(&mut self, c: Context, g: &mut GlGraphics) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let bwidth = self.grid.box_width();
        let bheight = self.grid.box_height();
        let boxes = self.grid.boxes.clone();
        let pos_x = self.grid.pos_x.clone();
        let pos_y = self.grid.pos_y.clone();
        let line_width = self.grid.line_width.clone();
        let past_boxes = self.grid.past_boxes.values().clone();

        // Clear the screen.
        clear(BLACK, g);
        let transform = c.transform;

        for b in boxes.iter() {
            let x_coord = if b.x == 0 {pos_x} else {pos_x + ((b.x as f64) * bwidth) + ((b.x) as f64) * line_width};
            let y_coord = if b.y == 0 {pos_y} else {pos_y + b.y as f64 * bheight + (b.y) as f64 * line_width};

            let rect = rectangle::rectangle_by_corners(
                (x_coord - line_width) as f64,
                (y_coord - line_width) as f64,
                (x_coord + (bwidth + 2.0 * line_width)) as f64,
                (y_coord + (bheight + 2.0 * line_width)) as f64);
            rectangle(RED, rect, transform, g);
            let rect = rectangle::rectangle_by_corners(
                (x_coord) as f64,
                (y_coord) as f64,
                (x_coord + bwidth) as f64,
                (y_coord + bheight) as f64);
            rectangle(b.color, rect, transform, g);
        }

        for  b in past_boxes {
            let x_coord = if b.x == 0 {pos_x} else {pos_x + ((b.x as f64) * bwidth) + ((b.x) as f64) * line_width};
            let y_coord = if b.y == 0 {pos_y} else {pos_y + b.y as f64 * bheight + (b.y) as f64 * line_width};
            let rect = rectangle::rectangle_by_corners(
                (x_coord) as f64,
                (y_coord) as f64,
                (x_coord + bwidth) as f64,
                (y_coord + bheight) as f64);
            rectangle(b.color, rect, transform, g);
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.dt_aggr += args.dt;
        if self.dt_aggr >= (1.0 / 10.0) {
            self.grid.update();
            self.dt_aggr = 0.0;
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("GridWalker", [WINDOW_X, WINDOW_Y])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut walker = Walker::new();
    let gl = &mut GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                walker.render(c, gl);
            });
        }

        if let Some(args) = e.update_args() {
            walker.update(&args);
        }
    }
}
