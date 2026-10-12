fn add_chars_n(mut x: String, character: char, mut count: u32) -> String {
    while count > 0 {
        x += &String::from(character);
        count -= 1;
    }
    x
}
fn add_chars_n_borrowed(x: &mut String, character: char, mut count: u32) {
    while count > 0 {
        *x += &String::from(character);
        count -= 1;
    }
}
fn add_space(x: &mut String, mut count: u32) {
    while count > 0 {
        x.push(' ');
        count -= 1;
    }
}

fn add_str(x: &mut String, y: String) {
    *x += &y;
}

fn add_integer(x: &mut String, mut integerr: i32) {
    let sign: i32 = if integerr >= 0 { 1 } else { -1 };
    let mut integerr_reversed_string: String = String::from("");
    let mut add_underscore: u8 = 0;
    integerr = if sign == 1 { integerr } else { -integerr };
    if integerr == 0 {
        *x += "0";
        return;
    }
    while integerr != 0 {
        if add_underscore == 3 {
            integerr_reversed_string += "_";
            add_underscore = 0;
        }
        let c = (integerr % 10) as u8;
        integerr_reversed_string += &String::from((c + b'0') as char);
        integerr /= 10i32;
        add_underscore += 1;
    }
    *x += if sign == -1i32 { "-" } else { "" };
    for i in integerr_reversed_string.chars().rev() {
        x.push(i);
    }
}

fn add_float(x: &mut String, mut floatt: f32) {
    let mut copyfloat: f32 = 1.0;

    while copyfloat <= floatt {
        copyfloat += 1.0;
    }
    copyfloat -= 1.0;
    add_integer(x, copyfloat as i32);
    floatt -= copyfloat;
    *x += ".";
    let mut precision = 0;
    while floatt < 10.0 && precision < 6 {
        floatt *= 10.0;
        copyfloat = 1.0;
        while copyfloat <= floatt {
            copyfloat += 1.0;
        }
        copyfloat -= 1.0;
        add_integer(x, copyfloat as i32);
        floatt -= copyfloat;
        if floatt == 0.0 {
            return;
        };
        precision += 1;
    }
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);
        i += 1;
    }
    println!("{}", s);

    /*Problema 2 */
    s = String::from("");
    add_chars_n_borrowed(&mut s, 'B', 10);
    println!("{}\n", s);

    /*Problema 3*/
    s = String::from("");
    add_float(&mut s, 2.038);
    println!("{}", s);
    s = String::from("");
    add_space(&mut s, 10);
    add_str(&mut s, "I 💚".to_string());
    add_space(&mut s, 160);
    add_str(&mut s, "RUST.".to_string());
    add_str(&mut s, "\n".to_string());
    add_str(&mut s, "Most".to_string());
    add_space(&mut s, 11);
    add_str(&mut s, "crate".to_string());
    add_integer(&mut s, 306437968);
    add_space(&mut s, 6);
    add_str(&mut s, "and".to_string());
    add_space(&mut s, 10);
    add_str(&mut s, "latest".to_string());
    add_space(&mut s, 10);
    add_str(&mut s, "is".to_string());
    add_space(&mut s, 80);
    add_str(&mut s, " downloaded".to_string());
    add_space(&mut s, 10);
    add_str(&mut s, " has".to_string());
    add_space(&mut s, 10);
    add_str(&mut s, " downloads".to_string());

    println!("{}", s);
}
