use typst_syntax::Spanned;
use crate::diag::{LoadError, LoadedWithin, SourceResult};
use crate::engine::Engine;
use crate::foundations::{Value, func};
use crate::loading::{DataSource, Load};
use rodio::{Decoder, DeviceSinkBuilder, Player, Source};
use std::f64;

/// Play an audio file (mp3, wav, etc).
///
/// # Arguments
/// * `volume` - The volume of the audio, where 1.0 is normal volume.
/// * `speed` - The speed of the audio, where 1.0 is normal speed.
///
/// # Example
/// ```example
/// audio("sound.mp3", volume: 0.5, speed: 1.2)
/// ```
#[func]
pub fn audio(
    engine: &mut Engine,
    source: Spanned<DataSource>,
    #[named]
    #[default(1.0)]
    volume: f64,
    #[named]
    #[default(1.0)]
    speed: f64,
) -> SourceResult<Value> {
    let loaded = source.load(engine.world)?;
    play_audio_from_bytes(&loaded.data, volume as f32, speed as f32)
        .map_err(format_audio_error)
        .within(&loaded)?;
    Ok(Value::None)
}


fn play_audio_from_bytes(bytes: &[u8], volume: f32, speed: f32) -> Result<(), rodio::decoder::DecoderError> {
    // Open the default device sink and attach a player for playback.
    let handle = DeviceSinkBuilder::open_default_sink()
        .map_err(|e| rodio::decoder::DecoderError::IoError(format!("DeviceSinkBuilder error: {e}")))?;
    let player = Player::connect_new(&handle.mixer());

    let cursor = std::io::Cursor::new(bytes.to_vec());
    let source = Decoder::new(cursor)?;

    // Apply filters for volume and speed
    let amplified_source = source.amplify(volume);
    let speed_source = amplified_source.speed(speed);

    player.append(speed_source);
    player.sleep_until_end();
    Ok(())
}


fn format_audio_error(error: rodio::decoder::DecoderError) -> LoadError {
    LoadError::new(crate::diag::ReportPos::default(), "failed to play audio", error)
}
