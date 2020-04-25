use nannou::prelude::*;
use nannou_osc as osc;

mod oscevent;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    port: u16,
    receiver: osc::Receiver,
    stream1: oscevent::OscEvent,
    stream2: oscevent::OscEvent,
}

fn model(_app: &App) -> Model {
    let port = 1211;

    // Bind an `osc::Receiver` to a port.
    let receiver = osc::receiver(port).unwrap();

    //
    let stream1 = oscevent::OscEvent::new();
    let stream2 = oscevent::OscEvent::new().name("stream2".to_string());

    // Create a simple UI to display received messages.
    Model {
        port,
        receiver,
        stream1,
        stream2,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut news: bool = false;

    for (packet, _addr) in model.receiver.try_iter() {
        let mut sc_event = oscevent::OscEvent::new();

        // println!("{:?}", packet);
        let messages = packet.into_msgs();

        sc_event.parse_messages(messages);

        match &sc_event.stream_name[..] {
            "stream1" => {
                model.stream1 = sc_event;
            }
            "stream2" => {
                model.stream2 = sc_event;
            }
            _ => (),
        };

        news = true;
    }

    if news {
        println!("Stream1 : {:?}", &model.stream1);
        println!("Stream2 : {:?}", &model.stream2);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    // Stream 1
    let normed_freq = oscevent::explin(model.stream1.freq, 20.0, 20000.0, 0.0, 1.0);
    let radius = model.stream1.amp * (0.5 * win.h());
    let x = map_range(model.stream1.pan, -1.0, 1.0, win.left(), win.right());
    let y = map_range(normed_freq, 0.0, 1.0, win.bottom(), win.top());
    let p = pt2(x, y);
    draw.ellipse().xy(p).color(RED).radius(radius);

    // Stream 2
    let x = map_range(model.stream2.pan, -1.0, 1.0, win.left(), win.right());
    let y = oscevent::explin(model.stream2.freq, 20.0, 20000.0, win.bottom(), win.top());
    let radius2 = model.stream2.amp * (0.5 * win.h());
    let p = pt2(x, y);
    draw.ellipse().xy(p).color(BLUE).radius(radius2);

    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
