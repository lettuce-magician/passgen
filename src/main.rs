// Thanks Github compilot for the snippets and shit i didnt know.

// Known Bugs:
// Validating a password with an & on windows (and probably other OS's) will try to run another program with it.

mod pass;
use pass::Valid;
use std::env;

const USAGE: &str = "Usage: passgen (validate/generate) [password/length]";

fn main() {
    let arg: Vec<String> = env::args().collect();
    let len = arg.len();

    if len < 2 {
        println!("{}", USAGE);
    } else if arg[1] == "validate" {
        let pass = arg[2].to_string();
        let result = pass::validate(&pass);
        
        match result.is_valid {
            Valid::Yes => {
                println!("Your password is strong.");
            }
            Valid::No => {
                println!("Your password haves {} problems (2 or above means you have to change it!):", result.status.len());
                for msg in result.status {
                    println!("{}", msg);
                }
            }
        }
    } else if arg[1] == "generate" {
        let pass = pass::generate(&arg[2].parse::<usize>().unwrap());
        println!("{}", pass);
    } else {
        println!("{}", USAGE);
    }
}