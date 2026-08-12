#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
enum State {
    JustMet,
    Engaged,
    Playful,
    LowInterest,
    MutualInterest,
    GoodBye,
}
#[derive(Clone, Debug)]
enum Action {
    Ask,
    Story,
    Tease,
    Coffee,
    Space,
    Goodbye,
}

fn pulicy(state: State) -> Action {
    match state {
        State::Engaged => Action::Story,
        State::GoodBye => Action::Goodbye,
        State::JustMet => Action::Ask,
        State::Playful => Action::Coffee,
        State::LowInterest => Action::Space,
        State::MutualInterest => Action::Coffee,
    }
}
fn main() {
    println!("Hello, world!");
    let s1 = State::JustMet;
    let a1 = Action::Ask;
    println!("{s1:?}->{a1:?}");
}
