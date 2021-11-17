use rand::Rng;

const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%*()-";

pub enum Valid {
    Yes,
    No
}

pub struct Password {
    pub is_valid: Valid,
    pub status: Vec<String>
}

pub fn generate(&len: &usize) -> String {
    let mut final_str = String::new();
    for _ in 1..len {
        let random = rand::thread_rng().gen_range(0..CHARS.len()-1);
        let letter = &CHARS[random..=random+1];
        final_str.push_str(letter);
    };
    
    final_str
}

pub fn validate(pass: &str) -> Password {
    let mut valid = Valid::Yes;
    let mut status = Vec::new();
    let messages = [
        "Dosent have atleast 8 characters",
        "Dosent have uppercase and/or letters",
        "Dosent have a special character",
    ];

    if pass.len() < 8 {
        valid = Valid::No;
        status.push(messages[0].to_string());
    } 
    if !pass.chars().any(|c| c.is_uppercase()) {
        valid = Valid::No;
        status.push(messages[1].to_string());
    }
    if !pass.chars().any(|c| c.is_ascii_punctuation()) {
        valid = Valid::No;
        status.push(messages[2].to_string());
    }

    Password {
        is_valid: valid,
        status: status
    }
}