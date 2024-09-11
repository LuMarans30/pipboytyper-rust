use std::{thread::sleep, time::Duration};

#[derive(Clone, Copy)]
pub struct Config {
    pub min_time_char: u64,
    pub max_time_char: u64,
    pub time_line: u64,
    pub end_section_time: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            min_time_char: 30,
            max_time_char: 40,
            time_line: 500,
            end_section_time: 10000,
        }
    }
}

pub struct PipBoy {
    config: Config,
}

impl PipBoy {
    pub fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(PipBoy { config })
    }

    pub fn print_boot_log(
        &self,
        terminal_writer: &mut crate::helper::TerminalWriter,
    ) -> Result<(), crate::helper::HelperError> {
        sleep(Duration::from_millis(500));

        let (header, spaces_str) =
            terminal_writer.display_center("*************** PIP-OS(R) V7.1.0.8 ***************")?;

        terminal_writer.scroll(&header)?;

        const TEXT: &str = "COPYRIGHT 2075 ROBCO\nLOADER 1.1\nEXEC VERSION 41.10\n64K RAM SYSTEM\n38911 BYTES FREE\nNO HOLOTAPE FOUND\nLOAD ROM(1): DEITRIX 303";

        for line in TEXT.lines() {
            let offset_line = terminal_writer.display_offset(line, &spaces_str);
            terminal_writer.scroll(&offset_line)?;
        }

        sleep(Duration::from_millis(self.config.end_section_time));
        Ok(())
    }
}
