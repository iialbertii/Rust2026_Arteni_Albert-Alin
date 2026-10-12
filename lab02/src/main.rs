fn add_chars_n(mut x: String, character: char, mut count: u32) -> String {
    while count > 0 {
        x += &String::from(character);
        count -= 1;
    }
    x
}
fn add_chars_n_void(x: &mut String, character: char, mut count: u32) {
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
    let sign:i32 = if integerr>=0 {1} else {-1};
    let mut integerr_rev:String= String::from("");
    let mut add_underscore:u8=0;
    integerr= if sign==1 {integerr} else {-integerr};
    while integerr!=0{
        if add_underscore==3{
            integerr_rev+="_";
            add_underscore=0;
        }
        let c=(integerr%10) as u8;
        integerr_rev+=&String::from(( c + b'0') as char) ;
        integerr/=10i32;
        add_underscore+=1;
    }
    *x+= if sign==-1i32 {"-"} else {""};
    for i in integerr_rev.chars().rev(){
        x.push(i);
    }
}

fn add_float(x: &mut String, floatt: f32) {

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
    add_chars_n_void(&mut s, 'B', 10);
    println!("{}\n", s);

    /*Problema 3*/
    add_str(&mut s, "I love Rust <3".to_string());
    add_integer(&mut s, 9100);

    println!("{}", s);



}
