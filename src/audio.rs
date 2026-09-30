use rodio::{
    Decoder, Player,
    source::Source,
    stream::{DeviceSinkBuilder, MixerDeviceSink},
};
use std::{
    fs::File,
    path::Path,
    process::{Child, Command},
    sync::OnceLock,
};

pub struct AudioEngine {
    output: Option<MixerDeviceSink>,
    current: Option<Player>,
    speech: Option<Child>,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            output: None,
            current: None,
            speech: None,
        }
    }

    pub fn stop(&mut self) {
        self.current.take();
        if let Some(mut speech) = self.speech.take() {
            let _ = speech.kill();
            let _ = speech.wait();
        }
    }

    pub fn play(
        &mut self,
        resources: &Path,
        audio_id: Option<&str>,
        french_text: &str,
        slow: bool,
    ) -> Result<(), String> {
        self.stop();

        let clip_error = if let Some(id) = audio_id {
            match self.play_clip(resources, id, slow) {
                Ok(()) => return Ok(()),
                Err(error) => error,
            }
        } else {
            "No recorded audio is linked to this phrase.".to_owned()
        };

        match self.play_speech(french_text, slow) {
            Ok(()) => Ok(()),
            Err(error) => Err(format!("{clip_error} {error}")),
        }
    }

    fn play_clip(&mut self, resources: &Path, id: &str, slow: bool) -> Result<(), String> {
        let path = resources.join("audio").join(format!("{id}.mp3"));
        let file =
            File::open(&path).map_err(|error| format!("Audio clip is unavailable: {error}"))?;
        let decoder = Decoder::try_from(file)
            .map_err(|error| format!("Could not decode the audio clip: {error}"))?;

        if self.output.is_none() {
            self.output = Some(
                DeviceSinkBuilder::open_default_sink()
                    .map_err(|error| format!("Could not open audio output: {error}"))?,
            );
        }
        let output = self.output.as_ref().expect("audio output initialized");
        let player = Player::connect_new(output.mixer());
        player.append(decoder.speed(if slow { 0.72 } else { 1.0 }));
        self.current = Some(player);
        Ok(())
    }

    fn play_speech(&mut self, french_text: &str, slow: bool) -> Result<(), String> {
        let mut command = Command::new("say");
        if let Some(voice) = french_voice() {
            command.arg("-v").arg(voice);
        }
        command
            .arg("-r")
            .arg(if slow { "125" } else { "185" })
            .arg(french_text);
        self.speech = Some(
            command
                .spawn()
                .map_err(|error| format!("French voice fallback is unavailable: {error}"))?,
        );
        Ok(())
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        self.stop();
    }
}

fn french_voice() -> Option<&'static str> {
    static VOICE: OnceLock<Option<String>> = OnceLock::new();
    VOICE
        .get_or_init(|| {
            let output = Command::new("say").arg("-v").arg("?").output().ok()?;
            let text = String::from_utf8_lossy(&output.stdout);
            for preferred in ["fr_CA", "fr_FR"] {
                for line in text.lines() {
                    if let Some(locale_at) = line.find(preferred) {
                        let voice = line[..locale_at].trim();
                        if !voice.is_empty() {
                            return Some(voice.to_owned());
                        }
                    }
                }
            }
            None
        })
        .as_deref()
}
