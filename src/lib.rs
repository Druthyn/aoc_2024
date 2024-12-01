pub mod template;

// Use this file to add helper functions and additional modules.

pub fn split_file_into_int_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let parsed_input = input.split_ascii_whitespace()
                                                        .map(|x| x.parse().unwrap());
    let left: Vec<u32> = parsed_input.clone()
                                            .step_by(2)
                                            .collect();
    let right: Vec<u32> = parsed_input.skip(1)
                                            .step_by(2)
                                            .collect();
    (left, right)
}
