use std::{fs, io};

pub fn list_files(dir: &str) -> io::Result<Vec<String>> {
    fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path().display().to_string()))
        .collect::<Result<Vec<_>, io::Error>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        match list_files(".") {
            Ok(list) => assert_eq!(list.len(), 4),
            Err(err) => assert_eq!(err.to_string(), "No such file or directory (os error 2)"),
        }
    }
}
