fn main() {
    let status_updates: Vec<&str> = vec![
        "A tavern and a smithy are nearby.",
        "You decide to enter the tavern.",
        "The tavern is entirely empty of people, but the bar seems well-stocked.",
        "You decide to make yourself a drink. You make a 'Rusty Nail', a drink consisting of Scotch Whisky and Drambuie. The irony of mixing whisky with whisky is not lost on you.",
    ];
    #[expect(clippy::print_stdout, reason = "I do what I want.")]
    for update in status_updates {
        println!("{update}");
    }
}

#[cfg(test)]
mod tests {}
