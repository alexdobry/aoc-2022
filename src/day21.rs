use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
        }
    }
}

#[derive(Debug, Clone)]
enum Task {
    Const(i64),
    Bin(String, String, Operator),
}

type Monkeys = HashMap<String, Task>;

#[derive(Debug, Clone)]
enum Value {
    Const(i64),
    Humn,
    Bin(Box<Value>, Box<Value>, Operator),
}

fn read_input() -> Monkeys {
    let file_path = "inputs/day21.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Monkeys {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (name, task) = line.split_once(": ").unwrap();
        let task = if let Some((l, r)) = task.split_once(" + ") {
            Task::Bin(l.to_string(), r.to_string(), Operator::Add)
        } else if let Some((l, r)) = task.split_once(" - ") {
            Task::Bin(l.to_string(), r.to_string(), Operator::Sub)
        } else if let Some((l, r)) = task.split_once(" * ") {
            Task::Bin(l.to_string(), r.to_string(), Operator::Mul)
        } else if let Some((l, r)) = task.split_once(" / ") {
            Task::Bin(l.to_string(), r.to_string(), Operator::Div)
        } else {
            Task::Const(task.parse().unwrap())
        };
        monkeys.insert(name.to_string(), task);
    }

    monkeys
}

fn simplify_task(monkeys: &mut Monkeys, task: String) -> Value {
    if task == "humn" {
        return Value::Humn;
    }
    match monkeys.get(&task).unwrap().clone() {
        Task::Const(value) => Value::Const(value),
        Task::Bin(l, r, op) => match (simplify_task(monkeys, l), simplify_task(monkeys, r)) {
            (Value::Const(l), Value::Const(r)) => Value::Const(op.apply(l, r)),
            (l, r) => Value::Bin(Box::from(l), Box::from(r), op),
        },
    }
}

fn solve_task(monkeys: &mut Monkeys, task: Task) -> i64 {
    match task {
        Task::Const(value) => value,
        Task::Bin(l, r, op) => {
            let l_task = monkeys.get(&l).unwrap();
            let l_value = solve_task(monkeys, l_task.clone());
            let l_mut = monkeys.get_mut(&l).unwrap();
            *l_mut = Task::Const(l_value);

            let r_task = monkeys.get(&r).unwrap();
            let r_value = solve_task(monkeys, r_task.clone());
            let r_mut = monkeys.get_mut(&r).unwrap();
            *r_mut = Task::Const(r_value);

            op.apply(l_value, r_value)
        }
    }
}

fn solve_for_humn(value: Value, result: i64) -> i64 {
    match value {
        Value::Humn => result,
        Value::Bin(l, r, op) => match (*l, *r) {
            (Value::Const(l), r) => match op {
                Operator::Add => solve_for_humn(r, result - l),
                Operator::Sub => solve_for_humn(r, -(result - l)),
                Operator::Mul => solve_for_humn(r, result / l),
                Operator::Div => solve_for_humn(r, l / result),
            },
            (l, Value::Const(r)) => match op {
                Operator::Add => solve_for_humn(l, result - r),
                Operator::Sub => solve_for_humn(l, result + r),
                Operator::Mul => solve_for_humn(l, result / r),
                Operator::Div => solve_for_humn(l, result * r),
            },
            _ => unreachable!(),
        },
        Value::Const(_) => unreachable!(),
    }
}

pub fn solve1() {
    let mut monkeys = read_input();
    let root = monkeys.get("root").unwrap().clone();
    let result = solve_task(&mut monkeys, root);
    println!("{result}");
}

pub fn solve2() {
    let mut monkeys = read_input();
    let (l_val, r_val) = match monkeys.get("root").unwrap().clone() {
        Task::Bin(l, r, _) => (
            simplify_task(&mut monkeys, l),
            simplify_task(&mut monkeys, r),
        ),
        Task::Const(_) => unreachable!(),
    };

    if let Value::Const(r) = r_val {
        let result = solve_for_humn(l_val, r);
        println!("{result}")
    }
}
