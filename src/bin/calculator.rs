use std::io::{self, Write};

/// 한 줄 입력 → String (공백 제거)
fn read_line(prompt: &str) -> String {
    loop {
        print!("{prompt} ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_ok() {
            let trimmed = buf.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
        eprintln!("⚠️  입력 오류, 다시 시도하세요.");
    }
}

/// 숫자(f64) 입력
fn read_num(prompt: &str) -> f64 {
    loop {
        match read_line(prompt).parse::<f64>() {
            Ok(n) => return n,
            Err(_) => eprintln!("⚠️  숫자를 정확히 입력하세요."),
        }
    }
}

/// 연산자 입력 (+ - * /)
fn read_op() -> char {
    loop {
        match read_line("연산자 (+ - * /):").chars().next() {
            Some(op @ ('+' | '-' | '*' | '/')) => return op,
            _ => eprintln!("⚠️  잘못된 연산자입니다."),
        }
    }
}

/// 사칙연산 수행
fn calc(a: f64, b: f64, op: char) -> Option<f64> {
    Some(match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' if b != 0.0 => a / b,
        _ => return None,
    })
}

fn main() {
    println!("🧮 Rust CLI 계산기");

    loop {
        let a = read_num("첫 번째 숫자:");
        let b = read_num("두 번째 숫자:");
        let op = read_op();

        match calc(a, b, op) {
            Some(res) => println!("= {res}"),
            None => println!("⚠️  계산 실패 (0으로 나눔 등)"),
        }

        if read_line("계속하시겠습니까? (y/N):").to_lowercase() != "y" {
            println!("👋 종료합니다. 감사합니다!");
            break;
        }
    }
}
