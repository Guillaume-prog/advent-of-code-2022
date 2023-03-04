#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Rock, Paper, Scissors
}

pub enum Outcome {
    Win, Loss, Draw
}

impl Shape {

    // Returns the outcome of the match between this shape and another shape
    pub fn outcome_against(&self, other: &Shape) -> Outcome {
        if self == other {
            return Outcome::Draw
        } else if &self.wins_against() == other {
            return Outcome::Win
        } else {
            return Outcome::Loss
        }
    }

    // Returns the shape this shape will win against
    pub fn wins_against(&self) -> Shape {
        match self {
            Shape::Rock     => Shape::Scissors,
            Shape::Paper    => Shape::Rock,
            Shape::Scissors => Shape::Paper
        }
    }

    // Returns the shape this shape will lose to
    pub fn loses_to(&self) -> Shape {
        match self {
            Shape::Rock     => Shape::Paper,
            Shape::Paper    => Shape::Scissors,  
            Shape::Scissors => Shape::Rock
        }
    }
}