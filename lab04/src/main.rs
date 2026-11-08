use std::{fs, io};
const ABBREV: [(&str, &str); 4] = [
    ("pentru", "pt"),
    ("pentru", " ptr"),
    ("domnul", "dl"),
    ("doamna", "dna"),
];

fn p1(x: &str) {
    let mut maxbytes: usize = usize::MIN;
    let mut maxchars: usize = usize::MIN;
    for line in x.lines() {
        if maxbytes < line.len() {
            maxbytes = line.len();
        }
        if maxchars < line.chars().count() {
            maxchars = line.chars().count();
        }
    }
    for line in x.lines() {
        if maxbytes == line.len() {
            println!("Biggest line as bytes: {line}");
        }
        if maxchars == line.chars().count() {
            println!("Biggest line as chars: {line}");
        }
    }
}

fn full_word(cuvant: &str) -> &str {
    for (intreg, abv) in ABBREV {
        if cuvant == abv {
            return intreg;
        }
    }
    cuvant
}

fn cypher(x: char) -> char {
    match x {
        'a'..='z' => (((x as u8) - b'a' + 13u8) % 26 + b'a') as char,
        'A'..='Z' => (((x as u8) - b'A' + 13u8) % 26u8 + b'A') as char,
        _ => x,
    }
}

fn main() {
    let s1 = match fs::read_to_string("examples/p1.in") {
        Ok(s1) => s1,
        Err(e) => {
            println!("Err:{}", e);
            return;
        }
    };
    p1(&s1);

    let s2 = match fs::read_to_string("examples/p2.in") {
        Ok(s2) => s2,
        Err(e) => {
            println!("Err:{}", e);
            return;
        }
    };

    let mut s2_encrypted = String::new();

    for c in s2.chars() {
        if !c.is_ascii() {
            println!("There are non ascii characters");
            break;
        }
        s2_encrypted.push(cypher(c));
    }
    println!("Encrypted message: {}", s2_encrypted);

    let s3 = match fs::read_to_string("examples/p3.in") {
        Ok(s3) => s3,
        Err(e) => {
            println!("Err:{}", e);
            return;
        }
    };
    let mut s3_full = String::new();
    for cuv in s3.split(' ') {
        s3_full.push_str(full_word(cuv));
        s3_full.push(' ');
    }
    match fs::write("examples/p3.out", &s3_full) {
        Ok(_) => println!("P3 verifica fisierul p3.out!"),
        Err(e) => println!("P3 eroare la scriere in p3.out, {e}"),
    }

    let s4 = match fs::read_to_string("examples/p4_hosts.in") {
        Ok(s4) => s4,
        Err(e) => {
            println!("Err:{}", e);
            return;
        }
    };
    for line in s4.lines(){
        let mut ip_addr=String::from(" => ");
        let mut step:bool=true;
        for word in line.split(' '){
            if word=="#"{
             break;
            }
            match step{
                true=>  ip_addr.push_str(word),
                false=> ip_addr.insert_str(0,word ),
            }
            step= !step;
        }
        if ip_addr!=" => "{
             println!("{ip_addr}");
        }
    }

}
