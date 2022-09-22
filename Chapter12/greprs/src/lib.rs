use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // 작은 변수에 대해서는 clone()을 만드는 것이 더 유리할 수도 있다.
        let query = args[1].clone();
        let filename = args[2].clone();
        
        // CASE_INSENSITIVE라는 환경변수를 설정할 수 있다.
        // $env:CASE_INSENSITIVE=1      for powershell
        // export CASE_INSENSITIVE=1      
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = 
    if config.case_sensitive {
        search(&config.query, &contents)
    } 
    else {
        search_case_insensitive(&config.query, &contents)
    };

    //println!은 test 모듈이 있기 때문에 필요가 없다.
    for line in results {
        println!("{}", line);
    }

    Ok(())
}


/*
    contents의 각 줄에 대한 반복작업
    해당 줄에 우리의 쿼리 문자열이 포함되어 있는지 검사
    그렇다면, 우리가 반환할 값 목록에 추가
    그렇지 않다면, 통과
    일치하는 결과 목록을 반환
*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
} 


//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// ";

//         assert_eq!(
//             vec!["safe, fast, productive."],
//             search(query, contents)
//         );
//     }

