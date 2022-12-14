use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(complete::i32, tag(","), complete::i32)(input)
}

pub fn line(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let (input, corners) = separated_list1(tag(" -> "), pair)(input)?;
    Ok((
        input,
        corners
            .iter()
            .zip(corners.iter().skip(1))
            .flat_map(|(&left, &right)| -> Vec<(i32, i32)> {
                if left.0 == right.0 {
                    let (start, end) = if left.1 < right.1 {
                        (left.1, right.1)
                    } else {
                        (right.1, left.1)
                    };
                    (start..end).map(|y| (left.0, y)).collect()
                } else if left.1 == right.1 {
                    let (start, end) = if left.0 < right.0 {
                        (left.0, right.0)
                    } else {
                        (right.0, left.0)
                    };
                    (start..=end).map(|x| (x, left.1)).collect()
                } else {
                    panic!("Nope!")
                }
            })
            .collect::<Vec<_>>(),
    ))
}

pub fn lines(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let (input, lines) = separated_list1(newline, line)(input)?;
    Ok((input, lines.into_iter().flatten().collect()))
}
