use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u8,
    multi::separated_list1,
    sequence::{pair, preceded},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Sample {
    red: Option<u8>,
    green: Option<u8>,
    blue: Option<u8>,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u8,
    samples: Vec<Sample>,
}

#[derive(Debug, PartialEq)]
pub struct MinimalGame {
    red: u32,
    green: u32,
    blue: u32,
}

impl MinimalGame {
    fn create(game: &Game) -> MinimalGame {
        let mut red = u32::MIN;
        let mut green = u32::MIN;
        let mut blue = u32::MIN;

        for sample in game.samples.iter() {
            if let Some(game_red) = sample.red {
                let game_red = game_red as u32;
                if game_red > red {
                    red = game_red;
                }
            }
            if let Some(game_green) = sample.green {
                let game_green = game_green as u32;
                if game_green > green {
                    green = game_green;
                }
            }
            if let Some(game_blue) = sample.blue {
                let game_blue = game_blue as u32;
                if game_blue > blue {
                    blue = game_blue;
                }
            }
        }

        MinimalGame { red, green, blue }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub fn parse_color(input: &str) -> IResult<&str, (u8, &str)> {
    pair(
        u8,
        preceded(tag(" "), alt((tag("red"), tag("green"), tag("blue")))),
    )(input)
}

pub fn parse_colors(input: &str) -> IResult<&str, Sample> {
    let (input, colors) = separated_list1(tag(", "), parse_color)(input)?;
    let mut sample = Sample {
        red: None,
        green: None,
        blue: None,
    };

    for color in colors {
        match color.1 {
            "red" => sample.red = Some(color.0),
            "green" => sample.green = Some(color.0),
            "blue" => sample.blue = Some(color.0),
            &_ => panic!("Wrong color"),
        }
    }

    Ok((input, sample))
}

pub fn parse_samples(input: &str) -> IResult<&str, Vec<Sample>> {
    separated_list1(tag("; "), parse_colors)(input)
}

pub fn game_parser(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = u8(input)?;
    let (input, samples) = preceded(tag(": "), parse_samples)(input)?;
    let game = Game { id, samples };

    Ok((input, game))
}

fn check_game(game: &Game) -> bool {
    for sample in game.samples.iter() {
        if let Some(red) = sample.red {
            if red > 12 {
                return false;
            };
        }

        if let Some(green) = sample.green {
            if green > 13 {
                return false;
            };
        }

        if let Some(blue) = sample.blue {
            if blue > 14 {
                return false;
            };
        }
    }

    return true;
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Should've been able to read the file");
    let lines = file.lines();
    let mut sum: u64 = 0;
    for line in lines {
        let result = game_parser(line);
        if let Ok((_, game)) = result {
            let minimal = MinimalGame::create(&game);
            println!("minimal: {:?}, power: {}", &minimal, minimal.power());

            sum += minimal.power() as u64;
        }
    }

    println!("sum: {}", sum);
}

#[cfg(test)]
mod tests {

    use nom::Finish;

    use crate::{game_parser, parse_color, parse_colors, parse_samples, Sample};

    #[test]
    fn parse_game() -> Result<(), nom::error::Error<&'static str>> {
        let result = game_parser("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").finish();

        let (input, parsed) = match result {
            Err(err) => return Result::Err(err),
            Ok(v) => (v.0, v.1),
        };

        assert_eq!("", input);
        assert_eq!(1, parsed.id);
        assert_eq!(3, parsed.samples.len());

        Ok(())
    }

    #[test]
    fn test_parse_color() {
        let (input, qtd, color) = match parse_color("3 blue") {
            Ok((input, (qtd, color))) => (input, qtd, color),
            Err(_) => panic!("result failed"),
        };

        assert_eq!("", input);
        assert_eq!(3, qtd);
        assert_eq!("blue", color);
    }

    #[test]
    fn test_parse_colors() {
        let (input, sample) = match parse_colors("3 blue, 1 red, 4 green") {
            Ok((input, sample)) => (input, sample),
            Err(_) => panic!("result failed"),
        };

        assert_eq!("", input);
        assert_eq!(Some(3), sample.blue);
        assert_eq!(Some(4), sample.green);
        assert_eq!(Some(1), sample.red);
    }

    #[test]
    fn test_parse_samples() {
        let (input, sample) = match parse_samples("3 blue, 1 red, 4 green; 4 red") {
            Ok((input, sample)) => (input, sample),
            Err(_) => panic!("result failed"),
        };

        assert_eq!("", input);
        assert_eq!(2, sample.len());
        assert_eq!(
            Sample {
                red: Some(1),
                green: Some(4),
                blue: Some(3)
            },
            sample[0]
        );

        assert_eq!(
            Sample {
                red: Some(4),
                green: None,
                blue: None
            },
            sample[1]
        );
    }
}
