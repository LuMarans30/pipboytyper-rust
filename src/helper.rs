use rand::Rng;
use soloud::{AudioExt, LoadExt, Soloud, Wav};
use std::{fs, io::Write, path::Path, thread::sleep, time::Duration};
use thiserror::Error;

use crate::pip_boy::Config;

#[derive(Error, Debug)]
pub enum HelperError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Sound error: {0}")]
    Sound(String),
    #[error("Terminal size error")]
    TerminalSize,
}

const BASH_CODES: [&str; 3] = [
    "\x1B[2J\x1B[1;1H", // clear
    "\x1B[?25h",        // visible cursor
    "\x1B[?25l",        // invisible cursor
];

pub enum BashCode {
    Clear,
    VisibleCursor,
    InvisibleCursor,
}

pub fn print_code(code: BashCode) -> Result<(), HelperError> {
    let code_str = match code {
        BashCode::Clear => BASH_CODES[0],
        BashCode::VisibleCursor => BASH_CODES[1],
        BashCode::InvisibleCursor => BASH_CODES[2],
    };
    print!("{}", code_str);
    std::io::stdout().flush()?;
    Ok(())
}

pub struct SoundPlayer {
    sl: Soloud,
    wav_keypress: Wav,
    //wav_enter: Wav,
}

impl SoundPlayer {
    pub fn new(filename_keypress: &str, _filename_enterkey: &str) -> Result<Self, HelperError> {
        let sl = Soloud::default().map_err(|e| HelperError::Sound(e.to_string()))?;
        let wav_keypress = Self::load_wav(filename_keypress)?;
        //let wav_enter = Self::load_wav(filename_enterkey)?;

        Ok(SoundPlayer {
            sl,
            wav_keypress,
            //wav_enter,
        })
    }

    fn load_wav(filename: &str) -> Result<Wav, HelperError> {
        let mut wav = Wav::default();
        let file_contents = fs::read(Path::new(
            format!("./resources/audio/{}", filename).as_str(),
        ))?;
        wav.load_mem(&file_contents)
            .map_err(|e| HelperError::Sound(e.to_string()))?;
        Ok(wav)
    }

    pub fn play_keypress(&self) {
        self.sl.play(&self.wav_keypress);
    }

    /*pub fn play_enter(&self) {
        self.sl.play(&self.wav_enter);
    }*/
}

pub struct TerminalWriter {
    sound_player: SoundPlayer,
    config: Config,
}

impl TerminalWriter {
    pub fn new(config: Config) -> Result<Self, HelperError> {
        let sound_player = SoundPlayer::new("pipboy_keypress.ogg", "pipboy_enterkey.ogg")?;
        Ok(TerminalWriter {
            sound_player,
            config,
        })
    }

    pub fn scroll(&self, s: &str) -> Result<(), HelperError> {
        let mut rng = rand::thread_rng();

        for c in s.chars() {
            print_code(BashCode::VisibleCursor)?;

            print!("{c}");
            if c == '\n' {
                //self.sound_player.play_enter();
                print_code(BashCode::InvisibleCursor)?;
            } else if c != ' ' {
                self.sound_player.play_keypress();
                sleep(Duration::from_millis(rng.gen_range(
                    self.config.min_time_char..self.config.max_time_char,
                )));
            }
            std::io::stdout().flush()?;
        }

        sleep(Duration::from_millis(self.config.time_line));
        Ok(())
    }

    pub fn display_center(&self, s: &str) -> Result<(String, String), HelperError> {
        let cols = termsize::get().ok_or(HelperError::TerminalSize)?.cols;
        let spaces = (cols - s.len() as u16) / 2;
        let spaces_str = " ".repeat(spaces.into());

        Ok((format!("\n{}{}", spaces_str, s), spaces_str))
    }

    pub fn display_offset(&self, text: &str, spaces: &str) -> String {
        format!("\n{}{}", spaces, text)
    }
}
