use nannou::prelude::*;
mod world;
mod cell;

fn main() {
    nannou::app(model)
		.loop_mode(LoopMode::Wait)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(PURPLE);
}
