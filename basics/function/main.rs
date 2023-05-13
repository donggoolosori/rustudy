fn main() {
    print_numbers_to(10);

    if is_even(2) {
        println!("2 is even");
    }
}

fn print_numbers_to(num: u32) {
    println!("{}", num);
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
