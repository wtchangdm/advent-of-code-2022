pub mod fs_helper {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader, Error, Lines};

    pub fn read_lines(path: &str) -> Result<Lines<BufReader<File>>, Error> {
        let file = File::open(path)?;
        Ok(io::BufReader::new(file).lines())
    }
}
