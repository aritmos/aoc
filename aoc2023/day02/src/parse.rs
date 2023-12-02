use nom::{
    bytes::complete::*,
    character::complete::{alphanumeric1, newline, space1, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::hand::Hand;

pub fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Hand>>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Hand>> {
    let (hands, _) = take_until(": ")(input)?;
    let (hands, _) = take(2_usize)(hands)?;
    separated_list1(tag("; "), parse_hand)(hands)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (rest, dice) =
        separated_list1(tag(", "), separated_pair(u32, space1, alphanumeric1))(input)?;
    let mut hand = Hand::default();
    for (n, color) in dice {
        match color {
            "red" => hand.red = n,
            "green" => hand.green = n,
            "blue" => hand.blue = n,
            _ => unreachable!(),
        }
    }
    Ok((rest, hand))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("../inputs/sample.txt");
        let res = parse_input(input);
        assert!(res.is_ok());
        println!("{:?}", res.unwrap().1);
    }
}
