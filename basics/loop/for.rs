fn main() {
    for i in 1..11 {
        println!("{}", i);
    }

    let animals = vec!["Rabbit", "tiger", "cow"];

    for (idx, animal) in animals.iter().enumerate() {
        println!("{} {}", animal, idx);
    }
}
