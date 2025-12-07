pub const MAX_ADJACENT_ROLLS: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Empty,
    Roll,
}

impl Cell {
    pub fn rolls(adjacent: [&Self; 8]) -> usize {
        adjacent.iter().filter(|cell| cell == &&&Self::Roll).count()
    }
}

#[derive(Clone)]
pub struct Grid(pub Vec<Vec<Cell>>);

impl FromIterator<Vec<Cell>> for Grid {
    fn from_iter<T: IntoIterator<Item = Vec<Cell>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Grid {
    #[rustfmt::skip]
    const ADJACENT_OFFSETS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.0.get(y).and_then(|row| row.get(x))
    }

    pub fn adjacent(&self, x: usize, y: usize) -> [&Cell; 8] {
        std::array::from_fn(|index| {
            let (change_x, change_y) = Self::ADJACENT_OFFSETS[index];
            if let (Some(x), Some(y)) = (
                x.checked_add_signed(change_y),
                y.checked_add_signed(change_x),
            ) {
                self.get(x, y).unwrap_or(&Cell::Wall)
            } else {
                &Cell::Wall
            }
        })
    }
}

pub fn parse_input(input: &'static str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|symbol| match symbol {
                    b'@' => Cell::Roll,
                    b'.' => Cell::Empty,
                    other => panic!("unsupported character {other}"),
                })
                .collect::<Vec<Cell>>()
        })
        .collect()
}
