use interpreter::Interpreter;
use std::io;
use std::io::Write;

pub fn call_op(interpreter: &mut Interpreter, color_value: (i32, i32)) {
    match color_value {
        (0, 1) => push(interpreter),
        (0, 2) => pop(interpreter),
        (1, 0) => add(interpreter),
        (1, 1) => subtract(interpreter),
        (1, 2) => multiply(interpreter),
        (2, 0) => divide(interpreter),
        (2, 1) => modulo(interpreter),
        (2, 2) => not(interpreter),
        (3, 0) => greater(interpreter),
        (3, 1) => pointer(interpreter),
        (3, 2) => switch(interpreter),
        (4, 0) => duplicate(interpreter),
        (4, 1) => roll(interpreter),
        (4, 2) | (5, 0) => input(interpreter),
        (5, 1) => out_number(interpreter),
        (5, 2) => out_char(interpreter),
        _ => (),
    }
}

fn pop_two(interpreter: &mut Interpreter) -> Option<(i32, i32)> {
    let a = match interpreter.stack.pop() {
        Some(n) => n as i32,
        None => return None,
    };

    let b = match interpreter.stack.pop() {
        Some(n) => n as i32,
        None => return None,
    };

    Some((a, b))
}

fn push(interpreter: &mut Interpreter) {
    // println!("push: {:?}", interpreter.stack);
    // println!("current_size: {:?}", interpreter.current_size);
    interpreter.stack.push(interpreter.current_size as i32);
}

fn pop(interpreter: &mut Interpreter) {
    // println!("pop: {:?}", interpreter.stack);

    interpreter.stack.pop();
}

fn add(interpreter: &mut Interpreter) {
    // println!("add: {:?}", interpreter.stack);

    if let Some((a, b)) = pop_two(interpreter) {
        interpreter.stack.push(a + b);
    }
}

fn subtract(interpreter: &mut Interpreter) {
    // println!("subtract: {:?}", interpreter.stack);

    if let Some((a, b)) = pop_two(interpreter) {
        interpreter.stack.push(b - a);
    }
}

fn multiply(interpreter: &mut Interpreter) {
    // println!("multiply: {:?}", interpreter.stack);

    if let Some((a, b)) = pop_two(interpreter) {
        interpreter.stack.push(a * b);
    }
}

fn divide(interpreter: &mut Interpreter) {
    // println!("divide: {:?}", interpreter.stack);

    if let Some((a, b)) = pop_two(interpreter) {
        interpreter.stack.push(b / a);
    }
}

fn modulo(interpreter: &mut Interpreter) {
    // println!("modulo: {:?}", interpreter.stack);

    if let Some((a, b)) = pop_two(interpreter) {
        interpreter.stack.push(b % a);
    }
}

fn not(interpreter: &mut Interpreter) {
    // println!("not: {:?}", interpreter.stack);

    if let Some(n) = interpreter.stack.pop() {
        if n == 0 {
            interpreter.stack.push(1);
        } else {
            interpreter.stack.push(0);
        }
    }
}

fn greater(interpreter: &mut Interpreter) {
    // println!("greater: {:?}", interpreter.stack);

    if let Some((a, b)) = pop_two(interpreter) {
        if b > a {
            interpreter.stack.push(1);
        } else {
            interpreter.stack.push(0);
        }
    }
}

fn pointer(interpreter: &mut Interpreter) {
    // println!("pointer: {:?}", interpreter.stack);

    if let Some(n) = interpreter.stack.pop() {
        for _ in 1..n.abs() + 1 {
            if n > 0 {
                interpreter.step_dp();
            } else {
                interpreter.step_dp_counter();
            }
        }
    }
}

fn switch(interpreter: &mut Interpreter) {
    // println!("switch: {:?}", interpreter.stack);

    if let Some(n) = interpreter.stack.pop() {
        for _ in 1..n.abs() + 1 {
            interpreter.codel_chooser.toggle();
        }
    }
}

fn duplicate(interpreter: &mut Interpreter) {
    // println!("duplicate: {:?}", interpreter.stack);

    let x = match interpreter.stack.last() {
        Some(n) => *n,
        None => return (),
    };

    interpreter.stack.push(x);
}

fn roll(interpreter: &mut Interpreter) {
    // println!("roll: {:?}", interpreter.stack);

    if let Some((rolls, depth)) = pop_two(interpreter) {
        let neg_depth = interpreter.stack.len() as i32 - depth;

        for _ in 1..rolls.abs() + 1 {
            if rolls > 0 {
                let x = interpreter.stack.pop().unwrap();
                interpreter.stack.insert(neg_depth as usize, x);
            } else {
                let x = interpreter.stack.remove(neg_depth as usize);
                interpreter.stack.push(x);
            }
        }
    }
}

fn input(interpreter: &mut Interpreter) {
    // println!("in number: {:?}", interpreter.stack);

    print!("Please type a number: ");

    io::stdout().flush().unwrap();

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: i32 = value.trim().parse().expect("Please type a number!");

    interpreter.stack.push(value);
}

fn out_number(interpreter: &mut Interpreter) {
    // println!("out number: {:?}", interpreter.stack);

    if let Some(n) = interpreter.stack.pop() {
        println!("{}", n);
    }
}

fn out_char(interpreter: &mut Interpreter) {
    // println!("out char: {:?}", interpreter.stack);

    if let Some(n) = interpreter.stack.pop() {
        println!("{}", char::from(n as u8));
    }
}
