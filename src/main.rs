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

    println!("Are you satisfied with your option?");

    let satisfaction: String = get_input();

    if satisfaction == "yes"{
        println!("Have a nice day :)");
    }
        else if satisfaction == "no"{
            println!("Do you want spicy or bland food?");
        }
        else {
            println!("fuck you");
        }
    } else if hungry == "no" {
        println!("Ok :)");
    } else {
        println!("fuck you");
    }

    println!("Are you thirsty?");

    let thirst = get_input();
        if thirst == "yes"{
            println!("Scale of 1-10, how thirsty?");
        }
        else if thirst == "no"{
            println! ("Ok :)");
        }
        else {
            println!("fuck you")
        }
}

fn get_input() -> String {
    let mut input = String::new();
    let my_stdin = stdin();
    my_stdin.read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();
    input.to_string()
}
