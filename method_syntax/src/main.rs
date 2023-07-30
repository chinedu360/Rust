#[derive(Debug)]
// Methods are similar to functions: we declare them with the fn keyword and a name, they can have params and a return value, and they conatins some code that runs when the method is called from somewhere else.

//unlike functions, methods are defined within the context of a struct(or an enum or a trait object) and there first parameter is always "self", this represents the instance of the struct the method isbeing called on.

//Defining methods
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     //we can give a method a name similar to one of the struct fields
//     //often but not always when we give a method the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called "getters"
//     //Getter are useful bcos we can make the field private  but the method public, and thus enable read-only access to that field as part of the  types's public API.
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!("The area of the rectangle is {} square pixels.", rect1.area());

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width );
//     }
// }

//METHODS WITH MORE PARAMETERS
struct Rectangle {
    width: u32,
    height: u32,
}

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     //method with more parameters
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };

//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3 {}", rect2.can_hold(&rect3));
// }

//Associated Functions
// Note: All functions defined within the "impl" block are called associated functions, bcos they are defined within the type named after the "impl".

//we can define associated functions as fuctions that do not have "self" as there first parameter and are not methods, bcos they dont need an instance of type to work with.
//An example of this which we have used is the "String::from" function that is defined on the string type.

//Associated functions that arent methods are often used for constructors that will return a new instance of the struct. These are often called "new", but " new" isn't a special name and the isn't built into the language.

//EXAMPLE
// we can choose to provide an associated function name "square" that would have one dimension params and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same values twice.

// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self { 
//             width: size,
//             height: size,
//         }
//     }
// }

// The "Self" keyword in the return type and in the body of the function are aliases for the type that appears from the "impl" keyword in this case the "Rectangle"

// To call the assiociated function, we use the "::" syntax with the struct name;
// like below
// let sq = Rectangle::square(3);

// the fucntion is namespaced by the struct: the :: syntax is used for both associted fuctions and namespaces created by modules.


//Multiple impl Blocks
// Each structs is allowed to have multiple blocks.
// for example the below 
// impl Rectangle {
//     fn area(&self) -> u32 { self.width * self.height }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

//is equal to the following to the below

// impl Rectangle {
//     fn area(&self) -> u32 { self.width * self.height }
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

//there is no need to separate this methods into multiple "impl" blocks, but its worthy to note that this is a valid syntax.


//METHOD CALLS ARE SYNTATIC SUGAR FOR FUNCTION CALLS