fn main() {

    // tuples:
    tuples_example();

    // arrays:
    arrays_example();

    // functions:
    let result = sum(5, 10);
    println!("The sum is: {}", result);

    // loops
    loops_example();
}

fn loops_example() {
    let mut count = 0;

    // while loop
    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }

    // for loop : 1
    let numbers = [10, 20, 30, 40, 50];
    for number in numbers.iter() {
        println!("The number is: {}", number);
    }
    // for loop : 2
    let res = loop {
        count += 1;
        if count >= 10 {
            break count * 2; // returning value from loop
        }
    };
    println!("The result from loop is: {}", res);

    // for loop : 3
    for i in (1..6).rev() {
        println!("Countdown: {}", i);
    }
    
}

fn tuples_example() {
    let tup = ("romit", 22);
    let (name, age) = tup;

    println!("Hello, {}. You are {} years old.", name, age);
    println!("Hello, {}. You are {} years old.", tup.0, tup.1);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y   
}

fn arrays_example() {

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    println!("The error code is: {}", not_found);

    let same_values = [3; 5]; // creates an array of 5 elements, all set to 3
    println!("Array with same values: {:?}", same_values);

}
