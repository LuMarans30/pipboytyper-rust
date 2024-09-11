mod helper;
mod pip_boy;

use helper::{print_code, BashCode, TerminalWriter};
use pip_boy::{Config, PipBoy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let pip_boy = PipBoy::new(config)?;
    let mut terminal_writer = TerminalWriter::new(config)?;

    print_code(BashCode::Clear)?;
    print_code(BashCode::InvisibleCursor)?;

    pip_boy.print_boot_log(&mut terminal_writer)?;

    print_code(BashCode::Clear)?;
    print_code(BashCode::VisibleCursor)?;

    //TODO: Vault-Boy animation (vault_boy.rs)

    Ok(())
}
