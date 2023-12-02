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
        .map(|line| {
            let mut item = line.chars().filter_map(|char| {
                char.to_digit(10)
            });

    // take the first and last number 
            let first = item.next().expect("Should be a number");
            let last = item.last();
            
            match last {
                Some(num) => {
                    format!("{first}{num}")
                },
    // if there is only one number, put it in twice
                None => format!("{first}{first}")
            }
            .parse::<u32>()
            .expect("Should be a number.")

        })
    // add all the numbers together
        .sum::<u32>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Error> {
        let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process(test_input);
        assert_eq!("142", result?);
        Ok(())
    }
}
