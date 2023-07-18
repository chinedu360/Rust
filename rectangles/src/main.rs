// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }


// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//refactoring with tuples
// fn main() {
//     let rect1 = (30, 50);

//     println!("The area of the rectangle is {} square pixels", area(rect1))
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Tuples lets us add a bit of structure and we are now passing just one argument.
//But there is a tradeoff...
//This refactored version is less clear, this is because tuples do not name thier element, so we nhave to index into parts of the tuples this makes our calculation less obvious.

//mixing up the width and height wouldn't matter for the area calculation, but if we want to draw the rectangle on a screen it would matter! we would have to keep in mind that the width in the tuple has an index of 0 and the height in the tuple has an index of 1. This would seem very difficult for someone else to figure out and keep in mind if they were using our code.

//Refactoring with Structs: Adding More Meaning
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("The area of the rectangle is {} square pixels.", area(&rect1));
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

//This is an added clarity. The above

// the below code won't run
// struct  Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let react1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {}", react1)
// }
// The above code won't run, but when we compile we get the below error.
// error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`

//Note: putting :? inside the print(pintln!("rect1 is {:?}", rect1))macro in a curly brace tells println! that we want to use an output format called Debug.

// The debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we are debugging our code.
 
//if we compile our code after adding this
//pintln!("rect1 is {:?}", rect1)

//we still get an error
//error[E0277]: `Rectangle` doesn't implement `Debug`
//Then the compiler gives us a helpful note:
//    = help: the trait `Debug` is not implemented for `Rectangle`
//    = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`

//Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our structs. In other to achieve this we add the outer attribute #[derive(Debug)] just before the struct lsiting. like below.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let react1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("reactr1 is {:?}", react1);
}

// when we run our code we get the below
// Finished dev [unoptimized + debuginfo] target(s) in 0.33s
//  Running `target/debug/rectangles`
// reactr1 is Rectangle { width: 30, height: 50 }
// nedu@Chinedus-MacBook-Air rectangles % 

//In situations where we have a big struct we can use {:#?} inside our print macro this gives us an easier to read output.