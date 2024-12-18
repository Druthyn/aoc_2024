pub mod template;

// Use this file to add helper functions and additional modules.
pub fn split_input_at_emptyline(input: &str) -> Vec<Vec<&str>> {
    let mut out = vec![vec![]];
    let mut tail = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            tail += 1;
            out.push(vec![])
        } else {
            out[tail].push(line);
        }
    }

    out
}
