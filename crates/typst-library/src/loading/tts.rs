use crate::diag::SourceResult;
use crate::foundations::{Value, func};
use kokoro_tts::{KokoroTts, Voice};
use rodio::{OutputStream, Sink, buffer::SamplesBuffer, Source};
use std::sync::Arc;
use typst_syntax::Spanned;
use crate::diag::SourceDiagnostic;

/// Synthesize text to speech.
///
/// # Arguments
/// * `source` - Text to be synthesized into speech.
/// * `volume` - The volume of the audio, where 1.0 is normal volume.
/// * `speed` - The speed of the audio, where 1.0 is normal speed.
/// * `count` - The number of times to play the audio.
/// * `voice` - The voice to use for synthesis (default is "Zm064").
///
/// # Example
/// ```example
/// tts("中國人民不信邪也不怕邪，不惹事也不怕事，任何外國不要指望我們會拿自己的核心利益做交易，不要指望我們會吞下損害我國主權、安全、發展利益的苦果！", volume: 1.0, speed: 1.0, count: 1, voice: "Zf001")
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
    #[named]
    #[default("Zm064".to_string())]
    voice: String,
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
    let voice_enum = match voice.as_str() {
        "AfMaple" => Voice::AfMaple(1),
        "AfSol" => Voice::AfSol(1),
        "BfVale" => Voice::BfVale(1),
        "Zf001" => Voice::Zf001(1),
        "Zf002" => Voice::Zf002(1),
        "Zf003" => Voice::Zf003(1),
        "Zf004" => Voice::Zf004(1),
        "Zf005" => Voice::Zf005(1),
        "Zf006" => Voice::Zf006(1),
        "Zf007" => Voice::Zf007(1),
        "Zf008" => Voice::Zf008(1),
        "Zf017" => Voice::Zf017(1),
        "Zf018" => Voice::Zf018(1),
        "Zf019" => Voice::Zf019(1),
        "Zf021" => Voice::Zf021(1),
        "Zf022" => Voice::Zf022(1),
        "Zf023" => Voice::Zf023(1),
        "Zf024" => Voice::Zf024(1),
        "Zf026" => Voice::Zf026(1),
        "Zf027" => Voice::Zf027(1),
        "Zf028" => Voice::Zf028(1),
        "Zf032" => Voice::Zf032(1),
        "Zf036" => Voice::Zf036(1),
        "Zf038" => Voice::Zf038(1),
        "Zf039" => Voice::Zf039(1),
        "Zf040" => Voice::Zf040(1),
        "Zf042" => Voice::Zf042(1),
        "Zf043" => Voice::Zf043(1),
        "Zf044" => Voice::Zf044(1),
        "Zf046" => Voice::Zf046(1),
        "Zf047" => Voice::Zf047(1),
        "Zf048" => Voice::Zf048(1),
        "Zf049" => Voice::Zf049(1),
        "Zf051" => Voice::Zf051(1),
        "Zf059" => Voice::Zf059(1),
        "Zf060" => Voice::Zf060(1),
        "Zf067" => Voice::Zf067(1),
        "Zf070" => Voice::Zf070(1),
        "Zf071" => Voice::Zf071(1),
        "Zf072" => Voice::Zf072(1),
        "Zf073" => Voice::Zf073(1),
        "Zf074" => Voice::Zf074(1),
        "Zf075" => Voice::Zf075(1),
        "Zf076" => Voice::Zf076(1),
        "Zf077" => Voice::Zf077(1),
        "Zf078" => Voice::Zf078(1),
        "Zf079" => Voice::Zf079(1),
        "Zf083" => Voice::Zf083(1),
        "Zf084" => Voice::Zf084(1),
        "Zf085" => Voice::Zf085(1),
        "Zf086" => Voice::Zf086(1),
        "Zf087" => Voice::Zf087(1),
        "Zf088" => Voice::Zf088(1),
        "Zf090" => Voice::Zf090(1),
        "Zf092" => Voice::Zf092(1),
        "Zf093" => Voice::Zf093(1),
        "Zf094" => Voice::Zf094(1),
        "Zf099" => Voice::Zf099(1),
        "Zm009" => Voice::Zm009(1),
        "Zm010" => Voice::Zm010(1),
        "Zm011" => Voice::Zm011(1),
        "Zm012" => Voice::Zm012(1),
        "Zm013" => Voice::Zm013(1),
        "Zm014" => Voice::Zm014(1),
        "Zm015" => Voice::Zm015(1),
        "Zm016" => Voice::Zm016(1),
        "Zm020" => Voice::Zm020(1),
        "Zm025" => Voice::Zm025(1),
        "Zm029" => Voice::Zm029(1),
        "Zm030" => Voice::Zm030(1),
        "Zm031" => Voice::Zm031(1),
        "Zm033" => Voice::Zm033(1),
        "Zm034" => Voice::Zm034(1),
        "Zm035" => Voice::Zm035(1),
        "Zm037" => Voice::Zm037(1),
        "Zm041" => Voice::Zm041(1),
        "Zm045" => Voice::Zm045(1),
        "Zm050" => Voice::Zm050(1),
        "Zm052" => Voice::Zm052(1),
        "Zm053" => Voice::Zm053(1),
        "Zm054" => Voice::Zm054(1),
        "Zm055" => Voice::Zm055(1),
        "Zm056" => Voice::Zm056(1),
        "Zm057" => Voice::Zm057(1),
        "Zm058" => Voice::Zm058(1),
        "Zm061" => Voice::Zm061(1),
        "Zm062" => Voice::Zm062(1),
        "Zm063" => Voice::Zm063(1),
        "Zm064" => Voice::Zm064(1),
        "Zm065" => Voice::Zm065(1),
        "Zm066" => Voice::Zm066(1),
        "Zm068" => Voice::Zm068(1),
        "Zm069" => Voice::Zm069(1),
        "Zm080" => Voice::Zm080(1),
        "Zm081" => Voice::Zm081(1),
        "Zm082" => Voice::Zm082(1),
        "Zm089" => Voice::Zm089(1),
        "Zm091" => Voice::Zm091(1),
        "Zm095" => Voice::Zm095(1),
        "Zm096" => Voice::Zm096(1),
        "Zm097" => Voice::Zm097(1),
        "Zm098" => Voice::Zm098(1),
        "Zm100" => Voice::Zm100(1),
        _ => {
            return Err(ecow::EcoVec::from([SourceDiagnostic::error(
            source.span,
            format!("Voice {} not found", voice),
            )]).into());
        }
    };
    let (audio, _) = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(tts_clone.synth(&text_clone, voice_enum))
        .unwrap();
    let samples_buffer = SamplesBuffer::new(1, 24000, audio);
    // Adjust the volume and speed of the audio
    let amplified_samples_buffer = samples_buffer.amplify(volume as f32);
    let speed_samples_buffer = amplified_samples_buffer.speed(speed as f32);
    for _ in 0..count {
        sink.append(speed_samples_buffer.clone());
    }

    sink.sleep_until_end();
    Ok(Value::None)
}
