use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn random_point(rect: &Rect) -> Point2 {
    let x = random_range(rect.left(), rect.right());
    let y = random_range(rect.top(), rect.bottom());

    pt2(x, y)
}

fn trajectory(start: Point2, end: Point2, steps: i32) -> Vec<Point2> {
    let mut vec = vec![];

    for step in 0..steps {
        let thispoint = start.lerp(end, step as f32 / steps as f32);
        vec.push(thispoint);
    }

    vec
}

fn model(app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(STEELBLUE);

    let w = app.window_rect();
    let mut v1 = w.top_left();
    let mut v2 = w.bottom_right();
    let mut v3 = w.top_right();
    let mut v4 = w.bottom_left();

    let perlin = Perlin::new();
    let val = perlin.get([app.time as f64, 1.0 - app.time as f64]) as f32;
    let freq = val * 0.25; //noisegen.get([app.time, app.time]) as f32;
    let normsin = map_range((freq * app.time).sin(), -1.0, 1.0, 0.0, 1.0);

    // Rotate vectors
    v1 *= Vector2::new(0.0, (freq * 1.0928 * app.time).sin());
    v2 *= Vector2::new((freq * 1.0951 * app.time).cos(), 0.0);
    v3 *= Vector2::new(0.0, (freq * 2.01 * app.time).cos());
    v4 *= Vector2::new((freq * 1.952 * app.time).sin(), 0.0);

    // Create trajectories (of Point's) between two vectors
    for p in trajectory(v1, v2.lerp(v1, normsin), 10) {
        draw.ellipse().xy(p).color(RED);
    }

    for p in trajectory(v2, v1.lerp(v2, normsin), 10) {
        draw.ellipse().xy(p).color(PLUM);
    }

    for p in trajectory(v3, v4.lerp(v3, normsin), 10) {
        draw.ellipse().xy(p).color(BLUE);
    }

    for p in trajectory(v4, v3.lerp(v4, normsin), 10) {
        draw.ellipse().xy(p).color(YELLOW);
    }

    draw.to_frame(&app, &frame).unwrap();
}
