use std::{fs::File, io::Read};

pub fn run() -> std::io::Result<()> {
    // Open the puzzle input with the corrupted memory
    let corrupted_memory_file = File::open("src/input.txt")?;

    // Stores all do(), don't() and mul() functions in the order they are encountered
    let functions = find_functions_from_file(corrupted_memory_file);

    // Run all the logic for applying the mul() function
    let sum_of_products = run_functions(functions);

    println!("Sum of products: {sum_of_products}");

    Ok(())
}

fn find_functions_from_file(mut file: File) -> Vec<String> {
    let mut corrupted_memory = String::new();
    file.read_to_string(&mut corrupted_memory)
        .expect("Error parsing file buffer to string");

    // Regex to find do() OR don't() OR mul() functions
    let functions =
        regex::Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").expect("Error reading regex");

    // Run regex on text looking for the functions
    let functions: Vec<String> = functions
        .find_iter(corrupted_memory.as_str())
        .map(|f| f.as_str().to_string())
        .collect();

    functions
}

fn run_functions(functions: Vec<String>) -> u64 {
    let mut sum: u64 = 0;

    // Defines whether or not the next mul() function should be skipped
    let mut enabled: bool = true;

    // Set the `enabled` state and, if it is a mul(), sums the products of the parameters
    for function in functions {
        match function.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            // If it is a mul() function
            _ => {
                if enabled {
                    // Regex to search for numbers to be multiplied
                    let mul_params_regex =
                        regex::Regex::new(r"\d{1,3},\d{1,3}").expect("Error reading regex");

                    // Collect all correct mul params into a Vec - this looks like "999,999"
                    let mul_params: Vec<&str> = mul_params_regex
                        .find_iter(&function)
                        .map(|f| f.as_str())
                        .collect();

                    // Collect a Vec of &str into a String
                    mul_params.join("\n");

                    // Separates the vec by ",", converts it to u64 and adds the product to the sum
                    for numbers in mul_params {
                        let nums_to_mul: Vec<u64> =
                            numbers.split(',').map(|num| num.parse().unwrap()).collect();
                        sum += nums_to_mul[0] * nums_to_mul[1];
                    }
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::part_two::{find_functions_from_file, run_functions};
    use std::fs::File;

    #[test]
    fn test_find_functions_from_file() {
        let corrupted_memory_file = File::open("src/input.txt").expect("Error open file");

        assert_eq!(
            find_functions_from_file(corrupted_memory_file),
            [
                "do()",
                "mul(180,108)",
                "don't()",
                "mul(345,132)",
                "don't()",
                "mul(78,98)",
                "do()",
                "mul(23,25)"
            ]
        );
    }

    #[test]
    fn test_run_functions() {
        let functions = vec![
            "do()",
            "mul(180,108)",
            "don't()",
            "mul(345,132)",
            "don't()",
            "mul(78,98)",
            "do()",
            "mul(23,25)",
        ];

        let functions = functions.iter().map(|f| f.to_string()).collect();

        assert_eq!(run_functions(functions), 20015);
    }
}
