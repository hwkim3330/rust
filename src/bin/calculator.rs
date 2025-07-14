use std::io::{self}; // 표준 입출력 모듈 사용

/// 사용자에게 메시지를 출력하고 문자열 입력을 받는 함수
fn read_line(prompt: &str) -> String {
    loop {
        println!("{prompt}"); // 프롬프트 메시지 출력

        let mut buf = String::new(); // 사용자 입력을 저장할 버퍼
        if io::stdin().read_line(&mut buf).is_ok() {
            let trimmed = buf.trim(); // 앞뒤 공백 제거
            if !trimmed.is_empty() {
                return trimmed.to_string(); // 문자열 반환
            }
        }

        // 입력 오류 또는 빈 입력인 경우 경고
        eprintln!("입력 오류, 다시 시도하세요.");
    }
}

/// 실수(f64)를 입력받는 함수
fn read_num(prompt: &str) -> f64 {
    loop {
        // 문자열 입력 → 실수로 파싱
        match read_line(prompt).parse::<f64>() {
            Ok(n) => return n, // 파싱 성공 시 반환
            Err(_) => eprintln!("숫자를 정확히 입력하세요."), // 실패 시 경고
        }
    }
}

/// 사칙연산자(+ - * /)를 입력받는 함수
fn read_op() -> char {
    loop {
        // 첫 글자를 추출해 연산자인지 확인
        match read_line("연산자 입력 (+ - * /):").chars().next() {
            Some(op @ ('+' | '-' | '*' | '/')) => return op, // 유효한 연산자면 반환
            _ => eprintln!("유효한 연산자를 입력하세요."), // 유효하지 않으면 경고
        }
    }
}

/// 두 숫자와 연산자를 받아 계산 결과를 반환하는 함수
fn calc(a: f64, b: f64, op: char) -> Option<f64> {
    Some(match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' if b != 0.0 => a / b, // 0 나누기 방지
        _ => return None,
    })
}

/// 메인 함수
fn main() {
    println!("Rust CLI 계산기 프로그램");

    loop {
        println!(); // 줄바꿈
        println!("숫자 입력 단계");

        let a = read_num("첫 번째 숫자:");
        let b = read_num("두 번째 숫자:");

        println!();
        println!("연산자 선택 단계");

        let op = read_op();

        println!();
        println!("계산 결과");

        println!("{a} {op} {b} = ");
        match calc(a, b, op) {
            Some(res) => println!("{res}"),
            None => println!("계산 실패 (0으로 나눔 등)"),
        }

        println!();
        let again = read_line("계속하시겠습니까? (y/N):");

        if again.to_lowercase() != "y" {
            println!("프로그램을 종료합니다.");
            break;
        }
    }
}
