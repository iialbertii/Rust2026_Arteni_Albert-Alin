fn is_prime(x: u32) -> bool {
    let mut i: u32 = 2;

    if x == 0 {
        return false;
    }
    while i <= { x / 2 } {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }

    return true;
}

fn is_coprime(x: u32, y: u32) -> bool {
    let mut copy_x: u32 = x;
    let mut copy_y = y;
    let mut r: u32;
    while copy_x != 0 {
        r = copy_y % copy_x;
        copy_y = copy_x;
        copy_x = r;
    }

    return copy_y == 1;
}

fn main() {
    let mut x: u32 = 0;
    let mut y: u32;
    println!("Problem 1: \n \n");
    while x <= 100 {
        if is_prime(x) == true {
            println!("Number {} is prime\n", x);
        } else {
            println!("Number {} is not prime\n", x);
        }
        x += 1;
    }

    print!("\n\n Problem 2 \n\n");
    x = 0;

    while x <= 100 {
        y = x + 1;
        while y <= 100 {
            if y > 100 {
                break;
            }
            if is_coprime(x, y) == true {
                println!("Numbers x: {} y: {} is coprime\n", x, y);
            } else {
                println!("Number x:{} y: {} is not coprime\n", x, y);
            }

            y += 1;
            if y >= 100 {
                break;
            }
        }
        x += 1;
    }

    print!("\n\n Problem 3 \n\n");

    x = 99;

    while x != 0 {
        print!(
            "{x} bottles of beer on the wall,\n{x} bottles of beer.\nTake one down, pass it around,\n"
        );
        x -= 1;
        if x == 0 {
            println!("No bottles of beer on the wall.\n");
            break;
        } else {
            println!("{} bottles of beer on the wall.\n", x);
        }
    }
}
