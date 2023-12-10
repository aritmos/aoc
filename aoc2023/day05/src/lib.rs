use nom::{
    bytes::complete::*,
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    IResult,
};

pub type TriMap = (u64, u64, u64);

pub fn parse(input: &str) -> (Vec<u64>, Vec<Vec<TriMap>>) {
    parse_input(input).unwrap().1
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<Vec<TriMap>>)> {
    let (input, seed_line) = take_until("\n")(input)?;
    let (input, _) = take(2_u64)(input)?; // remove '\n\n'

    let (_, seeds) = parse_seeds(seed_line)?;
    let (_, maps) = separated_list1(tag("\n\n"), parse_map)(input)?;

    Ok(("", (seeds, maps)))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    // remove header
    let (input, _) = take_until(":")(input)?;
    let (input, _) = take(2_u64)(input)?;

    separated_list1(space1, u64)(input)
}

fn parse_map(input: &str) -> IResult<&str, Vec<TriMap>> {
    // remove header
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = take(1_u64)(input)?;

    separated_list1(newline, parse_trimap)(input)
}

fn parse_trimap(input: &str) -> IResult<&str, TriMap> {
    let (rest, trimap) = separated_list1(space1, u64)(input)?;
    assert_eq!(trimap.len(), 3);
    Ok((rest, (trimap[0], trimap[1], trimap[2])))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn parse_trimap_() {
    //     let input = "50 98 2";
    //     assert_eq!(parse_trimap(input), Ok(("", (50, 98, 2))));
    // }
    //
    // #[test]
    // fn parse_map_() {
    //     let input = "soil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15";
    //     assert_eq!(
    //         parse_map(input),
    //         Ok(("", vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)]))
    //     );
    // }
    //
    // #[test]
    // fn parse_seeds_() {
    //     let input = "seeds: 79 14 55 13";
    //     assert_eq!(parse_seeds(input), Ok(("", vec![79, 14, 55, 13])));
    // }

    #[test]
    fn parse_input_() {
        let input = include_str!("../inputs/sample.txt");
        let parsed_input = parse_input(input);
        // assert!(parsed_input.is_ok());
        println!("{:?}", parsed_input.unwrap().1);
    }
}
