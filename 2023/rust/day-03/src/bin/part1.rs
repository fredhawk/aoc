use std::collections::HashSet;

use anyhow::Error;
use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
    character::complete::digit1,
    combinator::iterator,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Empty,
    Symbol(SpanIVec2<'a>),
    Number(SpanIVec2<'a>),
}

fn with_xy(span: Span) -> SpanIVec2 {
    // column/location are 1-indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}
fn parse_grid(input: Span) -> IResult<Span, Vec<Value>> {
    let mut it = iterator(
        input,
        alt((
            digit1
                .map(|span| with_xy(span))
                .map(Value::Number),
            is_not(".\n0123456789")
                .map(|span| with_xy(span))
                .map(Value::Symbol),
            take_till1(|c: char| {
                c.is_ascii_digit() || c != '.' && c != '\n'
            })
            .map(|_| Value::Empty),
        )),
    );

    let parsed = it
        .filter(|value| value != &Value::Empty)
        .collect::<Vec<Value>>();
    let res: IResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}
fn process(_input: &str) -> Result<String, Error> {
    let objects = parse_grid(Span::new(_input)).unwrap().1;

    let symbol_map = objects
        .iter()
        .filter_map(|value| match value {
            Value::Empty => None,
            Value::Symbol(sym) => Some(sym.extra),
            Value::Number(_) => None,
        })
        .collect::<HashSet<IVec2>>();

    let result = objects
        .iter()
        .filter_map(|value| {
            let Value::Number(num) = value else {
                return None;
            };
            let surrounding_positions = [
                // east border
                IVec2::new(num.fragment().len() as i32, 0),
                // west border
                IVec2::new(-1, 0),
            ]
            .into_iter()
            .chain(
                // north border
                (-1..=num.fragment().len() as i32).map(
                    |x_offset| IVec2::new(x_offset, 1),
                ),
            )
            .chain(
                // south border
                (-1..=num.fragment().len() as i32).map(
                    |x_offset| IVec2::new(x_offset, -1),
                ),
            )
            .map(|pos| pos + num.extra)
            .collect::<Vec<IVec2>>();

            surrounding_positions
                .iter()
                .any(|pos| symbol_map.contains(pos))
                .then_some(
                    num.fragment()
                        .parse::<u32>()
                        .expect("should be a valid number"),
                )
        })
        .sum::<u32>();

    Ok(result.to_string())
}
//
// fn process(_input: &str) -> Result<String, Error> {
//     // take input and split to lines
//     let map = _input
//         .lines()
//         .enumerate()
//         .flat_map(|(y, line)| {
//             line.chars().enumerate().map(move |(x, character)| {
//                 (
//                     (y as i32, x as i32),
//                     match character {
//                         '.' => Value::Empty,
//                         c if c.is_ascii_digit() => {
//                             Value::Number(c.to_digit(10).expect("should be a number"))
//                         }
//                         c => Value::Symbol(c),
//                     },
//                 )
//             })
//         })
//         .collect::<BTreeMap<(i32, i32), Value>>();
//
//     let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
//     for ((y, x), value) in map.iter() {
//         if let Value::Number(num) = value {
//             match numbers.iter().last() {
//                 Some(v) => {
//                     let last_num = v.iter().last();
//                     match last_num {
//                         Some(((last_num_x, _), _)) => {
//                             if last_num_x + 1 == *x {
//                                 let last = numbers.iter_mut().last().expect("should exist");
//                                 last.push(((*x, *y), *num));
//                             } else {
//                                 numbers.push(vec![((*x, *y), *num)]);
//                             }
//                         }
//                         None => unimplemented!("shouldn't happen"),
//                     }
//                 }
//                 None => {
//                     numbers.push(vec![((*x, *y), *num)]);
//                 }
//             }
//         }
//     }
//
//     // map: entire grid
//     // numbers: sequential numbers
//     let mut total = 0;
//     for num_list in numbers {
//         // (x,y)
//         let positions = [
//             (1, 0),
//             (1, -1),
//             (0, -1),
//             (-1, -1),
//             (-1, 0),
//             (-1, 1),
//             (0, 1),
//             (1, 1),
//         ];
//         let num_positions: Vec<(i32, i32)> = num_list.iter().map(|((y, x), _)| (*x, *y)).collect();
//         let pos_to_check: Vec<(i32, i32)> = num_list
//             .iter()
//             .flat_map(|(pos, _)| {
//                 positions.iter().map(|outer_pos| {
//                     // outer_pos.x + pos.x, .y + .y
//                     (outer_pos.0 + pos.1, outer_pos.1 + pos.0)
//                 })
//             })
//             .unique()
//             .filter(|num| !num_positions.contains(num))
//             .collect();
//
//         // dbg!(pos_to_check.len(), pos_to_check);
//         let is_part_number = pos_to_check.iter().any(|pos| {
//             let value = map.get(pos);
//             #[allow(clippy::match_like_matches_macro)]
//             if let Some(Value::Symbol(_)) = value {
//                 true
//             } else {
//                 false
//             }
//         });
//
//         if is_part_number {
//             total += num_list
//                 .iter()
//                 .map(|(_, num)| num.to_string())
//                 .collect::<String>()
//                 .parse::<u32>()
//                 .unwrap()
//         }
//     }
//
//     Ok(total.to_string())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Error> {
        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(test_input);
        assert_eq!("4361", result?);
        Ok(())
    }
}
