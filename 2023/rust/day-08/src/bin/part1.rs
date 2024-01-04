use anyhow::Error;
use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alpha1, line_ending, multispace1,
    },
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[allow(clippy::type_complexity)]
fn parser(
    input: &str,
) -> IResult<
    &str,
    (
        Vec<Direction>,
        BTreeMap<&str, (&str, &str)>,
    ),
> {
    let (input, instructions) = many1(alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('L').map(|_| Direction::Left),
    )))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alpha1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(
                        alpha1,
                        tag(", "),
                        alpha1,
                    ),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>,
         (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;

    Ok((input, (instructions, map)))
}

fn process(_input: &str) -> Result<String, Error> {
    let (input, (instructions, map)) =
        parser(_input).expect("should validly parse");

    debug_assert_eq!(input, "");

    let mut current_node = "AAA";
    let Some(step_count) =
        instructions.iter().cycle().enumerate().find_map(
            |(index, instruction)| {
                let options = map
                    .get(current_node)
                    .expect("always exist at a valid node");
                let next_node = match instruction {
                    Direction::Left => options.0,
                    Direction::Right => options.1,
                };
                if next_node == "ZZZ" {
                    Some(index + 1)
                } else {
                    current_node = next_node;
                    None
                }
            },
        )
    else {
        panic!("infinite iterator can't produce None")
    };

    Ok(step_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Error> {
        let test_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = process(test_input);
        assert_eq!("2", result?);
        Ok(())
    }
}
