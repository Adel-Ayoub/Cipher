use std::path::{Path, PathBuf};

use rpassword::prompt_password;
use rprompt::prompt_reply;

use super::messages;

#[derive(Debug)]
pub enum Action {
    Encrypt,
    Decrypt,
    Exit,
}

pub fn welcome() {
    println!("{}", messages::WELCOME);
}

pub fn get_action() -> Option<Action> {
    println!("{}", messages::ACTIONS);
    let choice = match prompt_reply(messages::PROMPT).ok()?.as_str() {
        "1" => Some(Action::Encrypt),
        "2" => Some(Action::Decrypt),
        "3" => Some(Action::Exit),
        _ => None,
    };
    choice
}

pub fn get_password() -> String {
    return match prompt_password("enter password: ") {
        Ok(pw) => pw,
        Err(_) => {
            println!("invalid input");
            get_password()
        }
    };
}

pub fn get_input(prompt: &str) -> String {
    return match prompt_reply(prompt) {
        Ok(i) => i,
        Err(_) => {
            println!("invalid input");
            get_input(prompt)
        }
    };
}

pub fn select_file_checked() -> PathBuf {
    let input = get_input("enter a filepath: ");
    let path = Path::new(&input);
    if path.exists() {
        return path.to_owned();
    } else {
        return select_file_checked().to_owned();
    }
}

pub fn select_file() -> PathBuf {
    let input = get_input("enter a filepath: ");
    let path = Path::new(&input);
    path.to_owned()
}

pub fn select_rounds() -> u32 {
    return match get_input("how many rounds?: ").parse::<u32>() {
        Ok(r) => r,
        Err(_) => {
            println!("invalid input");
            select_rounds()
        }
    }
}
