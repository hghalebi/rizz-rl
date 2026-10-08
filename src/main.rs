#![allow(dead_code)]
use tracing::{Level, debug, debug_span, info, info_span, warn};
fn init_racing() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .compact()
        .init();
}
#[derive(Debug, Clone)]
struct Dist<T> {
    items: Vec<(T, f64)>,
}

impl<T> Dist<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> Dist<U> {
        Dist {
            items: self
                .items
                .into_iter()
                .map(|(value, probability)| (f(value), probability))
                .collect(),
        }
    }

    fn flat_map<U>(self, f: impl Fn(T) -> Dist<U>) -> Dist<U> {
        let mut out = Vec::new();
        for (current, probaility_so_far) in self.items {
            let futures = f(current);
            for (future, branch_probability) in futures.items {
                let probability = probaility_so_far * branch_probability;
                out.push((future, probability));
            }
        }
        Dist { items: out }
    }
    fn total_probability(&self) -> f64 {
        self.items.iter().map(|(_, probability)| probability).sum()
    }
}

impl<T: PartialEq> Dist<T> {
    fn probability_of(&self, traget: &T) -> f64 {
        self.items
            .iter()
            .filter(|(value, _)| value == traget)
            .map(|(_, probability)| probability)
            .sum()
    }
}
#[derive(Clone, Debug)]
struct Transition {
    state: State,
    action: Action,
    reward: f64,
    next: State,
}
type Trajectory = Vec<Transition>;
fn reward(state: &State, action: &Action, next: State) -> f64 {
    match (state, action, next) {
        (State::LowInterest, Action::Coffee, _) => -12.0,
        (State::Engaged, Action::Coffee, State::LowInterest) => -8.0,
        (_, _, State::MutualInterest) => 0.8,
        (State::Engaged, Action::Tease, State::Playful) => 3.0,
        (State::LowInterest, Action::Space | Action::Goodbye, State::GoodBye) => 2.0,
        (_, _, State::LowInterest) => -2.0,
        (_, Action::Ask | Action::Story, State::Playful | State::Engaged) => 1.0,
        _ => 0.0,
    }
}

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

fn policy(state: &State) -> Dist<Action> {
    match state {
        State::Engaged => Dist {
            items: vec![
                (Action::Goodbye, 0.0),
                (Action::Ask, 0.4),
                (Action::Story, 0.3),
                (Action::Tease, 0.2),
                (Action::Coffee, 0.1),
            ],
        },
        State::Playful => Dist {
            items: vec![
                (Action::Tease, 0.5),
                (Action::Coffee, 0.3),
                (Action::Ask, 0.2),
            ],
        },
        State::LowInterest => Dist {
            items: vec![(Action::Space, 0.6), (Action::Goodbye, 0.2)],
        },
        State::MutualInterest => Dist {
            items: vec![(Action::Coffee, 0.6), (Action::Story, 0.4)],
        },
        State::JustMet => Dist {
            items: vec![(Action::Ask, 0.7), (Action::Story, 0.3)],
        },
        State::GoodBye => Dist {
            items: vec![(Action::Goodbye, 1.0)],
        },
    }
}

fn world(state: &State, action: &Action) -> Dist<State> {
    match (state, action) {
        (State::JustMet, Action::Ask) => Dist {
            items: vec![(State::Engaged, 0.8), (State::LowInterest, 0.2)],
        },
        (State::JustMet, Action::Story) => Dist {
            items: vec![(State::Engaged, 0.60), (State::LowInterest, 0.40)],
        },
        (State::Engaged, Action::Ask) => Dist {
            items: vec![
                (State::Engaged, 0.65),
                (State::Playful, 0.20),
                (State::LowInterest, 0.15),
            ],
        },

        (State::Engaged, Action::Story) => Dist {
            items: vec![
                (State::Engaged, 0.55),
                (State::Playful, 0.30),
                (State::LowInterest, 0.15),
            ],
        },
        (State::Engaged, Action::Tease) => Dist {
            items: vec![
                (State::Playful, 0.55),
                (State::Engaged, 0.25),
                (State::LowInterest, 0.20),
            ],
        },
        (State::Engaged, Action::Coffee) => Dist {
            items: vec![(State::MutualInterest, 0.15), (State::LowInterest, 0.85)],
        },
        (State::Playful, Action::Ask) => Dist {
            items: vec![
                (State::Playful, 0.55),
                (State::Engaged, 0.35),
                (State::LowInterest, 0.10),
            ],
        },
        (State::Playful, Action::Tease) => Dist {
            items: vec![
                (State::Playful, 0.50),
                (State::Engaged, 0.20),
                (State::LowInterest, 0.30),
            ],
        },
        (State::Playful, Action::Coffee) => Dist {
            items: vec![(State::MutualInterest, 0.75), (State::LowInterest, 0.25)],
        },
        (State::LowInterest, Action::Space | Action::Goodbye) => Dist {
            items: vec![(State::GoodBye, 1.0)],
        },
        (State::MutualInterest, Action::Goodbye) => Dist {
            items: vec![(State::GoodBye, 1.0)],
        },
        (_, Action::Space | Action::Goodbye) => Dist {
            items: vec![(State::GoodBye, 1.0)],
        },
        (state, _) => Dist {
            items: vec![(state.clone(), 1.0)],
        },
    }
}
fn step_manual(state: &State) -> Dist<State> {
    let mut out = Vec::new();
    for (action, action_probability) in policy(state).items {
        let consecuences = world(state, &action);

        for (next, world_probability) in consecuences.items {
            let path_probability = action_probability * world_probability;
            out.push((next, path_probability));
        }
    }
    Dist { items: out }
}

fn step(state: &State) -> Dist<State> {
    policy(state).flat_map(|action| world(state, &action))
}
fn transition(state: &State) -> Dist<Transition> {
    policy(state).flat_map(|action| {
        world(state, &action).flat_map(|next| Dist {
            items: vec![(
                Transition {
                    state: state.clone(),

                    action: action.clone(),
                    reward: reward(state, &action, next.clone()),
                    next,
                },
                1.0,
            )],
        })
    })
}
fn main() {
    init_racing();
    println!("Hello, world!");
    let s1 = State::JustMet;

    let a1 = Action::Ask;
    info!("{:?}-> {:?}", s1, a1);

    println!("-------");
    let futures = world(&State::Engaged, &Action::Tease);
    for (state, probability) in &futures.items {
        info!("{:.0}% -> {state:?}", probability * 100.0,);
    }
    println!("-------");
    info!("total probability= {:.4}", futures.total_probability());
    let future_path = step_manual(&State::Engaged);
    for (state, probability) in future_path.items {
        info!("{probability:.4} -> {state:?}");
    }
    println!("---------------");
    let future_trajectory = step(&State::Engaged);
    for (state, probability) in future_trajectory.items {
        info!("{probability:.4} -> {state:?}");
    }
    println!("------- transition-------");
    let posibilities = transition(&State::Engaged);
    for (transition, probabilitis) in posibilities.items {
        info!(
            "{:.4}: {:?} --{:?} / {:+.1}--> {:?}",
            probabilitis, transition.state, transition.action, transition.reward, transition.next,
        );
    }
}
