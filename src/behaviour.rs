use super::events::Event;

pub trait BehaviourEventProcess {
    fn income_event(&mut self, event: Event);
}
