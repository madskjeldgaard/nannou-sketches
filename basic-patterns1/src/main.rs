use nannou::prelude::*;
use nannou_osc as osc;

mod noteevent;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    port: u16,
    receiver: osc::Receiver,
    stream1: noteevent::NoteEvent,
    stream2: noteevent::NoteEvent,
    stream3: noteevent::NoteEvent,
}

fn model(_app: &App) -> Model {
    let port = 1211;

    // Bind an `osc::Receiver` to a port.
    let receiver = osc::receiver(port).unwrap();

    // Setup two event streams, one on "/stream1"
    let stream1 = noteevent::NoteEvent::new().name("stream1".to_string());

    // Setup two event streams, one on "/stream2"
    let stream2 = noteevent::NoteEvent::new().name("stream2".to_string());

    // Setup two event streams, one on "/stream2"
    let stream3 = noteevent::NoteEvent::new().name("stream3".to_string());

    Model {
        port,
        receiver,
        stream1,
        stream2,
        stream3,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for (packet, _addr) in model.receiver.try_iter() {
        // Temporary event
        let mut tmp_event = noteevent::NoteEvent::new();

        // Unfold packet into messages
        let messages = packet.into_msgs();

        // Parse the incoming event's osc messages and add them to the local event
        tmp_event.parse_messages(messages);

        // Which stream is it?
        match &tmp_event.stream_name[..] {
            "stream1" => {
                println!("New event on stream1: {:?}", &tmp_event);
                model.stream1 = tmp_event;
            }
            "stream2" => {
                println!("New event on stream2: {:?}", &tmp_event);
                model.stream2 = tmp_event;
            }
            "stream3" => {
                println!("New event on stream2: {:?}", &tmp_event);
                model.stream3 = tmp_event;
            }
            _ => (),
        };
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    // Stream 1
    let r1 = model.stream1.amp * (0.25 * win.h());
    let x1 = map_range(model.stream1.pan, -1.0, 1.0, win.left(), win.right());
    let y1 = noteevent::explin(model.stream1.freq, 20.0, 20000.0, win.bottom(), win.top());
    let p1 = pt2(x1, y1);
    draw.ellipse().xy(p1).color(RED).radius(r1);

    // Stream 2
    let x2 = map_range(model.stream2.pan, -1.0, 1.0, win.left(), win.right());
    let y2 = noteevent::explin(model.stream2.freq, 20.0, 20000.0, win.bottom(), win.top());
    let r2 = model.stream2.amp * (0.25 * win.h());
    let p2 = pt2(x2, y2);
    draw.ellipse().xy(p2).color(BLUE).radius(r2);

    // Stream 3
    let x3 = map_range(
        model.stream3.pan + app.time.sin(),
        -2.0,
        2.0,
        win.left(),
        win.right(),
    );
    let y3 = noteevent::explin(model.stream3.freq, 20.0, 20000.0, win.bottom(), win.top());
    let r3 = model.stream3.amp * (0.75 * win.h());
    let p3 = pt2(x3, y3);
    draw.rect()
        .xy(p3)
        .color(GREEN)
        .w_h(r3, r3)
        .z_turns((app.time * model.stream3.amp).sin());

    let col = (
        model.stream1.freq.sin() as f64,
        noteevent::explin(model.stream2.freq, 20.0, 20000.0, 0.0, 1.0) as f64,
        noteevent::explin(model.stream3.freq, 20.0, 20000.0, 0.0, 1.0) as f64,
    );

    let bgcol = Rgb::from(col);

    draw.background().color(bgcol);

    draw.to_frame(app, &frame).unwrap();
}
