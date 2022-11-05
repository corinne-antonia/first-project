use std::io::stdin;

fn main() {
    println!("Are you hungry!");

    let hungry = get_input();

    if hungry == "yes" {
        println!("Do you want salty or sweet food?");

        let sweet_salty = get_input();

        if sweet_salty == "salty" {
            println!("Make chinese food");
        } else if sweet_salty == "sweet" {
            println!("have breakfast for dinner");
        } else {
            println!("fuck you");
        }
    } else if hungry == "no" {
        println!("Drink water!!!!!");
    } else {
        println!("fuck you");
    }
}

fn get_input() -> String {
    let mut input = String::new();
    let my_stdin = stdin();
    my_stdin.read_line(&mut input).unwrap();
    let input = input.trim();
    input.to_string()
}
