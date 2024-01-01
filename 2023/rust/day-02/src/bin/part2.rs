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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_fn() -> Result<(), Error> {
        let test_input = "";
        let result = process(test_input);
        assert_eq!("", result?);
        Ok(())
    }
}
