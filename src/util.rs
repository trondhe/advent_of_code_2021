#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;

pub fn file2str(filename: &str) -> String {
    let mut file = File::open(filename).expect("file {} not found");
    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("error reading file");
    string
}

pub fn str2vec_digits(input: &str) -> Vec<usize> {
    let string = String::from(input);
    let mut vec: Vec<usize> = vec![];
    for c in string.chars() {
        if let Some(digit) = c.to_digit(10) {
            vec.push(digit as usize);
        }
    }
    vec
}

pub fn str2vec_numbers(input: &str) -> Vec<usize> {
    let mut vec: Vec<usize> = vec![];
    for line in input.lines() {
        if let Ok(digit) = line.parse::<usize>() {
            vec.push(digit as usize);
        }
    }
    vec
}

pub fn str2vecinception(input: &str) -> Vec<Vec<usize>> {
    let lines = str2linevec(&input);
    let vector = lines2vecvec(lines);
    vector
}

pub fn str2linevec(string: &str) -> Vec<&str> {
    let lines: Vec<&str> = string.split("\r\n").collect();
    lines
}

pub fn lines2vecvec(lines: Vec<&str>) -> Vec<Vec<usize>> {
    let mut vector: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let row: Vec<usize> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        vector.push(row);
    }
    vector
}

#[cfg(test)]
mod str2vec_test {
    use super::*;

    fn str2vec_assert(input: &str, answer: Vec<usize>) {
        let result = str2vec_digits(&input);
        assert_eq!(result, answer)
    }

    #[test]
    fn given_empty_return_empty() {
        str2vec_assert("", vec![])
    }

    #[test]
    fn assert_one() {
        str2vec_assert("1", vec![1])
    }

    #[test]
    fn assert_oneone() {
        str2vec_assert("11", vec![1, 1])
    }

    #[test]
    fn assert_stuff() {
        str2vec_assert("192837465", vec![1, 9, 2, 8, 3, 7, 4, 6, 5])
    }
}

#[cfg(test)]
mod str2vecinception_test {
    use super::*;
    // Tab seperated values, newline seperated rows

    #[test]
    fn assert_1x1() {
        let string = "525";
        let result = str2vecinception(string);
        let expected = [[525]];
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_1x2() {
        let string = "111 222\r\n333 444";
        let result = str2vecinception(string);
        let expected = [[111, 222], [333, 444]];
        assert_eq!(result, expected);
    }

    fn assert_3x3() {
        let string = "1 2\r\n3\r\n4 5";
        let result = str2vecinception(string);
        let expected = vec![vec![1, 2], vec![3], vec![4, 5]];
        assert_eq!(result, expected);
    }
}
