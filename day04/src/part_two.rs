use std::io::{self, Read};

pub fn run() -> io::Result<()> {
    // Open the puzzle input
    let mut file = std::fs::File::open("src/input.txt")?;
    let mut text = String::new();

    file.read_to_string(&mut text)
        .expect("Error reading the file");

    let xmas_count = count_xmas_pattern(&text);

    println!("X-MAS pattern count: {xmas_count}");

    Ok(())
}

fn count_xmas_pattern(text: &str) -> u64 {
    let lines: Vec<&str> = text.lines().collect();
    let rows = lines.len();
    let cols = lines.first().unwrap().len();

    let pattern = "MAS";
    let pattern_rev = "SAM";

    let mut diagonal_count = 0;

    // Mark cells that are already checked
    let mut checked = vec![vec![false; cols]; rows];

    // Count patterns in diagonals
    for r in 0..(rows - 2) {
        for c in 0..(cols - 2) {
            let mut diagonal1 = String::new();
            let mut diagonal2 = String::new();

            // Build diagonals
            for i in 0..3 {
                diagonal1.push(lines[r + i].chars().nth(c + i).unwrap());
                diagonal2.push(lines[r + i].chars().nth(c + 2 - i).unwrap());
            }

            // Check if diagonals match pattern or reverse
            if (diagonal1 == pattern || diagonal1 == pattern_rev)
                && (diagonal2 == pattern || diagonal2 == pattern_rev)
            {
                diagonal_count += 1;

                // Mark cells as checked
                for i in 0..3 {
                    checked[r + i][c + i] = true;
                    checked[r + i][c + 2 - i] = true;
                }
            }
        }
    }

    let grid: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    let mut square_count = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if grid[r][c] == 'A' && !checked[r][c] {
                // Check the letters around 'A'
                if (grid[r - 1][c - 1] == 'M' && grid[r + 1][c + 1] == 'S'
                    || grid[r - 1][c - 1] == 'S' && grid[r + 1][c + 1] == 'M')
                    && (grid[r - 1][c + 1] == 'M' && grid[r + 1][c - 1] == 'S'
                        || grid[r - 1][c + 1] == 'S' && grid[r + 1][c - 1] == 'M')
                {
                    square_count += 1;
                }
            }
        }
    }

    // X-MAS count
    diagonal_count as u64 + square_count as u64
}

#[cfg(test)]
mod tests {
    use crate::part_two::count_xmas_pattern;

    const TEXT: &str = "MMSMMMMM\n\
                        SAAAMSSSM\n\
                        ASXSSSAXX\n\
                        MMSMAMMMX\n\
                        SAMMASMMX\n\
                        MXSAMXMMS\n\
                        MSXMAAMSM\n\
                        MAMXSASAM\n\
                        MMSMASMMX";

    #[test]
    fn test_count_xmas_pattern() {
        assert_eq!(count_xmas_pattern(TEXT), 4)
    }
}
