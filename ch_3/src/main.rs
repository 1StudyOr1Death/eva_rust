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
    println!("spaces : {spaces}");

    // let str:char = "z"; // error - 문자열 리터럴
    let _str: char = 'z'; // char 타입은 작은 따옴표

    // 튜플
    let tup = (1, 2, 3);
    let (tu1, tu2, tu3) = tup;
    println!("tuples: {tu1} {tu2} {tu3}");

    // 배열
    let _arr = [1, 2, 3, 4, 5]; // 길이 및 타입 고정
    let _same_arrs = [3, 5]; //[3,3,3,3,3]

    // 함수
    let results = another_function(1);
    println!("result of another function: {results}");

    test_conditionals();
    test_loops();
    test_loops_label();
    test_loops_while();
}

fn another_function(x: i32) -> i32 {
    println!("another function run. arg: {x}");

    5
}

fn test_conditionals() {
    let number = 7;

    if number < 5 {
        println!("조건문은 참")
    } else {
        println!("조건문은 거짓")
    }

    // if는 표현식이다.
    let condition = true;
    // 변수가 가질 수 있는 타입은 오직 하나이기 때문에, 서로 다른 타입의 표현식이 들어가면 오류가 남.
    // 이유는 컴파일 시점에 변수의 타입을 확실히 알아야 하기 때문에.
    let another_number = if condition { 5 } else { 6 };

    println!("result of another_number is : {another_number}")
}

fn test_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("loop result {result}")
}

fn test_loops_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn test_loops_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("liftoff");
}
