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
        let actions = vec![""];

        for action in &actions {
            action_queue.add_action((*action).to_owned());
        }
        let mut retrieved_actions = Vec::<Action>::new();
        while let Some(action) = action_queue.get_next_action() {
            retrieved_actions.push(action);
        }

        assert_eq!(actions, retrieved_actions);
    }
}
