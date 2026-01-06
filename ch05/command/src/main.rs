use std::fs::File;

use command::{Command, ReadFile, WriteFile};

fn main() -> Result<(), std::io::Error> {
    let file = File::options()
        .read(true)
        .create(true)
        .append(true)
        .open("file.txt")?;

    let commands: Vec<Box<dyn Command>> = vec![
        ReadFile::new(file.try_clone()?),
        WriteFile::new("file contents\n".into(), file.try_clone()?),
        ReadFile::new(file.try_clone()?),
    ];

    commands
        .into_iter()
        .try_for_each(|command| command.execute())
}
