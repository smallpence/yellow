mod editor;
mod poke;

use std::io::Result;

fn main() -> Result<()> {
    let rom = poke::ROM::new("../yellow.gbc")?;
    let rom_editor = editor::ROMEditor::new(&rom);

    rom_editor.run()?;

    Ok(())
}
