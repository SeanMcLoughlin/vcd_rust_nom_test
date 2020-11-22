use crate::helpers::strip;
use crate::types::{Command, TimeScale, TimeUnit};
use nom::bytes::complete::take_while1;
use nom::sequence::tuple;
use nom::{AsChar, IResult};
use std::str::FromStr;

fn contents(i: &str) -> IResult<&str, Box<dyn Command>> {
    let time_value = strip(take_while1(move |c: char| c.is_digit(10)));
    let time_unit = strip(take_while1(move |c: char| c.is_alpha()));

    let (input, (time_value, time_unit)) = tuple((time_value, time_unit))(i)?;

    Ok((
        input,
        Box::new(TimeScale(
            time_value.parse().unwrap(),
            TimeUnit::from_str(time_unit).unwrap(),
        )),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contents() {
        let my_contents = contents("1 ps").unwrap();
        assert_eq!(my_contents.0, "");

        let exp_timescale: TimeScale = *my_contents.1;
        let act_ti
        assert_eq!(*my_contents.1, *Box::new(TimeScale(1, TimeUnit::PS)));

        // assert_eq!(contents("2 us"), Ok(("", TimeScale(2, TimeUnit::US))));
    }

    #[test]
    fn extra_whitespace() {
        // assert_eq!(contents("3    ms"), Ok(("", TimeScale(3, TimeUnit::MS))));
        // assert_eq!(contents("  3  ms  "), Ok(("", TimeScale(3, TimeUnit::MS))));
        // assert_eq!(contents("3  ms "), Ok(("", TimeScale(3, TimeUnit::MS))));
    }

    #[test]
    fn newlines() {
        //         assert_eq!(
        //             contents(
        //                 r#"4
        // ns"#
        //             ),
        //             Ok(("", TimeScale(4, TimeUnit::NS)))
        //         );
    }
}
