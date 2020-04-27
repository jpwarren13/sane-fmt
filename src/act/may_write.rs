use std::{fs, path::Path, io};

pub type Act = fn(&Path, &String) -> io::Result<()>;

#[rustfmt::skip]
pub fn get(write: bool) -> Act {
    if write {
        |path, content| fs::write(path, content)
    } else {
        |_, _| Ok(())
    }
}
