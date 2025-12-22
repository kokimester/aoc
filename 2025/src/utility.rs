pub mod utility {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    pub fn open_and_read_file(file: String) -> Result<Vec<String>, io::Error> {
        // Open the file
        let file = File::open(file)?;

        // Create a buffered reader
        let reader = BufReader::new(file);

        // Collect lines into a vector
        let lines: Result<Vec<String>, io::Error> = reader.lines().collect();

        lines
    }
}
