fn main() {
    // 변수
    let mut x = 5;
    println!("x:{x}");
    x = 6;
    println!("x:{x}");

    // 상수
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("상수의 값 : {THREE_HOURS_IN_SECONDS}");

    // 섀도잉
    let x = x + 1;
    {
        let x = x + 2;
        println!("안쪽 스코프에서 x의 값은... {x}");
    }

    println!("바깥쪽 스코프에서 x의 값은... {x}");

    // 만약 아래 코드가 mut 이었다면 에러가 발생했을 것.
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces : {spaces}")
}
