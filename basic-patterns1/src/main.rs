use nannou::prelude::*;
use nannou_osc as osc;

mod oscevent;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    port: u16,
    receiver: osc::Receiver,
    event: oscevent::OscEvent,
}

fn explin(val: f32, inMin: f32, inMax: f32, outMin: f32, outMax: f32) -> f32 {
    let e = std::f32::EPSILON;
    (val / inMin).log(e) / (inMax / inMin).log(e) * (outMax - outMin) + outMin
}

fn model(app: &App) -> Model {
    let port = 1212;

    // Bind an `osc::Receiver` to a port.
    let receiver = osc::receiver(port).unwrap();

    //
    let event = oscevent::OscEvent::new();

    // Create a simple UI to display received messages.
    Model {
        port,
        receiver,
        event,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut news: bool = false;

    for (packet, addr) in model.receiver.try_iter() {
        let messages = packet.into_msgs();
        model.event.match_sc_addrs(messages);

        news = true;
    }

    if news {
        println!("New event: {:?}", model.event);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    let normed_freq = explin(model.event.freq, 20.0, 20000.0, 0.0, 1.0);
    let radius = model.event.amp * win.h();

    let x = map_range(model.event.pan, -1.0, 1.0, win.left(), win.right());
    let y = map_range(normed_freq, 0.0, 1.0, win.bottom(), win.top());
    let p = pt2(x, y);
    let normedmidi = (model.event.midinote as f32 * 2.0 / 128.0) as f32;
    let c = Rgba::new(normedmidi, 0.5, 0.8, normedmidi);

    draw.background().color(BLACK);
    draw.ellipse().xy(p).color(c).radius(radius);

    draw.to_frame(app, &frame).unwrap();
}
