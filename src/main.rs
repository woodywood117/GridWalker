use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    lifecycle::{run, Event, EventStream, Settings, Window, Key},
    Result,
};
use rand;

const WINDOW_X: f64 = 639.0;
const WINDOW_Y: f64 = 639.0;
const MAX_X: u32 = 40;
const MAX_Y: u32 = 40;

#[derive(Clone)]
struct Walker {
    x: u32,
    y: u32,
    color: (u8,u8,u8,f32),
    max_x: u32,
    max_y: u32,
    last_box: u32,
}

impl Walker {
    fn new(max_x: u32, max_y: u32) -> Walker {
        Walker {
            x: rand::random::<u32>() % max_x,
            y: rand::random::<u32>() % max_y,
            color: (rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 1.0),
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
    boxes: Vec<Walker>,
    past_boxes: std::collections::HashMap<(u32, u32), Walker>,
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

struct State {
    grid: Grid,
    num_walkers: i32,
    menu_open: bool,
    dt_inst: std::time::Instant,
}

impl State {
    fn new() -> State {
        let s = State {
            grid: Grid{
                pos_x: 0.0,
                pos_y: 0.0,
                line_width: 1.0,
                grid_width: WINDOW_X,
                grid_height: WINDOW_Y,
                grid_count_horizontal: MAX_X,
                grid_count_vertical: MAX_Y,
                boxes: vec![
                    Walker::new(MAX_X, MAX_Y),
                    Walker::new(MAX_X, MAX_Y),
                    Walker::new(MAX_X, MAX_Y),
                    Walker::new(MAX_X, MAX_Y),
                    Walker::new(MAX_X, MAX_Y),
                ],
                past_boxes: std::collections::HashMap::new(),
            },
            num_walkers: 5,
            menu_open: false,
            dt_inst: std::time::Instant::now(),
        };

        s
    }

    fn reset_grid(&mut self) {
        self.grid = Grid{
            pos_x: 0.0,
            pos_y: 0.0,
            line_width: 1.0,
            grid_width: WINDOW_X,
            grid_height: WINDOW_Y,
            grid_count_horizontal: MAX_X,
            grid_count_vertical: MAX_Y,
            boxes: vec![],
            past_boxes: std::collections::HashMap::new(),
        };
        for _ in 0..self.num_walkers {
            self.grid.boxes.push(Walker::new(MAX_X, MAX_Y))
        }
    }

    fn inc_grid_count(&mut self) {
        self.num_walkers += 1;
        self.grid.boxes.push(Walker::new(MAX_X, MAX_Y));
    }

    fn dec_grid_count(&mut self) {
        if self.num_walkers > 0 {
            self.num_walkers -= 1;
            self.grid.boxes.pop();
        }
    }

    fn render_menu(&mut self, g: &mut Graphics, w: &Window) {
        g.clear(Color::BLACK);
        g.present(&w);
    }

    fn render_grid(&mut self, g: &mut Graphics, w: &Window) {
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
        g.clear(Color::BLACK);

        for b in boxes.iter() {
            let x_coord = if b.x == 0 {pos_x} else {pos_x + ((b.x as f64) * bwidth) + ((b.x) as f64) * line_width};
            let y_coord = if b.y == 0 {pos_y} else {pos_y + b.y as f64 * bheight + (b.y) as f64 * line_width};

            // draw outer line
            let rect = Rectangle::new(Vector::new((x_coord - line_width) as f32, (y_coord - line_width) as f32),
                                      Vector::new((bwidth + 2.0 * line_width) as f32, (bheight + 2.0 * line_width) as f32));
            g.fill_rect(&rect, Color::RED);

            // draw inner box
            let rect = Rectangle::new(Vector::new(x_coord as f32, y_coord as f32),
                                      Vector::new(bwidth as f32, bheight as f32));
            g.fill_rect(&rect, Color::from_rgba(b.color.0, b.color.1, b.color.2, b.color.3));
        }

        // draw old boxes
        for  b in past_boxes {
            let x_coord = if b.x == 0 {pos_x} else {pos_x + ((b.x as f64) * bwidth) + ((b.x) as f64) * line_width};
            let y_coord = if b.y == 0 {pos_y} else {pos_y + b.y as f64 * bheight + (b.y) as f64 * line_width};
            let rect = Rectangle::new(Vector::new(x_coord as f32, y_coord as f32),
                                      Vector::new(bwidth as f32, bheight as f32));
            g.fill_rect(&rect, Color::from_rgba(b.color.0, b.color.1, b.color.2, b.color.3));
        }
        g.present(&w);
    }

    fn update(&mut self) {
        if self.dt_inst.elapsed() > std::time::Duration::from_millis(100) {
            self.grid.update();
            self.dt_inst = std::time::Instant::now();
        }
    }

    fn toggle_menu(&mut self) {
        self.menu_open = !self.menu_open;
    }
}

fn main() {
    run(
        Settings {
            size: Vector::new(WINDOW_X as f32, WINDOW_Y as f32).into(),
            title: "GridWalker",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    let mut walker = State::new();
    loop {
        if !walker.menu_open {
            walker.update();
        }

        if walker.menu_open {
            walker.render_grid(&mut gfx, &window);
        } else {
            walker.render_grid(&mut gfx, &window);
        }

        while let Some(e) = events.next_event().await {
            if let Event::KeyboardInput(key) = e {
                // println!("{:?}", key);
                if key.key() == Key::E && key.is_down() {
                    walker.toggle_menu();
                }
                if key.key() == Key::Return {
                    walker.reset_grid();
                }
                if key.key() == Key::Add || key.key() == Key::Equals {
                    walker.inc_grid_count();
                }
                if key.key() == Key::Subtract || key.key() == Key::Minus {
                    walker.dec_grid_count();
                }
                if key.key() == Key::Escape {
                   return Ok(());
                }
            }
        }
    }
}
