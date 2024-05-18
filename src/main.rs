fn pattern_1(n: i32) {
    for _ in 0..n {
        for _ in 0..n {
            print!("*");
        }
        println!();
    }
}

fn pattern_2(n: i32) {
    for i in 0..n {
        for _ in 0..i + 1 {
            print!("*");
        }
        println!();
    }
}

fn pattern_3(n: i32) {
    for i in 1..n + 1 {
        for j in 1..i + 1 {
            print!("{}", j);
        }
        println!();
    }
}

fn pattern_4(n: i32) {
    for i in 1..n + 1 {
        for _ in 1..i + 1 {
            print!("{}", i);
        }
        println!();
    }
}

fn pattern_5(n: i32) {
    for i in 1..n + 1 {
        for _ in 0..n - i + 1 {
            print!("*");
        }
        println!();
    }
}

fn pattern_6(n: i32) {
    for i in 1..n + 1 {
        for j in 0..n - i + 1 {
            print!("{}", j + 1);
        }
        println!();
    }
}

fn pattern_7(n: i32) {
    for i in 1..n + 1 {
        for _ in 0..n - i {
            print!(" ");
        }
        for _ in 0..(2 * i - 1) {
            print!("*");
        }
        println!();
    }
}

fn pattern_8(n: i32) {
    for i in 1..n + 1 {
        for _ in 1..i {
            print!(" ");
        }
        for _ in 0..2 * (n - i) + 1 {
            print!("*");
        }

        println!();
    }
}

fn main() {
    pattern_1(5);
    println!();

    pattern_2(5);
    println!();

    pattern_3(5);
    println!();

    pattern_4(5);
    println!();

    pattern_5(5);
    println!();

    pattern_6(5);
    println!();

    pattern_7(5);
    println!();

    pattern_8(5);
    println!();
}
