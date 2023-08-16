use nom::character::complete::alphanumeric1;
use nom::IResult;

pub enum Action {
    Checklist,
    Download,
    Unknown,
}

pub fn parse_command(command: &str) -> IResult<&str, &str> {
    let (rem, res) = alphanumeric1(command)?;
    Ok((rem, res))
}

pub fn match_command(command: &str) -> Action {
    match command {
        "checklist" => Action::Checklist,
        "download" => Action::Download,
        _ => Action::Unknown,
    }
}
