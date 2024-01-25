use std::fmt;
use std::path::PathBuf;

//class used to envelope the result which will display the requested lines
#[derive(Debug)]
pub struct Result<'a> {
    pub start: usize,
    pub end: usize,
    pub content: &'a Vec<u8>,
    pub path: &'a PathBuf,
    pub count: usize,
}

impl<'a> fmt::Display for Result<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //index for where a new line starts
        //search content to find \n and increment by 1 as we need the next character
        //if none are found use the beginning of the content
        let line_start = (0..self.start)
            .rev()
            .find(|v| self.content[*v] == b'\n')
            .map_or(0, |v| v + 1);

        //index for where a new line ends
        //no need to reverse because we only want the first instance
        //if none are found use the end of the content
        let line_end = (self.end..self.content.len())
            .find(|v| self.content[*v] == b'\n')
            .unwrap_or(self.content.len());

        //print line that contains regex pattern
        writeln!(
            f,
            "[{}] @ {:?}: {}",
            self.count,
            self.path,
            String::from_utf8_lossy(&self.content[line_start..line_end])
        )?;

        Ok(())
    }
}
