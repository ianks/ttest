use std::{error::Error, fs::File, io::BufReader};

use bstr::{io::BufReadExt, ByteSlice, Finder};

use crate::test_file::TestFile;

/// Searches and indexes a test file.
#[derive(Debug, PartialEq, Clone)]
pub struct TestIndex {
    entries: Vec<IndexEntry>,
}

impl TestIndex {
    /// Creates a new [`TestIndex`] from a list of patterns.
    pub fn build(test_file: &TestFile, patterns: &[&str]) -> Result<Self, Box<dyn Error>> {
        let searchers = patterns.iter().map(Finder::new).collect::<Vec<_>>();

        let file = File::open(test_file)?;
        let mut reader = BufReader::new(file);
        let mut current_line = 0;
        let mut entries = vec![];

        reader.for_byte_line_with_terminator(|line| {
            current_line += 1;

            if searchers.iter().any(|s| s.find(line).is_some()) {
                if let Ok(content) = line.to_str() {
                    entries.push(IndexEntry::new(current_line, content.to_string()));
                }
            }

            Ok(true)
        })?;

        Ok(Self { entries })
    }

    /// Returns the closest [`IndexEntry`] to the given line number.
    pub fn closest_to_line_number(&self, line: u32) -> Option<IndexEntry> {
        self.entries
            .iter()
            .min_by(|a, b| {
                let a_distance = a.line_number.abs_diff(line);
                let b_distance = b.line_number.abs_diff(line);
                b_distance.cmp(&a_distance)
            })
            .cloned()
    }
}

/// An indexed test.
#[derive(Debug, PartialEq, Clone)]
pub struct IndexEntry {
    line_number: u32,
    content: String,
}

impl IndexEntry {
    /// Creates a new [`IndexEntry`] from a line number and content.
    pub fn new(line_number: u32, content: String) -> Self {
        Self {
            line_number,
            content,
        }
    }

    /// Returns the content of the test.
    pub fn content(&self) -> &str {
        &self.content
    }
}
