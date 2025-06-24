use crate::diag::SourceResult;
use crate::foundations::{Value, func};
use kokoro_tts::{KokoroTts, Voice};
use rodio::{OutputStream, Sink, buffer::SamplesBuffer, Source};
use std::sync::Arc;
use typst_syntax::Spanned;

/// Synthesize text to speech.
///
/// # Arguments
/// * `source` - Text to be synthesized into speech.
/// * `volume` - The volume of the audio, where 1.0 is normal volume.
/// * `speed` - The speed of the audio, where 1.0 is normal speed.
/// * `count` - The number of times to play the audio.
///
/// # Example
/// ```example
/// tts("你好，世界！", volume: 1.0, speed: 1.0, count: 1)
/// ```
#[func]
pub fn tts(
    source: Spanned<String>,
    #[named]
    #[default(1.0)]
    volume: f64,
    #[named]
    #[default(1.0)]
    speed: f64,
    #[named]
    #[default(1)]
    count: usize,
) -> SourceResult<Value> {
    let text = source.v;
    let tts = Arc::new(
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(KokoroTts::new(
                "checkpoints/kokoro-v1.1-zh.onnx",
                "data/voices-v1.1-zh.bin",
            ))
            .unwrap(),
    );
    let tts_clone = Arc::clone(&tts);
    let text_clone = text.clone();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let (audio, _) = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(tts_clone.synth(&text_clone, Voice::Zm064(1)))
        .unwrap();
    let samples_buffer = SamplesBuffer::new(1, 24000, audio);
    // Adjust the volume and speed of the audio
    let amplified_samples_buffer = samples_buffer.amplify(volume as f32);
    let speed_samples_buffer = amplified_samples_buffer.speed(speed as f32);
    for _ in 0..count {
        // Assuming play_count is 1 for TTS, you can adjust as needed
        sink.append(speed_samples_buffer.clone());
    }

    sink.sleep_until_end();
    Ok(Value::None)
}
