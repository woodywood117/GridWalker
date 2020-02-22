use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;
use ggez::event::KeyMods;
use ggez::input::keyboard::KeyCode;

use rand;
use std::time::{Duration, Instant};

const WINDOW_X: f32 = 500.0;
const WINDOW_Y: f32 = 500.0;
const MAX_X: u32 = 40;
const MAX_Y: u32 = 40;
const UPDATES_PER_SECOND: f32 = 20.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

#[derive(Clone)]
struct Walker {
    x: u32,
    y: u32,
    color: [f32; 4],
    max_x: u32,
    max_y: u32,
    last_box: u32,
}

impl Walker {
    fn new(max_x: u32, max_y: u32) -> Walker {
        Walker {
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
    pos_x: f32,
    pos_y: f32,
    line_width: f32,
    grid_width: f32,
    grid_height: f32,
    grid_count_horizontal: u32,
    grid_count_vertical: u32,
    boxes: Vec<Walker>,
    past_boxes: std::collections::HashMap<(u32, u32), Walker>,
}

impl Grid {
    fn box_width(&mut self) -> f32 {
        (self.grid_width - (self.line_width * (self.grid_count_horizontal - 1) as f32)) / self.grid_count_horizontal as f32
    }
    fn box_height(&mut self) -> f32 {
        (self.grid_height - (self.line_width * (self.grid_count_vertical - 1) as f32)) / self.grid_count_vertical as f32
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
    last_update: std::time::Instant,
    menu_open: bool,
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
            last_update: Instant::now(),
            menu_open: false,
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

    fn toggle_menu(&mut self) {
        self.menu_open = !self.menu_open;
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) && !self.menu_open{
            self.grid.update();
            self.last_update = Instant::now();
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics:: clear(ctx, graphics::BLACK);

        let bwidth = self.grid.box_width();
        let bheight = self.grid.box_height();

        // draw boxes
        for b in self.grid.boxes.iter_mut() {
            let x_coord = if b.x == 0 {self.grid.pos_x} else {self.grid.pos_x + ((b.x as f32) * bwidth) + ((b.x) as f32) * self.grid.line_width};
            let y_coord = if b.y == 0 {self.grid.pos_y} else {self.grid.pos_y + b.y as f32 * bheight + (b.y) as f32 * self.grid.line_width};
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                ggez::graphics::Rect::new(
                    x_coord - self.grid.line_width,
                    y_coord - self.grid.line_width,
                    bwidth + 2.0 * self.grid.line_width,
                    bheight + 2.0 * self.grid.line_width,
                ),
                [1.0, 0.0, 0.0, 1.0].into(),
            )?;
            graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                ggez::graphics::Rect::new(x_coord, y_coord, bwidth, bheight),
                b.color.into(),
            )?;
            graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
        }

        for  b in self.grid.past_boxes.values() {
            let x_coord = if b.x == 0 {self.grid.pos_x} else {self.grid.pos_x + ((b.x as f32) * bwidth) + ((b.x) as f32) * self.grid.line_width};
            let y_coord = if b.y == 0 {self.grid.pos_y} else {self.grid.pos_y + b.y as f32 * bheight + (b.y) as f32 * self.grid.line_width};
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                ggez::graphics::Rect::new(x_coord, y_coord, bwidth, bheight),
                b.color.into(),
            )?;
            graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if key == KeyCode::E {
            self.toggle_menu();
        }
        if key == KeyCode::Return {
            self.reset_grid();
        }
        if key == KeyCode::Add || key == KeyCode::Equals {
            self.inc_grid_count();
        }
        if key == KeyCode::Subtract || key == KeyCode::Minus {
            self.dec_grid_count();
        }
        if key == KeyCode::Escape {
            event::quit(ctx);
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("GridWalker", "woodywood117")
        .window_setup(
            ggez::conf::WindowSetup {
                title: "GridWalker".to_owned(),
                samples: ggez::conf::NumSamples::Four,
                vsync: true,
                icon: "".to_owned(),
                srgb: true,
            })
        .window_mode(ggez::conf::WindowMode {
            width: WINDOW_X,
            height: WINDOW_Y,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            min_width: WINDOW_X,
            max_width: WINDOW_X,
            min_height: WINDOW_Y,
            max_height: WINDOW_Y,
            resizable: false,
        });
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut State::new();
    event::run(ctx, event_loop, state)
}
