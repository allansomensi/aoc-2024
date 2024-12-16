use std::io::Read;

/// The engine that searches for words in texts
struct WordFinder {
    text: String,
    word: String,
}

impl WordFinder {
    fn new(text: String, word: String) -> Self {
        Self { text, word }
    }

    /// Find vertical occurrences
    fn find_by_vertical(&self) -> u32 {
        let mut word_count: u32 = 0;
        let rows: Vec<&str> = self.text.lines().collect();

        for column in 0..rows.len() {
            let mut column_text = String::new();

            // Build the column
            for row in &rows {
                column_text.push(row.chars().nth(column).unwrap());
            }

            // Normal
            word_count += count_occurrences(&column_text, &self.word);
            // Reverse
            word_count += count_occurrences(&column_text, &reversed_word(&self.word));
        }

        word_count
    }

    fn find_by_horizontal(&self) -> u32 {
        let mut word_count: u32 = 0;

        for line in self.text.lines() {
            // Normal
            word_count += count_occurrences(line, &self.word);
            // Reverse
            word_count += count_occurrences(line, &reversed_word(&self.word));
        }

        word_count
    }

    /// Find diagonal occurrences
    fn find_by_diagonal(&self) -> u32 {
        let mut word_count: u32 = 0;
        let rows: Vec<&str> = self.text.lines().collect();

        // Helper to count occurrences in a diagonal
        fn count_diagonal_occurrences(
            rows: &[&str],
            start_row: usize,
            start_col: usize,
            delta_row: isize,
            delta_col: isize,
            word: &str,
        ) -> u32 {
            let mut diagonal_text = String::new();
            let mut row = start_row as isize;
            let mut col = start_col as isize;

            while row >= 0 && row < rows.len() as isize && col >= 0 && col < rows.len() as isize {
                diagonal_text.push(rows[row as usize].chars().nth(col as usize).unwrap());
                row += delta_row;
                col += delta_col;
            }

            count_occurrences(&diagonal_text, word)
                + count_occurrences(&diagonal_text, &reversed_word(word))
        }

        // Descending diagonals
        for start_row in 0..rows.len() {
            word_count += count_diagonal_occurrences(&rows, start_row, 0, 1, 1, &self.word);
        }
        for start_col in 1..rows.len() {
            word_count += count_diagonal_occurrences(&rows, 0, start_col, 1, 1, &self.word);
        }

        // Ascending diagonals
        for start_row in 0..rows.len() {
            word_count += count_diagonal_occurrences(&rows, start_row, 0, -1, 1, &self.word);
        }
        for start_col in 1..rows.len() {
            word_count +=
                count_diagonal_occurrences(&rows, rows.len() - 1, start_col, -1, 1, &self.word);
        }

        word_count
    }
}

fn count_occurrences(text: &str, pattern: &str) -> u32 {
    let mut count = 0;
    let mut pos = 0;

    while let Some(found) = text[pos..].find(pattern) {
        count += 1;
        pos += found + 1; // Move forward one position to capture overlap
    }

    count
}

fn reversed_word(word: &str) -> String {
    let reversed_word: String = word.chars().rev().collect();
    reversed_word
}

pub fn run() -> std::io::Result<()> {
    // Open the puzzle input
    let mut file = std::fs::File::open("src/input.txt")?;
    let mut raw_text = String::new();
    file.read_to_string(&mut raw_text)
        .expect("Error reading file");

    let wf = WordFinder::new(raw_text, String::from("XMAS"));

    let word_count: u32 = wf.find_by_vertical() + wf.find_by_horizontal() + wf.find_by_diagonal();

    println!("Word count: {word_count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::WordFinder;

    const TEXT: &str = "\
    XMASMXXASX\n\
    MSAMXMSMSS\n\
    AMXSXXAAMM\n\
    MXAMAMMSMX\n\
    XMMSAAXAMM\n\
    XXAAMSXAMA\n\
    SMXMSASXSS\n\
    SAXAMASMAA\n\
    MXXMASMAMM\n\
    MXMXAXMSSX";

    const WORD: &str = "XMAS";

    #[test]
    fn test_find_by_vertical() {
        let wf = WordFinder::new(TEXT.to_string(), WORD.to_string());
        println!("{TEXT}");
        assert_eq!(wf.find_by_vertical(), 5);
    }

    #[test]
    fn test_find_by_horizontal() {
        let wf = WordFinder::new(TEXT.to_string(), WORD.to_string());
        assert_eq!(wf.find_by_horizontal(), 3);
    }

    #[test]
    fn test_find_by_diagonal() {
        let wf = WordFinder::new(TEXT.to_string(), WORD.to_string());
        assert_eq!(wf.find_by_diagonal(), 4);
    }

    #[test]
    fn test_find_all_occurrences() {
        let wf = WordFinder::new(TEXT.to_string(), WORD.to_string());
        assert_eq!(
            wf.find_by_vertical() + wf.find_by_horizontal() + wf.find_by_diagonal(),
            12
        );
    }
}
