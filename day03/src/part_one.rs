use std::{fs::File, io::Read};

pub fn run() -> std::io::Result<()> {
    // Open the puzzle input with the corrupted memory
    let corrupted_memory_file = File::open("src/input.txt")?;

    // The parameters of the mul function
    let mul_params: String = find_mul_params_from_file(corrupted_memory_file);

    let sum_of_products = calculate_product_of_mul_params(mul_params);

    println!("Sum of products: {}", sum_of_products);

    Ok(())
}

fn find_mul_params_from_file(mut file: File) -> String {
    let mut corrupted_memory = String::new();
    // Collect all lines of the file into a String
    file.read_to_string(&mut corrupted_memory)
        .expect("Error parsing file buffer to string");

    // Regex to search for correct mul functions in corrupted memory file
    let mul_function_regex =
        regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("Error reading regex");

    // Collect all correct mul functions into a Vec - this looks like "mul(999,999)"
    let mul_functions: Vec<&str> = mul_function_regex
        .find_iter(&corrupted_memory)
        .map(|f| f.as_str())
        .collect();

    // Collect a Vec of &str into a String
    let mul_functions = mul_functions.join("\n");

    // Regex to search for numbers to be multiplied
    let mul_params_regex = regex::Regex::new(r"\d{1,3},\d{1,3}").expect("Error reading regex");

    // Collect all correct mul params into a Vec - this looks like "999,999"
    let mul_params: Vec<&str> = mul_params_regex
        .find_iter(&mul_functions)
        .map(|f| f.as_str())
        .collect();

    // Collect a Vec of &str into a String
    mul_params.join("\n")
}

fn calculate_product_of_mul_params(mul_params: String) -> u64 {
    let mut sum_of_products: u64 = 0;

    // For each row, divide both sides of ",", multiply them, and add them to `sum_of_products`
    for numbers in mul_params.lines() {
        let nums_to_mul: Vec<u64> = numbers.split(',').map(|num| num.parse().unwrap()).collect();
        sum_of_products += nums_to_mul[0] * nums_to_mul[1];
    }

    sum_of_products
}

#[cfg(test)]
mod tests {
    use crate::part_one::{calculate_product_of_mul_params, find_mul_params_from_file};
    use std::fs::File;

    #[test]
    fn test_find_mul_params_from_file() {
        let corrupted_memory_file = File::open("src/input.txt").expect("Error open file");
        assert_eq!(
            find_mul_params_from_file(corrupted_memory_file),
            "180,108\n345,132\n78,98\n23,25"
        );
    }

    #[test]
    fn test_calculate_product_of_mul_params() {
        let mul_params: String = String::from("180,108\n345,132");
        assert_eq!(calculate_product_of_mul_params(mul_params), 64980);
    }
}
