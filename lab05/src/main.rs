
use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn update(oldest: &mut Option<Student>, youngest: &mut Option<Student>, s: Student) {
    let is_older = match oldest {
        Some(o) => s.age > o.age,
        None => true,
    };
    if is_older {
        *oldest = Some(s.clone());
    }

    let is_younger = match youngest {
        Some(y) => s.age < y.age,
        None => true,
    };
    if is_younger {
        *youngest = Some(s);
    }
}

fn print_result(oldest: Option<Student>, youngest: Option<Student>) {
    match (oldest, youngest) {
        (Some(o), Some(y)) => {
            println!("Oldest:   {} ({}), phone {}", o.name, o.age, o.phone);
            println!("Youngest: {} ({}), phone {}", y.name, y.age, y.phone);
        }
        _ => println!("No students found."),
    }
}

fn parse_csv_line(line: &str) -> Option<Student> {
    let mut parts = line.split(',');
    let name = parts.next()?.to_string();
    let phone = parts.next()?.to_string();
    let age = match parts.next()?.parse::<u32>() {
        Ok(a) => a,
        Err(_) => return None,
    };
    if parts.next().is_some() {
        return None;
    }
    Some(Student { name, phone, age })
}

fn p1() {
    let content = match fs::read_to_string("./example/students.in") {
        Ok(c) => c,
        Err(e) => {
            println!("Cannot read students.in: {e}");
            return;
        }
    };

    let mut oldest: Option<Student> = None;
    let mut youngest: Option<Student> = None;
    let mut line_number = 0;

    for line in content.lines() {
        line_number += 1;
        if line.trim().is_empty() {
            continue;
        }
        match parse_csv_line(line) {
            Some(s) => update(&mut oldest, &mut youngest, s),
            None => println!("Skipping invalid line {line_number}: {line}"),
        }
    }

    print_result(oldest, youngest);
}


struct Canvas {
    pixels: [[char; 100]; 55],
}

fn new_canvas() -> Canvas {
    Canvas {
        pixels: [[' '; 100]; 55],
    }
}

fn set_pixels(canvas: &mut Canvas, pixels: &[(usize, usize, u8)]) {
    for &(row, col, code) in pixels {
        if row < 55 && col < 100 {
            canvas.pixels[row][col] = code as char;
        }
    }
}

fn print(canvas: Canvas) {
    for row in 0..55 {
        for col in 0..100 {
            print!("{}", canvas.pixels[row][col]);
        }
        println!();
    }
}

fn p2() {
    let mut canvas = new_canvas();
    let c = &mut canvas;
    
    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(c, &[(2, 16, 95), (5, 35, 92), (6, 3, 95), (2, 11, 95), (5, 3, 95)]);
    set_pixels(c, &[(2, 38, 95), (4, 9, 40), (3, 41, 124), (2, 37, 95), (2, 25, 124)]);
    set_pixels(c, &[(5, 27, 124), (2, 27, 124), (4, 0, 124), (3, 35, 47), (2, 18, 95)]);
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(c, &[(6, 30, 95), (5, 45, 95), (4, 31, 124), (4, 7, 124), (3, 43, 39)]);
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(c, &[(3, 44, 95), (3, 19, 92), (5, 23, 95), (3, 8, 47), (2, 10, 95)]);
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(c, &[(3, 10, 95), (4, 44, 95), (2, 9, 95), (0, 32, 95), (5, 2, 95)]);
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(c, &[(3, 46, 92), (5, 25, 44), (1, 43, 124), (5, 46, 47), (3, 15, 47)]);
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(c, &[(5, 44, 95), (2, 2, 95), (5, 10, 95), (5, 9, 95), (4, 43, 124)]);
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(c, &[(4, 40, 124), (3, 3, 95), (4, 4, 124), (6, 31, 47), (3, 4, 96)]);
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(c, &[(3, 12, 92), (2, 32, 95), (5, 37, 95), (5, 26, 95), (5, 39, 47)]);
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(c, &[(5, 36, 95), (7, 30, 95), (6, 4, 47), (4, 24, 95), (1, 32, 95)]);
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);
    print(canvas);
}

fn p3() {
    let students = match fs::read_to_string("./example/students.json") {
        Ok(c ) =>match p3_deserialiez(c){
            Ok(s)=> s,
            Err(_)=> {
                return;
            }
        },
        Err(e) => {
            println!("Cannot read students.json: {e}");
            return;
        }
    };
   

    let mut oldest: Option<Student> = None;
    let mut youngest: Option<Student> = None;

   
    for s in students {
        update(&mut oldest, &mut youngest, s);
    }

    print_result(oldest, youngest);
}
fn p3_deserialiez(content:String)->Result<[Student;4],serde_json::Error>{
    let students:[Student;4] =match serde_json::from_str(&content) {
        Ok(b) =>b,
        Err(e)=>{
            println!{"Cannot deserealize students.json: {e}"};
           return Err(e);
        }
    };
    Ok(students)

}


fn main() {
    p1();

    p2();

    p3();
}