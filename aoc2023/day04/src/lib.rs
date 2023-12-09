use nom::{
    bytes::complete::*,
    character::complete::{newline, space0, space1, u8},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub type Card = (Vec<u8>, Vec<u8>);

pub fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<u8>, Vec<u8>)> {
    let (input, _) = take_until(":")(input)?; // remove card number prefix
    let (input, _) = take(1_usize)(input)?;
    separated_pair(parse_nums, tag(" | "), parse_nums)(input)
}

fn parse_nums(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, _) = space0(input)?; // remove leading whitespace if first num has one digit
    separated_list1(space1, u8)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nums_parse() {
        let input = "41 48 83 86 17";
        assert_eq!(parse_nums(input).unwrap().1, vec![41, 48, 83, 86, 17]);

        let input = " 1 21 53 59 44";
        assert_eq!(parse_nums(input).unwrap().1, vec![1, 21, 53, 59, 44]);

        let input = "69 82 63 72 16 21 14  1";
        assert_eq!(
            parse_nums(input).unwrap().1,
            vec![69, 82, 63, 72, 16, 21, 14, 1]
        );
    }

    #[test]
    fn line_parse() {
        let input = " Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        assert_eq!(
            parse_line(input).unwrap().1,
            (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1])
        );
    }

    #[test]
    fn input_parse() {
        let input = include_str!("../inputs/sample.txt");
        assert!(parse_input(input).is_ok());
    }
}
