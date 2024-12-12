use std::{
    fs::File,
    io::{BufRead, BufReader},
};
pub fn run() -> std::io::Result<()> {
    // Open the puzzle input with the lists of ID's
    let lists_file = File::open("src/input.txt")?;
    let reader = BufReader::new(lists_file);

    let mut total_distance: u32 = 0;

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

    // Sorts both lists in ascending order
    left_list.sort();
    right_list.sort();

    // Calculate the distances between each pair of numbers and add to `total distance`
    total_distance += calculate_distance(&left_list, &right_list);

    println!("Total Distance = {total_distance}");

    Ok(())
}

/// Calculates the total distance by adding the absolute difference between each
/// pair of values ​​in the already sorted lists.
fn calculate_distance(right_list: &[u32], left_right: &[u32]) -> u32 {
    let mut distance: u32 = 0;

    for (num1, num2) in right_list.iter().zip(left_right.iter()) {
        distance += num1.abs_diff(*num2)
    }
    distance
}

#[cfg(test)]
mod tests {
    use crate::part_one::calculate_distance;

    #[test]
    fn test_calculate_distance() {
        let mut left_list: Vec<u32> = vec![77221, 61169, 49546, 11688, 15820];
        let mut right_list: Vec<u32> = vec![93653, 27995, 69782, 41563, 48282];

        left_list.sort();
        right_list.sort();

        assert_eq!(calculate_distance(&left_list, &right_list), 68359)
    }
}
