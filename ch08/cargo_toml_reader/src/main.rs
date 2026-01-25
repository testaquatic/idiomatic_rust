use cargo_toml_reader::CargoTomlReader;

fn main() -> Result<(), std::io::Error> {
    let cargo_reader = CargoTomlReader::new()?;
    cargo_reader.for_each(|(line_number, line)| println!("{}: {}", line_number, line));

    Ok(())
}
