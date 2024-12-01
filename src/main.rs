mod action_queue;

use action_queue::ActionQueue;

fn main() {
    let surroundings: Vec<&str> = vec![
        "A tavern and a smithy are nearby.",
        "The tavern is entirely empty of people, but the bar seems well-stocked.",
    ];
    let mut action_queue = ActionQueue::new();

    action_queue.add_action("You decide to enter the tavern.".to_owned());
    action_queue.add_action("You decide to make yourself a drink. You make a 'Rusty Nail', a drink consisting of Scotch Whisky and Drambuie. The irony of mixing whisky with whisky is not lost on you.".to_owned());

    #[expect(clippy::print_stdout, reason = "I do what I want.")]
    for description in surroundings {
        println!("{description}");
        if let Some(action) = action_queue.get_next_action() {
            println!("{action}");
        }
    }
}

#[cfg(test)]
mod tests {}
