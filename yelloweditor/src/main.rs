mod editor;
mod rom;

use std::io::Result;

fn main() -> Result<()> {
    let rom = rom::ROM::new("../yellow.gbc".to_string())?;
    let rom_editor = editor::ROMEditor::new(rom);

    rom_editor.run()?;

    Ok(())
}
