fn main() {
    let actions = vec!["Learn Rust", "Learn Go", "Convince Thomas"];
    let number_of_actions = actions.len();

    match number_of_actions {
        0 => println!("Nothing left to do, great job!"),
        1 => println!("There is one action:"),
        _ => println!("There are {} actions:", number_of_actions),
    }

    for action in actions.iter() {
        println!("{}", action);
    }
}
