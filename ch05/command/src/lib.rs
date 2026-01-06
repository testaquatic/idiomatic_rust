use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Seek, Write},
};

pub trait Command {
    fn execute(&self) -> Result<(), Error>;
}

pub struct ReadFile {
    receiver: File,
}

impl ReadFile {
    pub fn new(receiver: File) -> Box<Self> {
        Box::new(Self { receiver })
    }
}

impl Command for ReadFile {
    fn execute(&self) -> Result<(), Error> {
        println!("Reading from start of file");
        let mut reader = BufReader::new(&self.receiver);
        reader.seek(std::io::SeekFrom::Start(0))?;

        for (count, line) in reader.lines().enumerate() {
            println!("{:2}: {}", count + 1, line?);
        }

        Ok(())
    }
}

pub struct WriteFile {
    content: String,
    receiver: File,
}

impl WriteFile {
    pub fn new(content: String, receiver: File) -> Box<Self> {
        Box::new(Self { content, receiver })
    }
}

impl Command for WriteFile {
    fn execute(&self) -> Result<(), Error> {
        println!("Writing new content to file");
        let mut writer = self.receiver.try_clone()?;

        writer.write_all(self.content.as_bytes())?;
        writer.flush()?;

        Ok(())
    }
}
