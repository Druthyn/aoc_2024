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

pub mod grid {
    use std::{
        fmt::{Debug, Display}, ops::Deref
    };

    #[derive(Debug)]
    pub struct Grid<T>(Vec<Vec<T>>);

    impl<T> Grid<T>
    where
        T: Copy,
    {
        pub fn new(inner: Vec<Vec<T>>) -> Self {
            Grid(inner)
        }

        pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
            match self.0.get(y) {
                Some(row) => row.get(x),
                None => None,
            }
        }

        pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut T> {
            match self.0.get_mut(y) {
                Some(row) => row.get_mut(x),
                None => None,
            }
        }

        pub fn swap(&mut self, lhs: (usize, usize), rhs: (usize, usize)) -> bool {
            match self.get_mut(lhs) {
                Some(&mut l) => match self.get_mut(rhs) {
                    Some(&mut r) => {
                        self.0[lhs.1][lhs.0] = r;
                        self.0[rhs.1][rhs.0] = l;
                        true
                    }
                    None => false,
                },
                None => false,
            }
        }

        pub fn get_neighbours_4(&self, center: (usize, usize)) -> Vec<(T, (usize, usize))> {
            let mut out = vec![];

            let xs = [center.0.wrapping_sub(1), center.0.saturating_add(1)];
            let ys = [center.1.wrapping_sub(1), center.1.saturating_add(1)];

            for x in xs {
                if let Some(&v) = self.get((x, center.1)) {
                    out.push((v, (x, center.1)))
                }
            }

            for y in ys {
                if let Some(&v) = self.get((center.0, y)) {
                    out.push((v, (center.0, y)));
                }
            }

            out
        }

        pub fn get_neighbours_8(&self, center: (usize, usize)) -> Vec<(T, (usize, usize))> {
            let mut out = vec![];

            let xs = center.0.wrapping_sub(1)..center.0.saturating_add(1);

            for x in xs {
                let ys = center.1.wrapping_sub(1)..center.1.saturating_add(1);
                for y in ys {
                    if (x, y) == center {
                        continue;
                    }
                    if let Some(&v) = self.get((x, y)) {
                        out.push((v, (x, y)));
                    }
                }
            }

            out
        }
    }

    impl<T> Deref for Grid<T> {
        type Target = Vec<Vec<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug + Display> Display for Grid<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for line in &self.0 {
                for item in line {
                    write!(f, "{item}")?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
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
