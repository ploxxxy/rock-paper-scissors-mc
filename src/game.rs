use rand::{thread_rng, Rng};

pub enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn beats(&self, other: &Choice) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }

        match (self, other) {
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }
}

pub fn get_random_choice() -> Choice {
    let choices = [Choice::Rock, Choice::Paper, Choice::Scissors];
    let index = thread_rng().gen_range(0..3);
    choices[index]
}
