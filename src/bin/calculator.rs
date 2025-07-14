use std::io::{self, Write}; // 표준 입출력 모듈 가져오기

/// 사용자로부터 문자열 한 줄을 입력받는 함수
fn read_line(prompt: &str) -> String {
    loop {
        // 프롬프트 출력 및 버퍼 플러시
        print!("{prompt} ");
        io::stdout().flush().expect("출력 실패"); // panic 방지를 위해 expect 사용

        let mut buf = String::new(); // 입력을 받을 버퍼 생성

        // 입력을 읽고 성공하면 공백 제거 후 반환
        if io::stdin().read_line(&mut buf).is_ok() {
            let trimmed = buf.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string(); // 정상 입력 반환
            }
        }

        // 입력 실패 또는 빈 입력일 경우 경고 메시지 출력
        eprintln!("입력 오류, 다시 시도하세요.");
    }
}

/// 실수(f64)를 입력받는 함수
fn read_num(prompt: &str) -> f64 {
    loop {
        // read_line으로 입력받은 문자열을 f64로 파싱
        match read_line(prompt).parse::<f64>() {
            Ok(n) => return n, // 성공하면 반환
            Err(_) => eprintln!("숫자를 정확히 입력하세요."), // 실패 시 경고
        }
    }
}

/// 연산자 (+, -, *, /) 하나를 입력받는 함수
fn read_op() -> char {
    loop {
        // 입력값에서 첫 글자만 추출하여 유효한 연산자인지 확인
        match read_line("연산자 입력 (+ - * /):").chars().next() {
            Some(op @ ('+' | '-' | '*' | '/')) => return op, // 유효하면 반환
            _ => eprintln!("유효한 연산자를 입력하세요."), // 아니면 경고
        }
    }
}

/// 두 숫자와 연산자를 받아 계산 결과를 반환하는 함수
fn calc(a: f64, b: f64, op: char) -> Option<f64> {
    // op에 따라 연산을 수행하고 Some(result) 반환, 실패 시 None
    Some(match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' if b != 0.0 => a / b, // 0으로 나눌 수 없음
        _ => return None, // 잘못된 연산자나 0으로 나눌 경우
    })
}

/// 메인 함수 - 프로그램 실행의 시작점
fn main() {
    println!("Rust CLI 계산기 프로그램");

    // 반복적으로 계산기 기능 수행
    loop {
        println!(); // 줄바꿈
        println!("숫자 입력 단계");

        let a = read_num("첫 번째 숫자:"); // 첫 번째 숫자 입력
        let b = read_num("두 번째 숫자:"); // 두 번째 숫자 입력

        println!(); // 줄바꿈
        println!("연산자 선택 단계");

        let op = read_op(); // 연산자 입력

        println!(); // 줄바꿈
        println!("계산 결과");

        // 수식과 함께 결과 출력
        print!("{a} {op} {b} = ");
        match calc(a, b, op) {
            Some(res) => println!("{res}"), // 정상 결과 출력
            None => println!("계산 실패 (0으로 나눔 등)"), // 실패 메시지
        }

        println!(); // 줄바꿈
        let again = read_line("계속하시겠습니까? (y/N):");

        // 'y' 또는 'Y' 입력 시 반복, 그 외 종료
        if again.to_lowercase() != "y" {
            println!("프로그램을 종료합니다.");
            break;
        }
    }
}
