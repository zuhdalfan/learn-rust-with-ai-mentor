#[derive(Debug)] // this is for struct printing
struct User {
    name: String,
    age: u8,
}

enum Action {
    Add(i64),
    Sub(i64),
    Mul(i64),
}

fn use_the_struct() {
    let user = User { name: "Zuhdi".to_string(), age: 30};

    println!("User: {:?}", user);
}

fn apply_action(base: i64, action: Action) -> i64 {
    match action {
        Action::Add(v) => base + v,
        Action::Sub(v) => base - v,
        Action::Mul(v) => base * v,
    }
}

fn use_the_enum() {
    let base = 10;
    let mut action = Action::Add(5);
    let mut result = apply_action(base, action);
    println!("Result: {}", result);

    action = Action::Sub(5);
    result = apply_action(base,action);
    println!("Result: {}", result);


    action = Action::Mul(5);
    result = apply_action(base,action);
    println!("Result: {}", result);
}

fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn loud_name(name: String) -> String {
    name.to_uppercase()
}

fn borrow_and_ownership() {
    let user = User {
        name: "Ada".to_string(),
        age: 20,
    };

    let greeting = greet(&user.name); // borrowing, still usable
    println!("{}", greeting);

    let loud = loud_name(user.name); // moving, not usable anymore
    println!("{}", loud);

    // println!("{}", user.name); // error, user is not usable anymore

    println!("Age still ok: {}", user.age); // age is copy
}

fn main() {
    use_the_struct();
    use_the_enum();
    borrow_and_ownership();

}