use nannou::prelude::*;

pub fn divide_screen(app: &App, divide_by_x: u32, divide_by_y: u32) -> Vec<Rect> {
    let mut vec = vec![];
    let main_rect = app.main_window().rect();

    let rect_width = main_rect.w() / divide_by_x as f32;
    let rect_height = main_rect.h() / divide_by_y as f32;

    for x_num in 0..divide_by_x {
        for y_num in 0..divide_by_y {
            let x = map_range(x_num, 0, divide_by_x, main_rect.left(), main_rect.right());
            let y = map_range(y_num, 0, divide_by_y, main_rect.bottom(), main_rect.top());
            let norm_x = x + (rect_width * 0.5);
            let norm_y = y + (rect_height * 0.5);

            let this_rect = Rect::from_x_y_w_h(norm_x, norm_y, rect_width, rect_height);

            vec.push(this_rect);
        }
    }

    vec
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    winrects: Vec<Rect>,
}
fn model(_app: &App) -> Model {
    Model { winrects: vec![] }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.winrects = divide_screen(app, 4, 4);
}

fn randcol(red: f32, blue: f32, green: f32) -> Rgb {
    let r = red * random_f32();
    let g = blue * random_f32();
    let b = green * random_f32();

    Rgb::from((r, g, b))
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(DARKORANGE);

    let t = app.time;

    let r = 10.0;
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

    let p = pt2(x, y);

    for &rect in model.winrects.iter() {
        if rect.contains(p) {
            let thispoint = pt2(rect.x(), rect.y());
            draw.rect()
                .xy(thispoint)
                .wh(rect.wh())
                .color(randcol(0.95, 0.95, 0.95));
        }
    }

    draw.ellipse().color(STEELBLUE).radius(r).xy(p);
    draw.to_frame(app, &frame).unwrap();
}
