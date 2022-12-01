use std::io::{self, Write};


enum Operator {
    Add,
    Sub,
    Div,
    Mul,
}

enum Token {
    Op(Operator),
    Number(i32)
}

fn lex(elements: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    for element in elements.split_whitespace().filter(|e| !e.is_empty()){
        match element {
            "+" => tokens.push(Token::Op(Operator::Add)),
            "*" => tokens.push(Token::Op(Operator::Mul)),
            "-" => tokens.push(Token::Op(Operator::Sub)),
            "/" => tokens.push(Token::Op(Operator::Div)),
            n => if let Ok(n) = n.parse::<i32>() {
                tokens.push(Token::Number(n));
            } else {
                return Err(format!("Invalid token {}", n));
            }
            
        }
    }

    Ok(tokens)
}

fn eval(expression: &[Token], stack: &mut Vec<i32>) -> Result<Option<i32>, &'static str> {
    let mut result = None;
    for token in expression.iter() {
        match token {
            Token::Op(operator) => {
                if stack.len() < 2 {
                    return Err("Not enough numbers in stack")
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                result = match operator {
                    Operator::Add => Some(a + b),
                    Operator::Sub => Some(a - b),
                    Operator::Mul => Some(a * b),
                    Operator::Div => Some(a / b),
                };
                stack.push(result.unwrap());
            },
            Token::Number(n) => {
                stack.push(*n);
            },
        };
    };
    Ok(result)
}

fn main() {
    let mut stack: Vec<i32> = Vec::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .unwrap();
        if line.trim() == "exit" {
            break;
        }
        let mem = match lex(&line) {
            Ok(value) => value,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        match eval(&mem, &mut stack) {
            Ok(Some(result)) => println!("< {}", result),
            Ok(None) => (),
            Err(e) => println!("Error: {}", e),
        }
    }
}
