use nom::bytes::complete::{tag, take_while};
use nom::bytes::streaming::take_until;
use nom::sequence::tuple;
use nom::IResult;

pub fn contents_as_string<'a>(i: &'a str, command_name: &str) -> IResult<&'a str, String> {
    let space = take_while(|c: char| c == ' ');
    let command = tag(command_name);
    let contents = take_until("$end");
    let end = tag("$end");

    let (input, (_, _, contents, _)) = tuple((space, command, contents, end))(i)?;

    Ok((
        input,
        contents
            .trim()
            .to_string()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" "),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            contents_as_string("$command fuzzy wuzzy wuz a bear $end", "$command"),
            Ok(("", "fuzzy wuzzy wuz a bear".to_string()))
        );

        assert_eq!(
            contents_as_string("$qwerty  fuzzy  wuzzy  had  no  hair  $end", "$qwerty"),
            Ok(("", "fuzzy wuzzy had no hair".to_string()))
        );
    }
}
