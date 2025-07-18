use std::io::{self}; // 표준 입력 모듈 가져오기

/// 수식을 숫자/연산자/괄호/등호 등으로 분리하는 함수
fn tokenize(expr: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();      // 최종 토큰 벡터
    let mut num_buf = String::new();  // 숫자 누적 버퍼

    for c in expr.chars() { // 수식을 한 글자씩 순회
        if c.is_ascii_digit() || c == '.' { // 숫자나 소수점이면
            num_buf.push(c);                // 숫자 버퍼에 누적
        } else {
            if !num_buf.is_empty() {        // 숫자 버퍼가 차있으면
                tokens.push(num_buf.clone()); // 숫자 토큰으로 추가
                num_buf.clear();              // 버퍼 초기화
            }
            if c.is_whitespace() { continue; } // 공백은 무시

            if "+-*/()=".contains(c) {         // 연산자/괄호/등호라면
                tokens.push(c.to_string());    // 토큰으로 추가
            } else {
                return Err(format!("알 수 없는 문자: '{}'", c)); // 예외 문자
            }
        }
    }

    if !num_buf.is_empty() {                  // 마지막 숫자 처리
        tokens.push(num_buf);
    }

    Ok(tokens) // 토큰 벡터 반환
}

/// 연산자 우선순위 지정
fn precedence(op: char) -> usize {
    match op {
        '+' | '-' => 1, // 낮은 우선순위
        '*' | '/' => 2, // 높은 우선순위
        _ => 0,          // 기타는 우선순위 없음
    }
}

/// 스택에서 연산자와 숫자 2개를 꺼내 실제 계산하는 함수
fn apply_op(nums: &mut Vec<f64>, ops: &mut Vec<char>) -> Result<(), String> {
    let op = ops.pop().ok_or("연산자 스택이 비어 있음")?; // 연산자 꺼내기
    let b  = nums.pop().ok_or("숫자 스택 부족")?;         // 뒤쪽 숫자
    let a  = nums.pop().ok_or("숫자 스택 부족")?;         // 앞쪽 숫자

    let res = match op { // 실제 계산
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0.0 { return Err("0으로 나눌 수 없습니다".into()); }
            a / b
        }
        _ => return Err(format!("지원하지 않는 연산자: {}", op)),
    };

    nums.push(res); // 결과 다시 숫자 스택에 넣기

    // 중간 결과 출력
    println!("  ▶ apply_op: {} {} {} = {}", a, op, b, res);
    println!("     nums stack: {:?}", nums);
    println!("     ops stack: {:?}\n", ops);

    Ok(())
}

/// 메인 함수
fn main() {
    println!("수식 입력 (예: 2+3*(4-1)=):"); // 안내 메시지

    let mut line = String::new(); // 사용자 입력 저장할 문자열
    if io::stdin().read_line(&mut line).is_err() {
        eprintln!("입력을 읽는 데 실패했습니다."); return;
    }

    let expr = line.trim(); // 앞뒤 공백 제거

    // 입력 문자열 → 토큰 분리
    let tokens = match tokenize(expr) {
        Ok(t) => t,
        Err(e) => { eprintln!("토큰화 오류: {}", e); return; }
    };
    println!("토큰: {:?}\n", tokens); // 토큰 출력

    let mut nums: Vec<f64> = Vec::new(); // 숫자 스택
    let mut ops:  Vec<char> = Vec::new(); // 연산자 스택
 
    // 토큰 순회하며 계산 수행
    for tok in tokens {
        if let Ok(n) = tok.parse::<f64>() { // 숫자일 경우
            nums.push(n);                   // 숫자 스택에 push
            println!("push number: {}", n);
            println!("  nums stack: {:?}\n", nums);
        } else if tok.len() == 1 {
            let c = tok.chars().next().unwrap();
            // 연산자/괄호/등호 처리
            // c는 단일 문자로 처리
            // 연산자 우선순위에 따라 계산

            match c {
                '+' | '-' | '*' | '/' => {
                    println!("process op: {}", c);
                    while !ops.is_empty() &&
                          precedence(*ops.last().unwrap()) >= precedence(c)
                          // c가 현재 스택의 마지막 연산자보다 우선순위가 낮거나 같을 때
                          // unwrap()는 안전하게 사용하기 위해
                          // 연산자 스택이 비어있지 않음을 보장

                    {
                        apply_op(&mut nums, &mut ops).unwrap(); // 우선순위 높은 것 먼저 계산

                        //unwrap()는 안전하게 사용하기 위해
                        // 연산자 스택이 비어있지 않음을 보장

                    }
                    ops.push(c); // 현재 연산자 push
                    println!("  pushed op: {}", c);
                    println!("  ops stack: {:?}\n", ops);
                }
                '(' => {
                    ops.push(c); // 여는 괄호는 무조건 push
                    println!("pushed '(': {:?}", ops);
                }
                ')' => {
                    println!("process ')'");
                    while *ops.last().unwrap() != '(' {
                        apply_op(&mut nums, &mut ops).unwrap(); // 괄호 안 계산
                    }
                    ops.pop(); // 여는 괄호 제거
                    println!("  popped '(', ops stack: {:?}\n", ops);
                }
                '=' => {
                    println!("process '=' → finalizing");
                    while !ops.is_empty() {
                        apply_op(&mut nums, &mut ops).unwrap(); // 남은 연산 모두 처리
                    }
                    break;
                }
                _ => {
                    eprintln!("알 수 없는 토큰: {}", c); return;
                }
            }
        } else {
            eprintln!("잘못된 토큰: {}", tok); return;
        }
    }

    // 최종 결과 출력
    if nums.len() == 1 {
        println!("최종 결과 = {}", nums[0]);
    } else {
        eprintln!("계산 오류: 남은 숫자 스택 = {:?}", nums);
    }
}



//  괄호 포함 테스트
// 2 * (3 + 4) = → (3 + 4 = 7 → 2 * 7 = 14)

// (8 + 2) * (5 - 3) = → (10 * 2 = 20)

// 50 / (2 + 3) = → 50 / 5 = 10

// ((2 + 3) * (4 + 1)) = → (5 * 5 = 25)

// ✅ 실수 및 소수점 포함
// 1.5 + 2.25 =

// 10.5 * 2.0 =

// 5.5 / 0.5 = → 결과: 11

// ✅ 복합 수식 (테스트용)
// 3 + 4 * 2 / (1 - 5) = → 우선순위와 괄호가 섞인 대표적인 예제 (결과: 1)

// ((1 + 2) * (3 + 4)) / 5 = → 21 / 5 = 4.2

// ((4 + 6) / 2) * (3 - 1) = → (10 / 2) * 2 = 10

// ❌ 예외 테스트 (에러 확인용)
// 10 / 0 = → 0으로 나눔 (오류 발생)

// 2 ++ 3 = → 연속된 연산자 (잘못된 토큰)

// 2 + (3 * 4 = → 괄호 짝이 안 맞음 (에러 발생)

