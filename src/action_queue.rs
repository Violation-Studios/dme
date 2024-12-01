extern crate alloc;
use alloc::collections::vec_deque::VecDeque;

pub type Action = String;

pub struct ActionQueue {
    actions: VecDeque<Action>,
}
impl ActionQueue {
    pub const fn new() -> Self {
        Self {
            actions: VecDeque::new(),
        }
    }
    pub fn add_action(&mut self, action: Action) {
        self.actions.push_back(action);
    }
    pub fn get_next_action(&mut self) -> Option<Action> {
        self.actions.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actions_are_retrieved_in_first_in_first_out_order() {
        let mut action_queue = ActionQueue::new();
        let actions = vec!["1", "2", "3"];

        add_actions_to_queue(&actions, &mut action_queue);
        let retrieved_actions = get_actions_from_queue(action_queue);

        assert_eq!(actions, retrieved_actions);
    }
    fn add_actions_to_queue(actions: &[&str], queue: &mut ActionQueue) {
        actions
            .iter()
            .for_each(|action| queue.add_action((*action).to_owned()));
    }
    fn get_actions_from_queue(mut action_queue: ActionQueue) -> Vec<Action> {
        let mut retrieved_actions = Vec::<Action>::new();
        while let Some(action) = action_queue.get_next_action() {
            retrieved_actions.push(action);
        }
        retrieved_actions
    }
}
