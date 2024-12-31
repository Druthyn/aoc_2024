use std::collections::HashMap;

use advent_of_code::split_input_at_emptyline;

advent_of_code::solution!(24);

#[derive(Clone, Copy)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn from_str(input: &str) -> Self {
        match input {
            "AND" => Self::And,
            "XOR" => Self::Xor,
            "OR" => Self::Or,
            _ => panic!(),
        }
    }

    fn operate<'a>(self, a: bool, b: bool) -> NodeAux<'a> {
        NodeAux::Value(match self {
            Operator::And => a & b,
            Operator::Or => a | b,
            Operator::Xor => a ^ b,
        })
    }
}

#[derive(Clone, Copy)]
enum NodeAux<'a> {
    Label(&'a str),
    Value(bool),
}

#[derive(Clone, Copy)]
struct BinaryExpr<'a> {
    op: Operator,
    lhs: NodeAux<'a>,
    rhs: NodeAux<'a>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let parts = split_input_at_emptyline(input);

    let mut assignments = HashMap::new();

    for line in &parts[0] {
        let lhs = &line[0..3];
        let rhs = line.chars().last()? == '1';
        assignments.insert(lhs, NodeAux::Value(rhs));
    }

    let mut expressions = HashMap::new();
    let mut endpoints = HashMap::new();

    for line in &parts[1] {
        // ntg XOR fgs -> mjb
        let pieces: Vec<_> = line.split(' ').collect();
        let node = BinaryExpr {
            op: Operator::from_str(pieces[1]),
            lhs: NodeAux::Label(pieces[0]),
            rhs: NodeAux::Label(pieces[2]),
        };

        if pieces[4][0..=0] == *"z" {
            endpoints.insert(pieces[4], node);
        } else {
            expressions.insert(pieces[4], node);
        }
    }

    loop {
        let mut removals = vec![];
        for &label in expressions.keys() {
            let expr = expressions.get(label).unwrap();
            let mut expr = *expr;

            if let NodeAux::Label(a) = expr.lhs {
                if let Some(&v) = assignments.get(a) { expr.lhs = v }
            }

            if let NodeAux::Label(b) = expr.rhs {
                if let Some(&v) = assignments.get(b) { expr.rhs = v }
            }

            if let NodeAux::Value(a) = expr.lhs {
                if let NodeAux::Value(b) = expr.rhs {
                    assignments.insert(label, expr.op.operate(a, b));
                    removals.push(label);

                }
            }


        }

        for l in removals {
            expressions.remove(l);
        }

        if expressions.is_empty() {
            break;
        }
    }

    let mut out = 0;

    for expr in endpoints.keys() {
        let bin_op = endpoints.get(expr).unwrap();
        let NodeAux::Label(lhs) = bin_op.lhs else { panic!() };
        let NodeAux::Label(rhs) = bin_op.rhs else { panic!() };

        let NodeAux::Value(a) = assignments.get(lhs).unwrap() else {panic!()};
        let NodeAux::Value(b) = assignments.get(rhs).unwrap() else {panic!()};

        let NodeAux::Value(res) = bin_op.op.operate(*a, *b) else {panic!()};

        if res {
            out += 1 << expr[1..].parse::<u64>().unwrap();
        }
    }
    Some(out)
}


pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
