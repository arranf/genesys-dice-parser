
/// Represents the type of dice that can be rolled
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dice {
    /// The Boost Dice is a blue d6.
    Boost,
    /// The Ability Dice is a green d8.
    Ability,
    /// The Proficiency Dice is a yellow d12.
    Proficiency,
    /// The Setback Dice is a black d6.
    Setback,
     /// The Difficulty Dice is a purple d8.
    Difficulty,
    /// The Challenge Dice is a red d12.
    Challenge,
    /// The Force is a white d12.
    Force
}
