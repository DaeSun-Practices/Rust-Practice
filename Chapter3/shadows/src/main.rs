fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2; //각각의 let에서 새로운 변수가 선언되어 bound된다고 보면 된다.

    println!("The value of x is: {}", x);

    
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    //데이터의 타입이 다르기에 mut로는 사용이 안 된다.
    //let mut spaces = "   ";
    //spaces = spaces.len();

    
    //데이터 타입들
    //기본 타입은 f64인데, 그 이유는 최신의 CPU 상에서는 f64가 f32와 대략 비슷한 속도를 내면서도 더 정밀한 표현이 가능하기 때문입니다.
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    //연산들
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;


    // boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation

    //문자 타입
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';


    //복합 타입!
    // 튜플
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 복합 타입을 해체하기
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // 복합 타입 인덱스 별로 접근하기
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


    // 배열 알아보기
    // 배열은 전부 같은 타입이어야 하고, 길이가 늘어나거나 줄어들지 않는다.
    let a = [1, 2, 3, 4, 5];

    // 배열은 stack 메모리에 위치하기에 메모리 주소로 접근이 가능하다.
    let first = a[0];
    let second = a[1];

    
}
