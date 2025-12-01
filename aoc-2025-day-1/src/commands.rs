use std::str::FromStr;

#[derive(Debug)]
enum ParseError {
    MeNoLikey,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Command {
    direction: Direction,
    distance: u32,
}

impl Command {
    const fn new(direction: Direction, distance: u32) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_at(1);
        let direction = match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(ParseError::MeNoLikey),
        };
        let distance = u32::from_str(distance).map_err(|_| ParseError::MeNoLikey)?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left() {
        assert_eq!(
            Command::new(Direction::Left, 0),
            Command::from_str("L0").unwrap()
        );
        assert_eq!(
            Command::new(Direction::Left, 100),
            Command::from_str("L100").unwrap()
        );
        assert_eq!(
            Command::new(Direction::Left, 10000),
            Command::from_str("L10000").unwrap()
        );
    }
    #[test]
    fn right() {
        assert_eq!(
            Command::new(Direction::Right, 0),
            Command::from_str("R0").unwrap()
        );
        assert_eq!(
            Command::new(Direction::Right, 100),
            Command::from_str("R100").unwrap()
        );
        assert_eq!(
            Command::new(Direction::Right, 10000),
            Command::from_str("R10000").unwrap()
        );
    }
}
