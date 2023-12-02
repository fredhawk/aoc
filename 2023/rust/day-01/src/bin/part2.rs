use anyhow::Error;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(_input: &str) -> Result<String, Error> {
    // take input and split to lines
    let output = _input
        .lines()
    // parse each line
        .map(process_line)
    // add all the numbers together
        .sum::<u32>();

    Ok(output.to_string())
}

fn process_line(line: &str) -> u32 {
    // process each line
    let mut item = (0..line.len()).filter_map(|idx| {
        // keep track of the index and walk over each index and build up the string slice until it
        // matches a case.
        let reduced_line = &line[idx..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        // When a result is found return it
        result.to_digit(10)
    });

    // take the first digit
    let first = item.next().expect("Should be a number");

    // take the last digit
    let last = item.last();

    // concatenate the digits based on if one or two exits on the line
    match last {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    // return the number
    .parse::<u32>()
    .expect("Should be a number")

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_fn() -> Result<(), Error> {
        let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = process(test_input);
        assert_eq!("281", result?);
        Ok(())
    }
}
