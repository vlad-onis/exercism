#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn compute_stack_value(stack: &mut Vec<i32>, op: &CalculatorInput) -> i32 {
    
    let op1 = stack.pop().unwrap();
    let op2 = stack.pop().unwrap();

    let res = match op {
        CalculatorInput::Add => {
            op2 + op1
        },
        CalculatorInput::Subtract => {
            op2 - op1
        },
        CalculatorInput::Multiply => {
            op2 * op1
        },
        CalculatorInput::Divide => {
            op2 / op1
        }
        CalculatorInput::Value(_) => { 0 },
    };

    res
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    
    if inputs.len() == 0 {
        return None;
    }

    match inputs[0] {
        CalculatorInput::Value(_) => {},
        _ => { return None; },
    }

    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        if let CalculatorInput::Value(v) = input {
            stack.push(*v);
        }else {
            if stack.len() < 2 { return None; }
            let res = compute_stack_value(&mut stack, input);
            stack.push(res);
        }
    }

    
    if stack.len() > 1 || stack.len() == 0 {
        return None;
    }

    stack.pop()

}
