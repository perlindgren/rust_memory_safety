// Rust move

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn print_a_value() {
    let point = Point { x: 1, y: 2 };
    println!("point {:?}", point);
}

#[test]
fn move_a_value() {
    let point = Point { x: 1, y: 2 };
    println!("point {:?}", point);

    // let us now move this value to another_variable
    let another_variable = point;
    println!("another_variable {:?}", another_variable);

    // let us try to access point
    // println!("point {:?}", point);
}

#[test]
fn move_a_value_as_argument() {
    fn some_function(point: Point) {
        println!("in some_function {:?}", point);
    }

    let point = Point { x: 1, y: 2 };
    println!("point {:?}", point);

    some_function(point);

    // let us try to access point
    // println!("point {:?}", point);
}
