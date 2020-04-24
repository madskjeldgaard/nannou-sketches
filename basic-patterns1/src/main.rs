use nannou::prelude::*;
use nannou_osc as osc;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    port: u16,
    receiver: osc::Receiver,
    event: OscEvent,
}

fn model(app: &App) -> Model {
    let port = 1212;

    // Bind an `osc::Receiver` to a port.
    let receiver = osc::receiver(port).unwrap();

    //
    let event = OscEvent {
        amp: 0.25,
        pan: 0.0,
        dur: 1.0,
        degree: 0,
        freq: 444.0,
    };

    // Create a simple UI to display received messages.
    Model {
        port,
        receiver,
        event,
    }
}

#[derive(Debug)]
struct OscEvent {
    dur: f32,
    degree: i32,
    amp: f32,
    freq: f32,
    pan: f32,
}

fn match_sc_addrs(event: &mut OscEvent, messages: Vec<osc::Message>) {
    for m in messages {
        for args in m.args.unwrap() {
            match (&m.addr[..], args) {
                ("/freq", osc::Type::Float(val)) => event.freq = val,
                ("/pan", osc::Type::Float(val)) => event.pan = val,
                ("/dur", osc::Type::Float(val)) => event.dur = val,
                ("/degree", osc::Type::Int(val)) => event.degree = val,
                ("/amp", osc::Type::Float(val)) => event.amp = val,
                _ => (),
            }
        }
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut news: bool = false;

    for (packet, addr) in model.receiver.try_iter() {
        let messages = packet.into_msgs();
        match_sc_addrs(&mut model.event, messages);

        news = true;
    }

    if news {
        println!("New event: {:?}", model.event);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    let normed_freq = map_range(model.event.freq, 80.0, 2500.0, 0.0, 1.0);
    let radius = model.event.amp * win.h();

    let x = map_range(model.event.pan, -1.0, 1.0, win.left(), win.right());
    let y = map_range(normed_freq, 0.0, 1.0, win.bottom(), win.top());

    let p = pt2(x, y);

    draw.background().color(BLACK);
    draw.ellipse().xy(p).color(STEELBLUE).radius(radius);

    draw.to_frame(app, &frame).unwrap();
}
