fn largest<T: Ord + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct AnyPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let int_point = Point { x: 6, y: 12 };
    let float_point = Point { x: 12.1, y: 6.1 };
    let both_point = AnyPoint { x: 5, y: 13.7 };
    println!("{}", int_point.x);
    println!("{}", float_point.distance_from_origin());

    let int_arr = [5, 3, 7, 1, 12, 6];
    let char_arr = ['a', 'g', 'c', 'd'];
    println!("{}", largest(&int_arr));
    println!("{}", largest(&char_arr));
}
