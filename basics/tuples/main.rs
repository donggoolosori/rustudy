fn main() {
    let tup1 = (20, "Rust", 3.4, false, (1, 4, 7));

    println!("{}", (tup1.4).2);
    println!("{}", tup1.3);

    let tup2 = (10, 20, 30);
    let (a, b, c) = tup2;
    println!("{} {} {}", a, b, c);
}
