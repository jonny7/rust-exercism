pub fn reply(message: &str) -> &str {

    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if m.to_uppercase() == m && m.chars().fold(false, |acc, c| char::is_alphabetic(c) || acc) => "Whoa, chill out!",
        m if m.ends_with('?') => "Sure.",
        _ => "Whatever.",
    }
}

