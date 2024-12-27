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

pub fn get_neighbours(x: usize, y: usize, grid: &[Vec<u32>]) -> [Option<(usize, usize)>; 4] {
    let mut out = [None; 4];

    let mut tail = 0;
    let ys = if y == 0 {
        vec![y + 1]
    } else {
        vec![y - 1, y + 1]
    };

    let xs = if x == 0 {
        vec![x + 1]
    } else {
        vec![x - 1, x + 1]
    };

    for y in ys {
        out[tail] = match grid.get(y) {
            Some(row) => row.get(x).map(|_| (x, y)),
            None => None,
        };
        if out[tail].is_some() {
            tail += 1;
        }
    }

    for x in xs {
        out[tail] = match grid.get(y) {
            Some(row) => row.get(x).map(|_| (x, y)),
            None => None,
        };
        if out[tail].is_some() {
            tail += 1;
        }
    }

    out
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a_prime = a;
    let mut b_prime = b;

    while b_prime != 0 {
        let t = b_prime;
        b_prime = a_prime % b_prime;
        a_prime = t;
    }

    a_prime
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}
