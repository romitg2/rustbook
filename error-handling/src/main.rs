use std::fs::File;

fn main() {
    // do_panic();

    let file = File::open("non_existent_file.txt");
    match file {
        Ok(_) => println!("File opened successfully."),
        Err(e) => eprintln!("Failed to open file: {}", e),
    }
}

fn do_panic() {
    panic!("This is another test panic message.");
}
