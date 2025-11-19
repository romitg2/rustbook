fn main() {
    let s = String::from("Hello, world!");
    let s_move = s;

    println!("{}", s_move);

    let s_copy = s_move.clone();
    println!("{}", s_copy);
}
