use std::{fs::File, io::Read};

pub fn read_file(file_name: &str) -> anyhow::Result<String> {
    let mut contents = String::new();
    File::open(file_name)?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub trait OptionAnyhow<T> {
    fn ok_anyhow(self) -> anyhow::Result<T>;
}

impl<T> OptionAnyhow<T> for Option<T> {
    fn ok_anyhow(self) -> anyhow::Result<T> {
        self.ok_or_else(|| anyhow::anyhow!("expected Some value"))
    }
}

pub type AnyResult<T> = anyhow::Result<T>;
