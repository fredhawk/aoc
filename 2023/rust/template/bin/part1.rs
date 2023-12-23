use anyhow::Error;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(_input: &str) -> Result<String, Error> {
    // take input and split to lines
    let output = _input
        .lines()
    // parse each line
        .map(|line| { })
        .sum::<u32>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Error> {
        let test_input = "";
        let result = process(test_input);
        assert_eq!("", result?);
        Ok(())
    }
}
