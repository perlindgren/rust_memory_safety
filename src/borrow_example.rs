// Rust borrow
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn copy_a_value() {
    let point = Point { x: 1, y: 2 };
    println!("point {:?}", point);

    // let us now borrow this value to another_variable
    let another_variable = &point;
    println!("another_variable {:?}", another_variable);

    // let us try to access point
    println!("point {:?}", point);
}

#[test]
fn borrow_a_value_as_argument() {
    fn some_function(point: &Point) {
        println!("in some_function {:?}", point);
    }

    let point = Point { x: 1, y: 2 };
    println!("point {:?}", point);

    // let some_function borrow the point
    some_function(&point);

    // let us try to access point
    println!("point {:?}", point);
}

#[test]
fn mutably_borrow_a_value_as_argument() {
    fn some_function(point: &mut Point) {
        println!("in some_function {:?}", point);
        point.x = 42;
    }

    let mut point = Point { x: 1, y: 2 };
    println!("point {:?}", point);

    // let some_function borrow the point
    some_function(&mut point);

    // let us try to access point
    println!("point {:?}", point);
}

#[test]
fn mutably_and_immutably_borrow_a_value_as_argument() {
    fn some_function(point: &mut Point) {
        println!("in some_function {:?}", point);
        point.x = 42;
    }

    let mut point = Point { x: 1, y: 2 };
    let another_variable = &point;
    println!("point {:?}", point);

    // let some_function mutably borrow the point
    some_function(&mut point);

    // let us try to access point
    println!("point {:?}", point);
    // let us now to access another_variable
    // println!("another variable {:?}", another_variable);
}
