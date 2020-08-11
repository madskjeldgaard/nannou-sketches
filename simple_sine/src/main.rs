use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::f64::consts::PI;

fn main() {
    nannou::app(model).run();
}

struct Model {
    stream: audio::Stream<Audio>,
}

struct Audio {
    phase: f64,
    freq: f64,
    volume: f32,
}

fn model(app: &App) -> Model {
    // Create a window to receive key pressed events.
    app.new_window().view(view).build().unwrap();
    // Initialise the audio API so we can spawn an audio stream.
    let audio_host = audio::Host::new();
    // Initialise the state that we want to live on the audio thread.
    let model = Audio {
        phase: 0.0,
        freq: 440.0,
        volume: 0.5,
    };
    let stream = audio_host
        .new_output_stream(model)
        .render(audio)
        .build()
        .unwrap();
    Model { stream }
}

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play a simple sine wave at the audio's current frequency in `freq`.
fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    audio.volume = 0.5;

    // play_sine(audio, buffer, freq, volume);
    play_sine(audio, buffer, audio.freq, audio.volume);
}

fn play_sine(audio: &mut Audio, buffer: &mut Buffer, freq: f64, volume: f32) {
    let sample_rate = buffer.sample_rate() as f64;

    for frame in buffer.frames_mut() {
        // Create sine wave
        let sine_amp = (2.0 * PI * audio.phase).sin() as f32;

        // Progress phase according to speed of frequency
        audio.phase += freq / sample_rate;
        audio.phase %= sample_rate;

        for channel in frame {
            *channel = sine_amp * volume;
        }
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    frame.clear(PINK);
}
