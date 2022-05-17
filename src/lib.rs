#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::pedantic::module_name_repetitions)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This crate provides functionality for the basic parsing of dice roll commands e.g. `yyypp`, `2g1y2p`, `difficulty difficulty ability proficiency`.
//! Given some input it will produce a `DiceRoll` struct which can be used to then calculate a result.


use std::collections::HashMap;

use nom::{branch, bytes, multi, character, combinator, sequence, Err};

/// Provides access to the `DiceRoll` struct.
pub mod dice_roll;
/// Provides access to the `ParserError` struct.
pub mod error;
/// Provices access to the `Dice` enum.
pub mod dice;


use crate::dice_roll::{DiceRoll};
use crate::dice::Dice;
use crate::error::ParserError;


// boost or blue or b
fn parse_dice_as_value(i: &str) -> nom::IResult<&str, Dice> {
    branch::alt((
        combinator::value(Dice::Ability, branch::alt((bytes::complete::tag_no_case("green"), bytes::complete::tag_no_case("g"), bytes::complete::tag_no_case("ability"), bytes::complete::tag_no_case("abil")))),
        combinator::value(Dice::Challenge, branch::alt((bytes::complete::tag_no_case("challenge"), bytes::complete::tag_no_case("cha"), bytes::complete::tag_no_case("red"), bytes::complete::tag_no_case("r"), ))),
        combinator::value(Dice::Proficiency, branch::alt((bytes::complete::tag_no_case("proficiency"), bytes::complete::tag_no_case("prof"), bytes::complete::tag_no_case("yellow"), bytes::complete::tag_no_case("y"),))),
        combinator::value(Dice::Difficulty, branch::alt((bytes::complete::tag_no_case("difficulty"), bytes::complete::tag_no_case("purple"), bytes::complete::tag_no_case("p"), bytes::complete::tag_no_case("diff"), bytes::complete::tag_no_case("dif")))),
        combinator::value(Dice::Setback, branch::alt((bytes::complete::tag_no_case("black"), bytes::complete::tag_no_case("k"), bytes::complete::tag_no_case("setback"), bytes::complete::tag_no_case("s"), ))),
        combinator::value(Dice::Force, branch::alt((bytes::complete::tag_no_case("force"), bytes::complete::tag_no_case("white"), bytes::complete::tag_no_case("w"), ))),
        combinator::value(Dice::Boost, branch::alt((bytes::complete::tag_no_case("blue"), bytes::complete::tag_no_case("boost"), bytes::complete::tag_no_case("b")))),
    ))(i)
}

// Matches: 2g or ggbbfd
fn parse_dice(i: &str) -> nom::IResult<&str, DiceRoll> {
    let result = sequence::tuple((
        combinator::opt(character::complete::digit1), parse_dice_as_value ))(i);
    match result {
        Ok((remaining, (number_of_dice, dice))) => Ok((
            remaining,
            DiceRoll::new(dice, number_of_dice.map_or(Ok(1), str::parse).unwrap()),
        )),
        Err(e) => Err(e),
    }
}

fn parse_group(i: &str) -> nom::IResult<&str, Vec<DiceRoll>> {
    let (remaining, rolls) = multi::many1(parse_dice)(i)?;
    
    let mut dice_counts: HashMap<Dice, u32> = HashMap::new();

    rolls.into_iter().for_each(|roll| {
        let group = dice_counts.entry(roll.die).or_insert(0);
       *group += roll.number_of_dice_to_roll;
    });

    let rolls = dice_counts.into_iter().map(|(key, value)| DiceRoll::new(key, value)).collect();
    Ok((remaining, rolls))
}

fn parse_groups(i: &str) -> nom::IResult<&str, Vec<Vec<DiceRoll>>> {
    let (remaining, (group_rolls, other_groups)) = sequence::tuple((
        parse_group,
        combinator::opt(sequence::tuple((
            character::complete::char(','),
            parse_groups,
        ))),
    ))(i)?;

    let other_groups_size = match &other_groups {
        Some((_, rolls)) => rolls.len(),
        None => 0,
    };

    let mut rolls: Vec<Vec<DiceRoll>> = Vec::with_capacity(other_groups_size + 1);
    rolls.push(group_rolls);
    if other_groups.is_some() {
        let (_, other_groups_rolls) = other_groups.unwrap();
        rolls.extend(other_groups_rolls);
    }
    Ok((remaining, rolls))
}

