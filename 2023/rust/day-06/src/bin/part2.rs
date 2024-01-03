use anyhow::Error;
use nom::{
    bytes::complete::is_not,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn nums(input: &str) -> IResult<&str, u64> {
    is_not("0123456789")
        .precedes(separated_list1(space1, digit1).map(
            |list| {
                list.join("")
                    .parse::<u64>()
                    .expect("a valid number")
            },
        ))
        .parse(input)
}
fn parse_times(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(nums, line_ending, nums).parse(input)
}

fn process(_input: &str) -> Result<String, Error> {
    let (_, (time, record_distance)) =
        parse_times(_input).expect("a valid parse");

    let result = (0..time)
        .filter_map(|speed| {
            let my_distance = (time - speed) * speed;
            (my_distance > record_distance)
                .then_some(my_distance)
        })
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_fn() -> Result<(), Error> {
        let test_input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(test_input);
        assert_eq!("71503", result?);
        Ok(())
    }
}
