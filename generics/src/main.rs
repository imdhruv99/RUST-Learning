// generics struct
// X is generic type of T, Y is U, they could be the same or they could be the different
struct Point<T, U> {
    x: T, 
    y: U,
}

// other will have different types
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// generics are mentioned with <>
// PartialOrd and Copy is traits. Means our generic type has to be orders and we could copy.
// such as int, ch etc.
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest  = list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn main() {
    // passing char list in generic function
    let char_list = vec!['x', 'k', 'm', 'd', 'i'];
    let char_largest = get_largest(char_list);
    println!("Largest character is: {}", char_largest);

    // passing int list in generic function
    let int_list = vec![500, 58674, 323289, 293, 2994];
    let int_largest = get_largest(int_list);
    println!("Largest integer is: {}", int_largest);

    // generic struct example
    let p3 = Point { x: 5, y: 6};
    let p4 = Point { x: 500.0, y: 600.0};
    let p5 = Point::mixup(p3, p4);
    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);
}