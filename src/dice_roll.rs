use crate::dice::Dice;


#[derive(Clone, Debug, PartialEq)]
/// This struct represents the information required to calculate the result of a dice roll given the command string.
/// Validating the sanity of each of the parameters is left to the user. e.g. The number of dice to roll could be 0.
pub struct DiceRoll {
    /// How many dice should be rolled.
    pub number_of_dice_to_roll: u32,
    /// Which type of dice it is.
    pub die: Dice,
}

impl DiceRoll {
    /// A convinience method for creating a `DiceRoll`.
    ///
    /// # Examples
    ///
    /// This represents rolling six Boost dice
    /// ```
    /// use dice_command_parser::dice_roll::{DiceRoll, Dice};
    ///
    /// let dice_roll = DiceRoll::new(Dice::Boost, 6);
    /// ```
    #[must_use]
    pub fn new(
        die: Dice,
        number_of_dice_to_roll: u32,
    ) -> Self {
        DiceRoll {
            die,
            number_of_dice_to_roll,
        }
    }
}
