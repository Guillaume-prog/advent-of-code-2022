use std::collections::HashMap;

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Operator {
    fn to_string(&self) -> &'static str {
        match *self {
            Operator::Add      => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide   => "/"
        }
    }
}

enum MonkeyYell {
    Number(MonkeyYellValue),
    Operation(&'static str, Operator, &'static str)
}

type MonkeyYellValue = u64;
type MonkeyMap = HashMap<&'static str, MonkeyYell>;

fn main() {
    let input = include_str!("../input.txt");
    let mut monkeys = load_monkeys(input);

    // Part 1
    let root = compute_monkey_yell(&monkeys, "root");
    println!("root: {}", root);

    // Part 2
    // println!("===");

    let path = find_human_path(&monkeys);
    // println!("path: {}\n===", path.join(" -> "));

    let human = compute_human_value(&monkeys, &path, false);
    // println!("===");
    println!("humn: {}", human);
    println!("Valid answer: {}", is_valid_answer(&mut monkeys, human));
}

fn is_valid_answer(monkeys: &mut MonkeyMap, value: MonkeyYellValue) -> bool {
    monkeys.remove("humn");
    monkeys.insert("humn", MonkeyYell::Number(value));

    if let MonkeyYell::Operation(left, _, right) = monkeys.get("root").unwrap() {
        let left_val = compute_monkey_yell(monkeys, *left);
        let right_val = compute_monkey_yell(monkeys, *right);

        return left_val == right_val;
    }
    unreachable!("Root shouldn't be a number");
}

fn compute_human_value(monkeys: &MonkeyMap, path: &Vec<&'static str>, verbose: bool) -> MonkeyYellValue {
    let mut value: MonkeyYellValue = 0;
    for monkey_name in path.clone() {
        if let MonkeyYell::Operation(left, op, right) = monkeys.get(monkey_name).unwrap() {
            let pcl = path.contains(left);
            
            let other_value = match pcl {
                true  => compute_monkey_yell(monkeys, right),
                false => compute_monkey_yell(monkeys, left)
            };

            if monkey_name == "root" {
                if verbose { println!("{} must equal {}", if pcl { left } else { right }, other_value); }
                value = other_value;
                continue;
            }

            value = match *op {
                Operator::Add      => value - other_value,
                Operator::Multiply => value / other_value,
                Operator::Subtract => if pcl { value + other_value } else { other_value - value },
                Operator::Divide   => if pcl { value * other_value } else { other_value / value }
            };

            // Display
            if verbose {
                let l = if pcl { left.to_string() } else { other_value.to_string() };
                let r = if pcl { other_value.to_string() } else { right.to_string() };
                let var = if pcl { left } else { right };
                println!("{} must equal {} [ {}: {} {} {} ]", var, value, monkey_name, l, op.to_string(), r);
            }
        }
    }

    value as MonkeyYellValue
}

// Recursive function to search for the path leading to 
fn find_human_path(monkeys: &MonkeyMap) -> Vec<&'static str> {
    fn recursive_search(monkeys: &MonkeyMap, name: &'static str, path: &mut Vec<&'static str>) -> Option<Vec<&'static str>> {
        path.push(name);
        if name == "humn" { return Some(path.to_vec()); }

        match monkeys.get(name).unwrap() {
            MonkeyYell::Number(_) => {
                path.pop();
                return None;
            },
            MonkeyYell::Operation(left, _, right) => {
                let left_path = recursive_search(monkeys, left, path);
                let right_path = recursive_search(monkeys, right, path);

                if left_path.is_some()       { return left_path; }
                else if right_path.is_some() { return right_path; }
                else                         { path.pop(); return None; }
            }
        }
    }

    recursive_search(monkeys, "root", &mut Vec::new()).unwrap()
}

fn compute_monkey_yell(monkeys: &MonkeyMap, name: &'static str) -> MonkeyYellValue {
    match monkeys.get(name).unwrap() {
        MonkeyYell::Number(number) => *number,
        MonkeyYell::Operation(name_1, op, name_2) => {
            let val_1 = compute_monkey_yell(monkeys, *name_1);
            let val_2 = compute_monkey_yell(monkeys, *name_2);

            match *op {
                Operator::Add      => val_1 + val_2,
                Operator::Subtract => val_1 - val_2,
                Operator::Multiply => val_1 * val_2,
                Operator::Divide   => val_1 / val_2
            }
        }
    }
}

fn load_monkeys(input: &'static str) -> MonkeyMap {
    let mut monkeys: MonkeyMap = HashMap::new();

    for line in input.lines() {
        let (name, op) = line.split_once(": ").unwrap();

        let yell: MonkeyYell;
        if let Ok(number) = op.parse::<MonkeyYellValue>() {
            yell = MonkeyYell::Number(number);
        } else {
            let tokens: Vec<_> = op.split(" ").collect();
            let operator = match tokens[1] {
                "+" => Operator::Add,
                "-" => Operator::Subtract,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                tok => unreachable!("Invalid token: {}", tok)
            };

            yell = MonkeyYell::Operation(tokens[0], operator, tokens[2]);
        }
        
        monkeys.insert(name, yell);
    }

    monkeys
}