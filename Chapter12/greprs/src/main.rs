extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    // arguments들을 string 형식으로 가져오기
    let args: Vec<String> = env::args().collect();

    // argument분석 함수 호출
    let config = Config::new(&args).unwrap_or_else(|err| {
        //println!은 test 모듈이 있기 때문에 필요가 없다.
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!은 test 모듈이 있기 때문에 필요가 없다.
    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        //println!은 test 모듈이 있기 때문에 필요가 없다.
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// Command : $ cargo run test poem.txt



// use std::fs::File;
// use std::io::prelude::*;
// use std::error::Error;



// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     // Result를 이용한 예외처리
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }

//     // fn new(args: &[String]) -> Config {

//     //     //panic!을 이용한 예외처리. argument가 3개 이상이 아니면 안됨
//     //     if args.len() < 3 {
//     //         panic!("not enough arguments, argument number is {}", args.len());
//     //     }
    
//     //     let query = args[1].clone();
//     //     let filename = args[2].clone();

//     //     Config { query, filename }
//     // }
// }

// // Config 생성자로 이 함수의 기능을 대체한다.
// // fn parse_config(args: &[String]) -> Config {
// //     //각각에 argument부여하기
// //     let query = args[1].clone();
// //     let filename = args[2].clone();

// //     Config { query, filename }
// // }


// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let mut f = File::open(config.filename)?;

//     let mut contents = String::new();
//     f.read_to_string(&mut contents)?;

//     println!("With text:\n{}", contents);

//     Ok(())
// }


// fn main() {
    
//     let args: Vec<String> = env::args().collect();

    
//     let config : Config = Config::new(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.filename);


//     if let Err(e) = run(config) {
//         println!("Application error: {}", e);

//         process::exit(1);
//     }
// }





