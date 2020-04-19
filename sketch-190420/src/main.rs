/*
 *
 * This sketch divides the window into a certain amount of squares and then draws those squares
 *
 */
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    screen_rects: Vec<Rect>,
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
    Model {
        screen_rects: vec![],
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.screen_rects = divide_screen(app, 8, 8);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let time = app.time;

    for (i, rect) in model.screen_rects.iter().enumerate() {
        let mut c = map_range(i, 0, model.screen_rects.len(), 0.0, 1.0);

        c = (c * time).sin() + 1.0 / 2.0;

        draw.rect()
            .x(rect.x() + rect.w() * 0.5)
            .y(rect.y() + rect.h() * 0.5)
            .w(rect.w())
            .h(rect.h())
            .rgba(c, 1.0 - c, c, c);
    }

    draw.to_frame(app, &frame).unwrap();
}
