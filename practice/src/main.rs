
/* ### 🟢 문제 1: 간단한 나눗셈 함수 (기초)

숫자 두 개를 받아서 나누는 함수를 작성하세요. 단, **0으로 나누는 경우**에는 에러를 반환해야 합니다.

* **함수명**: `safe_divide`
* **반환 타입**: `Result<f64, String>`
* **조건**:
* 분모가 0이면 `Err("0으로 나눌 수 없습니다".to_string())` 반환
* 성공하면 `Ok(결과값)` 반환


* **연습 포인트**: `main` 함수에서 `match`를 사용해 결과가 `Ok`일 때와 `Err`일 때 각각 다른 메시지를 출력하세요.
 */
fn safe_devide(a:f64, b:f64) -> Result<f64, String> {

    if b == 0.0 {
        return Err("0으로 나눌 수 없습니다.".to_string());
    }
    let result = a / b;

    Ok(result)
}


fn main() {

    let a = 10.0;
    let b = 0.0;

    let result = safe_devide(a, b);

    match result {
        Ok(n) => println!{"결과 : {}", n},
        Err(e) => println!("에러 : {}", e),
    }
}

