use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    winrects: Vec<Rect>,
}

fn divide_screen(app: &App, divide_by_x: u32, divide_by_y: u32) -> Vec<Rect> {
    let mut vec = vec![];
    let main_rect = app.window_rect();

    let rect_width = main_rect.w() / divide_by_x as f32;
    let rect_height = main_rect.h() / divide_by_y as f32;

    for x_num in 0..divide_by_x {
        for y_num in 0..divide_by_y {
            let x = map_range(x_num, 0, divide_by_x, main_rect.left(), main_rect.right());
            let y = map_range(y_num, 0, divide_by_y, main_rect.bottom(), main_rect.top());

            let this_rect = Rect::from_x_y_w_h(x, y, rect_width, rect_height);

            vec.push(this_rect);
        }
    }

    vec
}

fn model(_app: &App) -> Model {
    Model { winrects: vec![] }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.winrects = divide_screen(app, 12, 12);
}

fn point_in_rect(p: &Point2, rect: &Rect) -> bool {
    if p.x > rect.left() && p.x < rect.right() && p.y > rect.bottom() && p.y < rect.top() {
        true
    } else {
        false
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(DARKORANGE);

    let t = app.time;

    let r = 100.0;
    let w = app.window_rect();

    // Use the radius as an offset to make sure the circle is within the window of the app
    let modspeed = 2.0;
    let x = map_range((t * modspeed).sin(), -1.0, 1.0, w.left() + r, w.right() - r);
    let y = map_range(
        (t * (modspeed / 2.0)).sin(),
        1.0,
        -1.0,
        w.bottom() + r,
        w.top() - r,
    );

    let p = Point2::from((x, y));

    draw.ellipse().color(STEELBLUE).radius(r).xy(p);

    for rect in model.winrects.iter() {
        if point_in_rect(&p, &rect) {
            let norm_x = rect.x() + rect.w() * 0.5;
            let norm_y = rect.y() + rect.w() * 0.5;
            draw.rect().x(norm_x).y(norm_y).color(GREEN);
        }
        // Check if the point is within one of the screen's rectangles
    }

    draw.to_frame(app, &frame).unwrap();
}
