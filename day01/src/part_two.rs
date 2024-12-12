use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> std::io::Result<()> {
    // Open the puzzle input with the lists of ID's
    let lists_file = File::open("src/input.txt")?;
    let reader = BufReader::new(lists_file);

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    // Reads line by line and adds the values ​​to the lists
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let lists: Vec<&str> = line.split_whitespace().collect();

                left_list.push(lists[0].parse().expect("Error parsing left_list number"));
                right_list.push(lists[1].parse().expect("Error parsing right_list number"));
            }
            Err(e) => eprintln!("Error reading line: {e}"),
        }
    }

    let similarity_score: u32 = calculate_similarity_score(&left_list, &right_list);

    println!("Similarity Score: {similarity_score}");

    Ok(())
}

/// Calculates the similarity score by adding the product of the number of times that
/// a value from the first list appears in the second
fn calculate_similarity_score(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .map(|&l_item| {
            let count = right_list
                .iter()
                .filter(|&&r_item| r_item == l_item)
                .count();
            l_item * count as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part_two::calculate_similarity_score;

    #[test]
    fn test_calculate_similarity_score() {
        let mut left_list: Vec<u32> = vec![77221, 61169, 49546, 11688, 15820];
        let mut right_list: Vec<u32> = vec![93653, 77221, 77221, 41563, 77221];

        left_list.sort();
        right_list.sort();

        assert_eq!(calculate_similarity_score(&left_list, &right_list), 231663)
    }
}
