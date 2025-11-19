struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn distance_from_origin(&self) -> f64
    where
        T: Into<f64> + Copy,
    {
        let x: f64 = self.x.into();
        let y: f64 = self.y.into();
        (x.powi(2) + y.powi(2)).sqrt()
    }

    fn distance_from_origin_int(&self) -> f64
    where
        T: Into<i32> + Copy,
    {
        let x: i32 = self.x.into();
        let y: i32 = self.y.into();
        ((x.pow(2) + y.pow(2)) as f64).sqrt()
    }
}



fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 5.0, y: 10.0 };

    println!(
        "Distance of integer_point from origin: {}",
        integer_point.distance_from_origin_int()
    );

    println!(
        "Distance of float_point from origin: {}",
        float_point.distance_from_origin()
    );
}