/// Takes a string of dice input and returns a `Result<DiceRoll, ParserError>`
///
/// The string will be consumed in the process and must strictly match the format of the parser.
///
/// # Examples
///
/// Standard usage:
///
/// ```
/// use dice_command_parser::{parse_line, error::ParserError};
///
/// let input = "2rkyyg";
/// let dice_roll = parse_line(&input)?;
/// # Ok::<(), ParserError>(())
/// ```
///
/// # Errors
/// This function can fail when one of the following occurs
/// 1. The line failed to parse.
/// 2. An error occurred parsing the numbers provided. This will likely be an overflow or underflow error.
///
/// For more information see `ParserError`.
pub fn parse_line(i: &str) -> Result<Vec<Vec<DiceRoll>>, ParserError> {
    let whitespaceless: String = i.replace(" ", "");

    match parse_groups(&whitespaceless) {
        Ok((remaining, dice_rolls)) => {
            if !remaining.trim().is_empty() {
                return Err(ParserError::ParseError(format!(
                    "Expected remaining input to be empty, found: {0}",
                    remaining
                )));
            }
            return Ok(dice_rolls);
        }
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
            return Err(ParserError::ParseError(format!("{0}", e)));
        }
        Err(Err::Incomplete(_)) => {
            return Err(ParserError::Unknown);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Dice;

    #[test]
    fn test_parse_dice_as_value() {
        // Ability dice
        assert_eq!(
            parse_dice_as_value("green"),
            Ok(("", Dice::Ability))
        );
        assert_eq!(
            parse_dice_as_value("g"),
            Ok(("", Dice::Ability))
        );
        assert_eq!(
            parse_dice_as_value("ability"),
            Ok(("", Dice::Ability))
        );
        assert_eq!(
            parse_dice_as_value("abil"),
            Ok(("", Dice::Ability))
        );

        assert_eq!(
            parse_dice_as_value("challenge"),
            Ok(("", Dice::Challenge))
        );
        assert_eq!(
            parse_dice_as_value("cha"),
            Ok(("", Dice::Challenge))
        );
        assert_eq!(
            parse_dice_as_value("red"),
            Ok(("", Dice::Challenge))
        );
        assert_eq!(
            parse_dice_as_value("r"),
            Ok(("", Dice::Challenge))
        );

        assert_eq!(
            parse_dice_as_value("Proficiency"),
            Ok(("", Dice::Proficiency))
        );
        assert_eq!(
            parse_dice_as_value("prof"),
            Ok(("", Dice::Proficiency))
        );
        assert_eq!(
            parse_dice_as_value("yellow"),
            Ok(("", Dice::Proficiency))
        );
        assert_eq!(
            parse_dice_as_value("y"),
            Ok(("", Dice::Proficiency))
        );
        
        assert_eq!(
            parse_dice_as_value("difficulty"),
            Ok(("", Dice::Difficulty))
        );
        assert_eq!(
            parse_dice_as_value("diff"),
            Ok(("", Dice::Difficulty))
        );
        assert_eq!(
            parse_dice_as_value("purple"),
            Ok(("", Dice::Difficulty))
        );
        assert_eq!(
            parse_dice_as_value("p"),
            Ok(("", Dice::Difficulty))
        );

        assert_eq!(
            parse_dice_as_value("black"),
            Ok(("", Dice::Setback))
        );
        assert_eq!(
            parse_dice_as_value("setback"),
            Ok(("", Dice::Setback))
        );
        assert_eq!(
            parse_dice_as_value("k"),
            Ok(("", Dice::Setback))
        );

        assert_eq!(
            parse_dice_as_value("force"),
            Ok(("", Dice::Force))
        );
        assert_eq!(
            parse_dice_as_value("white"),
            Ok(("", Dice::Force))
        );
        assert_eq!(
            parse_dice_as_value("w"),
            Ok(("", Dice::Force))
        );

        assert_eq!(
            parse_dice_as_value("blue"),
            Ok(("", Dice::Boost))
        );
        assert_eq!(
            parse_dice_as_value("b"),
            Ok(("", Dice::Boost))
        );
        assert_eq!(
            parse_dice_as_value("boost"),
            Ok(("", Dice::Boost))
        );
        assert!(parse_dice_as_value("6 + 2").is_err());
    }

    #[test]
    fn test_parse_dice() {
        assert_eq!(
            parse_dice("2p"),
            Ok(("", DiceRoll::new(Dice::Difficulty, 2)))
        );
        assert_eq!(
            parse_dice("6ryyy"),
            Ok(("yyy", DiceRoll::new(Dice::Challenge, 6)))
        );
        assert_eq!(
            parse_dice("ggbpp"),
            Ok(("gbpp", DiceRoll::new(Dice::Ability, 1)))
        );

        assert!(parse_dice("*1").is_err());
    }

    #[test]
    fn test_parse_group() {
        assert_eq!(parse_group("6ryyy"), Ok(("", vec![DiceRoll::new(Dice::Challenge, 6), DiceRoll::new(Dice::Proficiency, 3)])));
        assert_eq!(parse_group("d"), Ok(("", vec![DiceRoll::new(Dice::Difficulty, 1)])));
        assert_eq!(parse_group("ddkb"), Ok(("", vec![DiceRoll::new(Dice::Difficulty, 2), DiceRoll::new(Dice::Setback, 1), DiceRoll::new(Dice::Boost, 1)])));
    }
}
