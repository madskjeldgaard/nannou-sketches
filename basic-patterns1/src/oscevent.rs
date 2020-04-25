use nannou_osc as osc;

#[derive(Debug)]
pub struct OscEvent {
    pub dur: f32,
    pub degree: i32,
    pub midinote: i32,
    pub amp: f32,
    pub freq: f32,
    pub pan: f32,
}

// Make new event containing defaults
impl OscEvent {
    pub fn new() -> OscEvent {
        OscEvent {
            freq: 444.0,
            pan: 0.0,
            dur: 1.0,
            degree: 0,
            midinote: 67,
            amp: 1.0,
        }
    }
}

// Pattern matching on osc addresses emitted by SuperCollider
// TODO: Make this work with bundles
impl OscEvent {
    pub fn match_sc_addrs(&mut self, messages: Vec<osc::Message>) {
        for m in messages {
            for args in m.args.unwrap() {
                match (&m.addr[..], args) {
                    ("/freq", osc::Type::Float(val)) => self.freq = val,
                    ("/pan", osc::Type::Float(val)) => self.pan = val,
                    ("/dur", osc::Type::Float(val)) => self.dur = val,
                    ("/degree", osc::Type::Int(val)) => self.degree = val,
                    ("/midinote", osc::Type::Int(val)) => self.midinote = val,
                    ("/amp", osc::Type::Float(val)) => self.amp = val,
                    _ => (),
                }
            }
        }
    }
}
