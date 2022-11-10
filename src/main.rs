use std::io::stdin;

fn main() {
    println!("Are you hungry!");

    let hungry = get_input();

    if hungry == "yes" {
        println!("On a scale of 1-10, how hungry are you?");
        let hungry_level = get_number();
        if hungry_level >= 1 && hungry_level <= 3 {
            println!("You aren't really hungry"); 
        }
        else if hungry_level > 3 && hungry_level <= 5 {
            println!("Have cheese and crackers for a snack");
        } 
        else if hungry_level > 5 && hungry_level <= 10 {
            println!("Do you want salty or sweet food?");
                let sweet_salty = get_input();
                if sweet_salty == "salty" {
                    println!("Make chinese food");
                } else if sweet_salty == "sweet" {
                    println!("have breakfast for dinner");
                } else {
                    println!("fuck you");
                }
            }
        } 

        println!("Are you satisfied with your option?");
        let satisfaction: String = get_input();

        if satisfaction == "yes" {
            println!("Ok :)"); 
        } else if satisfaction == "no" {
            println!("Do you want spicy or bland food?");
            let option_2: String = get_input();
            if option_2 == "bland" {
                println!("Eat toast");
            } else if option_2 == "spicy" {
                println!("Have indian food");
            } else {
                println!("fuck you");
            }
        }
    else if hungry == "no" {
        println!("Ok :)");
    } else {
        println!("fuck you");
    }

    println!("Are you thirsty?");

    let thirst = get_input();
    if thirst == "yes" {
        println!("Scale of 1-10, how thirsty?");
        let thirst_level = get_number();
        if thirst_level >= 1 && thirst_level <= 3 {
            println!("You don't need water");
        } else if thirst_level >= 4 && thirst_level <= 6 {
            println!("Have a samll glass");
        } else if thirst_level > 6 && thirst_level <= 10 {
            println!("Drink a bottle of water");
        }
    } else if thirst == "no" {
        println!("Ok :)");
    } else {
        println!("fuck you");
    }
}

// get_input() is input from the terminal and my_stdin is my terminal input - NOTE my stdout is my terminal output
fn get_input() -> String {
    let mut input = String::new();
    let my_stdin = stdin();
    my_stdin.read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();
    input.to_string()
}

fn get_number() -> u32 {
    let number: u32 = get_input().parse().unwrap();
    number // this is how I return the output
}
