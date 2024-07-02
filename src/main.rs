use std::f32::consts::PI;
use std::i16;
use hound;

const G3: f32 = 195.9977;
const C4: f32 = 261.6256;
const D4: f32 = 293.6648;
const G4: f32 = 391.9954;
const B4: f32 = 493.8833;
const A4: f32 = 440.0;
const C5: f32 = 523.2511;
const D5: f32 = 587.3295;
const E5: f32 = 659.2551;
const F5: f32 = 698.4565;

const SAMPLE_RATE: u32 = 44100;
const MAX_AMPLITUDE: f32 = i16::MAX as f32 / 5.0;

/// Computes a sine wave
fn sin(t: f32, freq: f32) -> f32 {
    return (t * freq * 2.0 * PI).sin()
}

/// Computes a basic ADSR envelope
fn envelope<F: FnMut(f32, f32) -> ()>(mut f: F) {
    let mut amplitude = 0.0;
    for t in (0 .. SAMPLE_RATE).map(|x| x as f32 / SAMPLE_RATE as f32) {
        // Attack
        if t <= 0.1 {
            amplitude = t*10.0 * MAX_AMPLITUDE;
        }

        f(t, amplitude);

        // Release
        if t >= 0.9 {
            amplitude = (1.0 - t)*10.0 * MAX_AMPLITUDE;
        }
    }
}

fn main() {
    println!("Hello, world!");

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("out/sine.wav", spec).unwrap();

    // Dm7
    envelope(|t, amplitude| {
        let sample = sin(t, D4)
            + sin(t, A4)
            + sin(t, C5)
            + sin(t, D5)
            + sin(t, F5);
        writer.write_sample((sample * amplitude) as i16).unwrap();
    });

    // G7
    envelope(|t, amplitude| {
        let sample = sin(t, G3)
            + sin(t, G4)
            + sin(t, B4)
            + sin(t, D5)
            + sin(t, F5);
        writer.write_sample((sample * amplitude) as i16).unwrap();
    });

    // Cmaj7
    envelope(|t, amplitude| {
        let sample = sin(t, C4)
            + sin(t, G4)
            + sin(t, B4)
            + sin(t, C5)
            + sin(t, E5);
        writer.write_sample((sample * amplitude) as i16).unwrap();
    });
}
