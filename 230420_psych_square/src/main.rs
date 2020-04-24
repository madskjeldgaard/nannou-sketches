use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn trajectory(start: Point2, end: Point2, steps: i32) -> Vec<Point2> {
    let mut vec = vec![];

    for step in 0..steps {
        let thispoint = start.lerp(end, (1 + step) as f32 / steps as f32);
        vec.push(thispoint);
    }

    vec
}
fn draw_circles(
    points: Vec<Point2>,
    color: Rgb,
    time: f32,
    foreign_point: Point2,
    draw: &nannou::app::Draw,
) {
    let n = Perlin::new();

    for (i, p) in points.iter().enumerate() {
        let thisnoise = n.get([(i as f64 + time as f64), (i as f64 - time as f64)]);
        let thisp = pt2(p.x + thisnoise as f32 * 10.0, p.y);
        draw.ellipse()
            .xy(thisp)
            .color(color)
            .radius(thisp.distance(foreign_point) / 4.0 + 10.0);
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    // let win = app.window_rect();

    let win = Rect::from_wh(app.window_rect().wh());

    draw.background().color(BLACK);

    let perlin = Perlin::new();
    let big_circle_point = pt2(
        map_range(
            perlin.get([0.95 * app.time as f64, 1.75 * app.time as f64]),
            0.0,
            1.0,
            win.top(),
            win.bottom(),
        ),
        map_range(
            perlin.get([app.time as f64, 0.5 * app.time as f64]),
            0.0,
            1.0,
            win.left(),
            win.right(),
        ),
    );

    draw.ellipse().color(PINK).xy(big_circle_point);

    // Left side
    draw_circles(
        trajectory(win.top_left(), win.bottom_left(), 10),
        Rgb::new(random_f32(), random_f32(), random_f32()),
        app.time,
        big_circle_point,
        &draw,
    );

    // Right side
    draw_circles(
        trajectory(win.top_right(), win.bottom_right(), 10),
        Rgb::new(random_f32(), random_f32(), random_f32()),
        app.time,
        big_circle_point,
        &draw,
    );

    // Bottom side
    draw_circles(
        trajectory(win.bottom_left(), win.bottom_right(), 10),
        Rgb::new(random_f32(), random_f32(), random_f32()),
        app.time,
        big_circle_point,
        &draw,
    );

    // Top side
    draw_circles(
        trajectory(win.top_left(), win.top_right(), 10),
        Rgb::new(random_f32(), random_f32(), random_f32()),
        app.time,
        big_circle_point,
        &draw,
    );

    draw.to_frame(app, &frame).unwrap();
}
