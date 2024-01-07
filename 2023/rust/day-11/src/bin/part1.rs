use anyhow::Error;
use glam::IVec2;
use itertools::Itertools;
use tracing::{info, span, Level};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(_input: &str) -> Result<String, Error> {
    let empty_rows = _input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            line.chars().all(|c| c == '.').then_some(index)
        })
        .collect::<Vec<usize>>();

    let mut columns = _input
        .lines()
        .map(|line| line.chars())
        .collect::<Vec<_>>();
    let empty_columns = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns {
            match iter.next() {
                Some(item) => {
                    items.push(item);
                }
                None => return None,
            }
        }
        Some(items)
    })
    .enumerate()
    .filter_map(|(index, column)| {
        column.iter().all(|c| c == &'.').then_some(index)
    })
    .collect::<Vec<usize>>();

    let galaxies = _input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, c)| {
                    if c == '#' {
                        Some(IVec2::new(x as i32, y as i32))
                    } else {
                        None
                    }
                },
            )
        })
        .collect::<Vec<IVec2>>();
    info!(?galaxies);
    let count = galaxies
        .iter()
        .combinations(2)
        .map(|s| {
            let galaxy_a_id = galaxies
                .iter()
                .position(|v| v == s[0])
                .unwrap()
                + 1;
            let galaxy_b_id = galaxies
                .iter()
                .position(|v| v == s[1])
                .unwrap()
                + 1;
            let my_span = span!(
                Level::INFO,
                "galaxy_map_span",
                ids=format!("{}-{}", galaxy_a_id.min(galaxy_b_id), galaxy_a_id.max(galaxy_b_id)),
                galaxy_a_id,
                galaxy_b_id,
                galaxy_a = ?galaxies
                    .iter()
                    .find(|v| v == &s[0])
                    .unwrap(),
                galaxy_b = ?galaxies
                    .iter()
                    .find(|v| v == &s[1])
                    .unwrap() // duplicates = acc.get(&index)
            );
            my_span.in_scope(|| {
                let galaxy_a_expanded = {
                    let expand_steps_row = empty_rows
                        .iter()
                        .position(|row| {
                            row > &(s[0].y as usize)
                        })
                        .unwrap_or(empty_rows.len());
                    let expand_steps_columns =
                        empty_columns
                            .iter()
                            .position(|column| {
                                column > &(s[0].x as usize)
                            })
                            .unwrap_or(empty_columns.len());

                    info!(
                        expand_steps_columns,
                        expand_steps_row
                    );

                    *s[0]
                        + IVec2::new(
                            (expand_steps_columns) as i32,
                            (expand_steps_row) as i32,
                        )
                };

                info!(?galaxy_a_expanded);

                let galaxy_b_expanded = {
                    let expand_steps_row = empty_rows
                        .iter()
                        .position(|row| {
                            row > &(s[1].y as usize)
                        })
                        .unwrap_or(empty_rows.len());
                    let expand_steps_columns =
                        empty_columns
                            .iter()
                            .position(|column| {
                                column > &(s[1].x as usize)
                            })
                            .unwrap_or(empty_columns.len());

                    info!(
                        expand_steps_columns,
                        expand_steps_row
                    );

                    *s[1]
                        + IVec2::new(
                            (expand_steps_columns) as i32,
                            (expand_steps_row) as i32,
                        )
                };

                info!(?galaxy_b_expanded);

                let v = (galaxy_a_expanded
                    - galaxy_b_expanded)
                    .abs();
                let distance = (v.x + v.y).abs();
                info!(?distance);
                distance
            })
        })
        .sum::<i32>();
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Error> {
        let test_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let result = process(test_input);
        assert_eq!("374", result?);
        Ok(())
    }
}
