// from_str.rs

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    Empty,
    BadLen,
    NoName,
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 1. 如果输入为空（包括只含空白字符），返回 Empty 错误
        if s.trim().is_empty() {
            return Err(ParsePersonError::Empty);
        }

        // 2. 按逗号分割
        let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();

        // 3. 必须正好有两个字段，否则 BadLen
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        // 4. 取出名字
        let name = parts[0];
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        // 5. 取出年龄并解析
        let age_str = parts[1];
        let age = age_str
            .parse::<usize>()
            .map_err(ParsePersonError::ParseInt)?;  // 解析失败 → ParseInt 错误

        // 6. 都成功了，构造 Person
        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}