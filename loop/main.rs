fn main() {
    let mut n = 0;

    loop {
        n += 1;

        if n > 10 {
            break;
        }

        if n == 1 {
            continue;
        }
        println!("value = {}", n);
    }
}
