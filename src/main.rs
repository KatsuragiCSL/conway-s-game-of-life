use nannou::prelude::*;
use std::{thread, time::Duration};
use std::cmp;
use x11::xlib::{XOpenDisplay, XDefaultScreen, XDisplayWidth, XDisplayHeight};

mod world;
mod cell;

static NROWS: usize = 100;
static NCOLS: usize = 100;

fn main() {
    nannou::app(model)
		.update(update)
        .run();
}


fn model(_app: &App) -> world::World {
	// determine screen resolution and hence cell display size
	let d = unsafe { XOpenDisplay(std::ptr::null()) };
	let s = unsafe { XDefaultScreen(d) };
	let w = unsafe { XDisplayWidth(d, s) };
	let h = unsafe { XDisplayHeight(d, s) };
	let SCREEN_SIZE_RECT = cmp::min(w, h);
	let CELL_SIZE = (SCREEN_SIZE_RECT / cmp::max(NROWS, NCOLS) as i32) as f32;
	let _window = _app
				.new_window()
				.title("Conway's Game Of Life")
				.size(SCREEN_SIZE_RECT as u32, SCREEN_SIZE_RECT as u32) // rectangle window
				.view(view)
				.build()
				.unwrap();
    world::World::new(NROWS, NCOLS)
}

fn update(_app: &App, _world: &mut world::World, _update: Update) {
	//thread::sleep(Duration::from_millis(100));
	_world.mutate();
}

fn view(_app: &App, _world: &world::World, frame: Frame){
	// divide pixels by dimension of the world
	let CELL_SIZE = (_app.window_rect().w() / cmp::max(NROWS, NCOLS) as f32);
	let draw = _app.draw();
	draw.background().color(WHITE);
	// black if the cell is alive
	for i in 0..NROWS {
		for j in 0..NCOLS {
			let cell =  _world.get_cell(i, j).unwrap();
			let color = if cell.get_liveness() {BLACK} else {WHITE};
			draw.rect() // draw a rectangle
				.w(CELL_SIZE)
				.h(CELL_SIZE)
				.x(CELL_SIZE * j as f32 + (CELL_SIZE)/ 2.0 - _app.window_rect().w()/ 2.0)
				.y(CELL_SIZE * i as f32 + (CELL_SIZE)/ 2.0 - _app.window_rect().h()/ 2.0)
				.color(color);
		}
	}
				
	draw.to_frame(_app, &frame).unwrap();
}
