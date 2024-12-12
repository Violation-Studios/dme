mod action_queue;
mod zone;

use action_queue::ActionQueue;
use zone::Zone;

#[expect(clippy::print_stdout, reason = "I do what I want.")]
fn main() {
    let tavern = Zone::new(
        "The Rusty Nail",
        "The tavern is entirely empty of people, but the bar seems well-stocked.",
    );
    let current_zone = &tavern;

    let mut action_queue = ActionQueue::new();

    action_queue.add_action("You decide to make yourself a drink. You make a 'Rusty Nail', a drink consisting of Scotch Whisky and Drambuie. The irony of mixing whisky with whisky is not lost on you.".to_owned());

    println!("|{}|", current_zone.name());
    println!("{}", current_zone.description());
    while let Some(action) = action_queue.get_next_action() {
        println!("{action}");
    }
}

#[cfg(test)]
mod tests {}
