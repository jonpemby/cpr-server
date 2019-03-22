use std::io::{BufRead, BufReader};
use std::fs::File as StdFile;
use std::path::Path;

pub struct File<'a> {
    path: &'a Path,
}

impl <'a> File<'a> {
    pub fn open(path: &Path) -> File { File { path } }

    pub fn read(&self) -> Option<String> {
        let mut buf = String::new();
        let file = StdFile::open(&self.path);

        if file.is_err() {
            return Option::None;
        }

        let reader = BufReader::new(file.unwrap());

        for line in reader.lines() {
            if line.is_err() {
                return Option::None;
            }

            buf.push_str(line.unwrap().as_str());
        }

        Option::Some(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::File;
    use std::path::Path;

    #[test]
    fn test_it_reads_file() {
        let path = Path::new("./fixtures/PortConfig.yaml");
        let file = File::open(&path);

        let res = file.read();

        assert_eq!(true, res.is_some());
        assert_eq!("port: 3310", res.unwrap());
    }

    #[test]
    fn test_it_returns_none_when_io_fails() {
        let path = Path::new("./fixtures/DoesNotExist.yaml");
        let file = File::open(&path);

        let res = file.read();

        assert_eq!(true, res.is_none());
    }
}