// fn main() {
    
//     let x = true;
//     ready(x); 
//     // ready(x); // x not found in this scope
//     // let x = true;

//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(m1, m2);
//     let s = format!("{} {}", m1, m2);
// }

// fn ready(y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }
//both string params are dropped after the end of the greet fn do for that reason  m1 & m2 are not accesible in the format!(..) which is undefined and the browser won't accept.
// fn greet(g1: String, g2: String) {
//     println!("{} {}", g1, g2);
// }

//we could return ownership of the string like this: 

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("World");

//     let (m1_again, m2_again) = greet(m1, m2);
    // let s = format!("{} {}", m1_again, m2_again);
// }

// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}", g1, g2);
//     (g1, g2)
// }

//This pattern is verbose. Rust provides a concise style of reading and writing without using moves through references.

// fn main() {
    // let m1 = String::from("Hello");
    // let m2 = String::from("World");

    // great(&m1, &m2); //take note of the ampersands
    // let s = format!("{} {}", m1, m2);

// }

// fn great(g1: &String, g2: &String) { // note the ampersands
//     println!("{} {}", g1, g2); 
// }

//The issue is withe the lifetime of the referred data, if we want to pass around a reference to a string we have to make sure that the underlying string loive long enough

//How do we fix this probs? in the below fn
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

//depending on the situation there are four ways we can extend the lifetime of the string.

//1. moving ownership of the stringout of the function, basiucally changing &String to String
// fn return_a_string() -> String {
//     let s = String::from("Hello world");
//     s
// }

//2. Another way is returning a String literal that lives forever(A "'static" string), This solution applies if we never intend to change the string and in this case the heap allocationis unnecessary
// fn return_a_string() -> &'String str {
//     "Hello World"
// }

//3. Another possibility is to defer borrow-checking to runtime by using garbage collection. for example you can use a reference-counted pointer
//in short Rc::clone only clones a pointer to s and not the data itself. At runtime the Rc checks when trhe last Rc pointing to the data has been dropped and then dellocates the data.
// use std::rc::Rc;
// fn return_a_string() -> Rc<String> {
//     let s = Rc::new(String::from("Hello World"));
//     Rc::clone(&s)
// }

//4. Yet another possibility is to have the caller provide a "slot" to put the string using a mutable reference:
// with this strategy, the caller is responsible for creating space for the string. This method can be verbose, but it can be more memory efficient if the caller needs needs to carefully control when allocations occurs.
// fn return_a_string(output: &mut String) {
//     output.replace_range(.., "Hello World");
// }

//Fixing an Unsafe Program: Not Enough Permissions
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}
//fixing the above error 

//1. an easy way is to change the typr of name from &Vec<String> to &mut Vec<String> just like below

fn stringify_name_with_title(name: &mut Vec<String>) {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
//as a general rule of thumn function should not mutate there inputs, this is because a person call the function may not expect there vector to change or be modified by this fn.

//2. Another option is to take ownership of the name, by chaanging &Vec<String> to Vec<String>
fn stringify_name_with_title(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

//this is not a good solution too! its is very rare for rust fn to take ownership of heap-owning data structure like Vec and String. This version of the fn would make the input name variable unusable, which is not good for the caller

//so the choice &vec is actually a good one, which we do not need to change. instead we can change the body of the function. There are many fixes which vary in how much memory they use. one possiblity is to be done using clone on the input name:

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

//by cloning name, we are allowed to mutate the local copy of the vector. However, the clone copies every string in the input. we can avoid unnecessary copies by adding the suffix later

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str("Esq.");
    full
}