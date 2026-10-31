use crate::CheckError::ErrorAddition;
use thiserror::Error;

#[derive(PartialEq)]
enum MyError {
    CharNotASCII,
    CharNotDIGIT,
    CharNotBASE16,
    CharNotLETTER,
    CharNotPRINTABLE,
}

#[derive(PartialEq)]
enum CheckError {
    ErrorAddition,
    ErrorMultiplication,
}

#[derive(Error, Debug, PartialEq)]
enum MyErrorBONUS {
    #[error("Err: This Char is not ASCII")]
    CharNotASCII,
    #[error("Err: This Char is not a Digit")]
    CharNotDIGIT,
    #[error("Err: This Char cannot be used as a Hexdigit")]
    CharNotBASE16,
    #[error("Err: This Char is not a Letter")]
    CharNotLETTER,
    #[error("Err: This Char is not Printable")]
    CharNotPRINTABLE,
}
fn next_prime(x: u16) -> Option<u16> {
    if x == u16::MAX {
        None
    } else {
        let mut next_number = x + 1;
        loop {
            let mut prime = true;
            let mut i: u16 = 2;
            while i < next_number / 2 {
                if next_number.is_multiple_of(i) {
                    prime = false;
                    break;
                }
                i += 1;
            }
            if prime {
                return Some(next_number);
            } else {
                if next_number == (u16::MAX) {
                    return None;
                }
            }
            next_number += 1;
        }
    }
}

fn checked_addition_ex2(x: u32, y: u32) -> u32 {
    if x > u32::MAX - y {
        panic!("Adunarea nu poate fi efectuata(overflow)")
    }
    x + y
}
fn checked_multiplication_ex2(x: u32, y: u32) -> u32 {
    if y != 0 && x > u32::MAX / y {
        panic!("Inmultirea nu poate fi efectuata(overflow)");
    }
    x * y
}

fn checked_addition_ex3(x: u32, y: u32) -> Result<u32, CheckError> {
    if x > u32::MAX - y {
        Err(CheckError::ErrorAddition)
    } else {
        Ok(x + y)
    }
}
fn checked_multiplication_ex3(x: u32, y: u32) -> Result<u32, CheckError> {
    if y != 0 && x > u32::MAX / y {
        Err(CheckError::ErrorMultiplication)
    } else {
        Ok(x * y)
    }
}
fn handlechekederr(myerror: CheckError) {
    match myerror {
        CheckError::ErrorAddition => println!("Adunarea nu poate fi efectuata(overflow)"),
        CheckError::ErrorMultiplication => println!("Inmultirea nu poate fi efectuata(overflow)"),
    }
}

fn to_uppercase(c: char) -> Result<char, MyError> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(MyError::CharNotLETTER)
    }
}

fn to_lowercase(c: char) -> Result<char, MyError> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(MyError::CharNotLETTER)
    }
}

fn print_char(c: char) -> Result<char, MyError> {
    let x = c as u32;
    match x {
        0..=127 => {
            println!("{c}");
            Ok(c)
        }
        _ => Err(MyError::CharNotPRINTABLE),
    }
}

fn char_to_number(c: char) -> Result<u32, MyErrorBONUS> {
    if !c.is_ascii() {
        Err(MyErrorBONUS::CharNotASCII)
    } else {
        if !c.is_ascii_digit() {
            Err(MyErrorBONUS::CharNotDIGIT)
        } else {
            Ok(c as u32)
        }
    }
}

fn char_to_number_hex(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        Err(MyError::CharNotASCII)
    } else {
        match c {
            'a'..='f' => Ok(c as u32 - 'a' as u32 + 10_u32),
            'A'..='F' => Ok(c as u32 - 'A' as u32 + 10_u32),
            '0'..='9' => Ok(c as u32 - '0' as u32),
            _ => Err(MyError::CharNotBASE16),
        }
    }
}

fn print_error(myerror: MyError) {
    match myerror {
        MyError::CharNotASCII => println!("Err: This Char is not ASCII"),
        MyError::CharNotDIGIT => println!("Err: This Char is not a Digit"),
        MyError::CharNotBASE16 => println!("Err: This Char cannot be used as a Hexdigit"),
        MyError::CharNotLETTER => println!("Err: This Char is not a Letter"),
        MyError::CharNotPRINTABLE => println!("Err: This Char is not Printable"),
    }
}

fn fibonacci(n: u32) -> Option<u32> {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    while a < n as u32 {
        let next = a + b;
        a = b;
        b = next;
    }
    if a == n { Some(n) } else { None }
}

fn main() {
    for i in 0..=u16::MAX {
        if let Some(t) = next_prime(i) {
            println!("Prime: {}", t);
        }
    }

    match checked_addition_ex3(u32::MAX, 1) {
        Ok(r) => println!("u32::MAX + 1 = {r}"),
        Err(e) => handlechekederr(e),
    }
    match checked_multiplication_ex3(50_000, 70_000) {
        Ok(r) => println!("Inmultire rezultat= {r}"),
        Err(e) => handlechekederr(e),
    }
    match checked_multiplication_ex3(330_000, 33_000) {
        Ok(r) => println!("Inmultire rezultat = {r}"),
        Err(e) => handlechekederr(e),
    }

    match to_uppercase('7') {
        Ok(r) => println!("to_uppercase = {r}"),
        Err(e) => print_error(e),
    }

    match to_lowercase('C') {
        Ok(r) => println!("to_lowercase = {r}"),
        Err(e) => print_error(e),
    }
    match print_char('C') {
        Ok(_) => {}
        Err(e) => print_error(e),
    }
    match char_to_number('A') {
        Ok(n) => println!("char_to_number = {n}"),
        Err(e) => println!("{e}"),
    }
    match char_to_number_hex('F') {
        Ok(n) => println!("char_to_number_hex = {n}"),
        Err(e) => print_error(e),
    }
    match fibonacci(8) {
        Some(n) => println!("Number {n} is Fibonacci"),
        None => println!("Number is not Fibonacci"),
    }
}
