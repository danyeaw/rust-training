fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let max = input.iter().max().unwrap();
    let min = input.iter().min().unwrap();

    println!("{} is largest and {} is smallest", max, min);
}
