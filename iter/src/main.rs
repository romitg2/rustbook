fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let mut v_iter = v.iter();

    let next = v_iter.next();

    match next {
        Some(value) => println!("The first value is: {}", value),
        None => println!("No value found"),
    }

    for value in v_iter {
        println!("The value is: {}", value);
    }

    let squared = v.iter().map(|x| x * x).collect::<Vec<i32>>();

    print!("Squared values: {:?}\n", squared);   
     
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}