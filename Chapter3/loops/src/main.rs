fn main() {
    //loop  반복문
    loop {
        println!("again!");
        break
    }

    //while 반복문
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    //for 반복문
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //for 반복문에서의 배열을 역순으로 점검하는 법
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
