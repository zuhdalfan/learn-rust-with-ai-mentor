#[derive(Debug)] // allow struct to be printed with {:?}
struct User { // define a simple user type
    name: String, // owned name string
    age: u8, // age stored as an 8-bit unsigned integer
} // end User struct

enum Action { // define possible actions for a calculator-like example
    Add(i64), // add this value
    Sub(i64), // subtract this value
    Mul(i64), // multiply by this value
} // end Action enum

fn use_the_struct() { // demo function for struct usage
    let user = User { name: "Zuhdi".to_string(), age: 30 }; // create a User with owned name

    println!("User: {:?}", user); // print the user using Debug format
} // end use_the_struct

fn apply_action(base: i64, action: Action) -> i64 { // apply an Action to a base number
    match action { // pattern match on the enum variant
        Action::Add(v) => base + v, // add the value
        Action::Sub(v) => base - v, // subtract the value
        Action::Mul(v) => base * v, // multiply by the value
    } // end match
} // end apply_action

fn use_the_enum() { // demo function for enum usage
    let base = 10; // starting value
    let mut action = Action::Add(5); // first action is add 5
    let mut result = apply_action(base, action); // apply action to base
    println!("Result: {}", result); // print result

    action = Action::Sub(5); // change action to subtract 5
    result = apply_action(base, action); // apply action to base
    println!("Result: {}", result); // print result

    action = Action::Mul(5); // change action to multiply by 5
    result = apply_action(base, action); // apply action to base
    println!("Result: {}", result); // print result
} // end use_the_enum

fn greet(name: &str) -> String { // borrow a name and return a greeting
    format!("Hello, {name}!") // build a new String using formatting
} // end greet

fn loud_name(name: String) -> String { // take ownership of name and return uppercase
    name.to_uppercase() // transform the string to uppercase
} // end loud_name

fn borrow_and_ownership() { // demo borrowing vs ownership moves
    let user = User { // create a new User
        name: "Ada".to_string(), // owned name string
        age: 20, // age as u8
    }; // end User literal

    let greeting = greet(&user.name); // borrow the name; user still usable
    println!("{}", greeting); // print the greeting

    let loud = loud_name(user.name); // move the name; user.name no longer usable
    println!("{}", loud); // print the uppercase name

    // println!("{}", user.name); // this would error because name was moved

    println!("Age still ok: {}", user.age); // age is Copy, so still usable
} // end borrow_and_ownership

fn main() { // program entry point
    use_the_struct(); // run the struct demo
    use_the_enum(); // run the enum demo
    borrow_and_ownership(); // run the ownership demo
} // end main
