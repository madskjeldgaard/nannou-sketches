use nannou_osc as osc;

#[derive(Debug)]
pub struct NoteEvent {
    pub stream_name: String,

    pub dur: f32,
    pub legato: f32,

    pub degree: i32,
    pub midinote: i32,
    pub octave: f32,

    pub amp: f32,
    pub db: f32,

    pub freq: f32,
    pub pan: f32,
}

// Pattern matching on osc addresses emitted by SuperCollider
impl NoteEvent {
    pub fn new() -> NoteEvent {
        NoteEvent {
            // This is a custom value used to be able to seperate incoming event streams from SC
            stream_name: "stream1".to_string(),

            dur: 1.0,
            legato: 0.8,

            degree: 0,
            midinote: 67,
            octave: 5.0,

            amp: 1.0,
            db: -20.0,

            freq: 444.0,
            pan: 0.0,
        }
    }

    pub fn name(mut self, name: String) -> NoteEvent {
        self.stream_name = name;

        self
    }

    pub fn parse_messages(&mut self, messages: Vec<osc::Message>) {
        for m in messages {
            match m.args {
                Some(args) => {
                    for arg in args {
                        match (&m.addr[..], arg) {
                            ("/stream_name", osc::Type::String(val)) => self.stream_name = val,

                            ("/dur", osc::Type::Float(val)) => self.dur = val,
                            ("/legato", osc::Type::Float(val)) => self.legato = val,

                            ("/degree", osc::Type::Int(val)) => self.degree = val,
                            ("/midinote", osc::Type::Int(val)) => self.midinote = val,
                            ("/octave", osc::Type::Float(val)) => self.octave = val,

                            ("/amp", osc::Type::Float(val)) => self.amp = val,
                            ("/db", osc::Type::Float(val)) => self.amp = val,

                            ("/freq", osc::Type::Float(val)) => self.freq = val,
                            ("/pan", osc::Type::Float(val)) => self.pan = val,

                            unrecognized => println!(
                                "Received unrecognized osc addr: {:?}. Ignoring it.",
                                unrecognized
                            ),
                        }
                    }
                }
                None => (),
            }
        }
    }
}

// This is repurposed from the source code of SuperCollider
pub fn explin(val: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    let e = std::f32::EPSILON;
    (val / in_min).log(e) / (in_max / in_min).log(e) * (out_max - out_min) + out_min
}
