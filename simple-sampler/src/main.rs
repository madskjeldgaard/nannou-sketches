use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;

fn main() {
    nannou::app(model).run();
}

struct Model {
    stream: audio::Stream<Audio>,
}

struct Audio {
    sound: audrey::read::BufFileReader,
}

fn model(app: &App) -> Model {
    // Create a window to receive key pressed events.
    app.new_window().view(view).build().unwrap();

    // Initialise the audio host so we can spawn an audio stream.
    let audio_host = audio::Host::new();

    // Load sound file
    let file_path = "harmonica1.wav".to_string();
    let sound = audrey::open(file_path).expect("failed to load sound");

    // Initialise the state that we want to live on the audio thread.
    let model = Audio { sound };

    let stream = audio_host
        .new_output_stream(model)
        .render(audio)
        .build()
        .unwrap();

    Model { stream }
}

fn play_file(buffer: &mut Buffer, sound: &mut audrey::read::BufFileReader) {
    let buffer_len_frames = buffer.len_frames();

    // How far into the sound file are we?
    let mut frame_count = 0;

    // First, get all frames from the sound loaded
    // TODO: Why filter_map ?
    let file_frames = sound.frames::<[f32; 2]>().filter_map(Result::ok);

    // Then iterate over each frame of the sound file and write it to the audio buffer
    for (frame, file_frame) in buffer.frames_mut().zip(file_frames) {
        // Read each sample of the frame
        for (sample, file_sample) in frame.iter_mut().zip(&file_frame) {
            // Move file's sample into audio buffer's sample
            *sample += *file_sample;
        }

        frame_count += 1;

        // If the sound yielded less samples than are in the buffer, it must have ended.
        // if frame_count < len_frames && looping {
        //     // Ended
        // }
    }
}

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play the audio file.
fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    play_file(buffer, &mut audio.sound);
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(PINK);
}
