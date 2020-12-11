use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use anyhow::{Context, Error};

/// Day 3a: Toboggan Trajectory (part 1)
///
/// # Description
///
/// With the toboggan login problems resolved, you set off toward the airport.
/// While travel by toboggan might be easy, it's certainly not safe: there's very
/// minimal steering and the area is covered in trees. You'll need to see which
/// angles will take you near the fewest trees.
///
/// Due to the local geology, trees in this area only grow on exact integer
/// coordinates in a grid. You make a map (your puzzle input) of the open
/// squares (`.`) and trees (`#`) you can see. For example:
///
/// ```text
/// ..##.......
/// #...#...#..
/// .#....#..#.
/// ..#.#...#.#
/// .#...##..#.
/// ..#.##.....
/// .#.#.#....#
/// .#........#
/// #.##...#...
/// #...##....#
/// .#..#...#.#
/// ```
///
/// These aren't the only trees, though; due to something you read about once
/// involving arboreal genetics and biome stability, the same pattern repeats to
/// the right many times:
///
/// ```text
/// ..##.........##.........##.........##.........##.........##.......  --->
/// #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
/// .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
/// ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
/// .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
/// ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
/// .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
/// .#........#.#........#.#........#.#........#.#........#.#........#
/// #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
/// #...##....##...##....##...##....##...##....##...##....##...##....#
/// .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
/// ```
///
/// You start on the open square (`.`) in the top-left corner and need to reach
/// the bottom (below the bottom-most row on your map).
///
/// The toboggan can only follow a few specific slopes (you opted for a cheaper
/// model that prefers rational numbers); start by counting all the trees you
/// would encounter for the slope right 3, down 1:
///
/// From your starting position at the top-left, check the position that is right
/// 3 and down 1. Then, check the position that is right 3 and down 1 from there,
/// and so on until you go past the bottom of the map.
///
/// The locations you'd check in the above example are marked here with O where
/// there was an open square and X where there was a tree:
///
/// ```text
/// ..##.........##.........##.........##.........##.........##.......  --->
/// #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
/// .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
/// ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
/// .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
/// ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
/// .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
/// .#........#.#........X.#........#.#........#.#........#.#........#
/// #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
/// #...##....##...##....##...#X....##...##....##...##....##...##....#
/// .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
/// ```
///
/// In this example, traversing the map using this slope would cause you to
/// encounter 7 trees.
///
/// Starting at the top-left corner of your map and following a slope of right 3
/// and down 1, how many trees would you encounter?
#[aoc_macros::challenge]
pub fn part_1(board: Board) -> Result<usize, Error> {
    Ok(trees_along_slope(&board, 3, 1))
}

/// Day 3b: Toboggan Trajectory (part 2)
///
/// # Description
///
/// Time to check the rest of the slopes - you need to minimize the probability
/// of a sudden arboreal stop, after all.
///
/// Determine the number of trees you would encounter if, for each of the
/// following slopes, you start at the top-left corner and traverse the map all
/// the way to the bottom:
///
/// ```text
/// Right 1, down 1.
/// Right 3, down 1. (This is the slope you already checked.)
/// Right 5, down 1.
/// Right 7, down 1.
/// Right 1, down 2.
/// ```
///
/// In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s)
/// respectively; multiplied together, these produce the answer 336.
///
/// What do you get if you multiply together the number of trees encountered on
/// each of the listed slopes?
#[aoc_macros::challenge]
pub fn part_2(board: Board) -> Result<usize, Error> {
    let combinations = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    Ok(combinations
        .iter()
        .copied()
        .map(|(horizontal, vertical)| {
            trees_along_slope(&board, horizontal, vertical)
        })
        .product())
}

fn trees_along_slope(
    board: &Board,
    horizontal_delta: usize,
    vertical_delta: usize,
) -> usize {
    let mut row = 0;
    let mut column = 0;
    let mut trees = 0;

    while row < board.height {
        let tile = board.tile_at(column, row);

        if tile == Tile::Tree {
            trees += 1;
        }

        row += vertical_delta;
        column += horizontal_delta;
    }

    trees
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Board {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Self {
        assert_eq!(width * height, tiles.len());

        Board {
            width,
            height,
            tiles,
        }
    }

    pub fn tile_at(&self, column: usize, row: usize) -> Tile {
        let ix = self.index(column % self.width, row);
        self.tiles[ix]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[Tile]> + '_ {
        let Board {
            ref tiles,
            width,
            height,
        } = *self;

        (0..height)
            .map(move |row| row * width)
            .map(move |first_index| &tiles[first_index..first_index + width])
    }

    fn index(&self, column: usize, row: usize) -> usize {
        column + row * self.width
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            for tile in row {
                match tile {
                    Tile::Tree => write!(f, "#")?,
                    Tile::Open => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().filter(|l| !l.is_empty());

        let mut tiles = Vec::new();

        // we parse the first line to get the width
        let first_line = lines.next().context("The board can't be empty")?;
        append_tiles(&mut tiles, first_line)
            .context("Unable to read line 1")?;

        let width = tiles.len();
        let mut height = 1;

        for line in lines {
            height += 1;
            let current_length = tiles.len();

            append_tiles(&mut tiles, line)
                .with_context(|| format!("Unable to read line {}", height))?;

            let items_added = tiles.len() - current_length;
            if items_added != width {
                anyhow::bail!("The board should be {} items wide but line {} had {} items", width, height, items_added);
            }
        }

        Ok(Board {
            tiles,
            width,
            height,
        })
    }
}

fn append_tiles(dest: &mut Vec<Tile>, line: &str) -> Result<(), Error> {
    for letter in line.trim().chars() {
        match letter {
            '#' => dest.push(Tile::Tree),
            '.' => dest.push(Tile::Open),
            other => anyhow::bail!(
                "The board can only contain \"#\" or \".\", found \"{}\"",
                other
            ),
        }
    }

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Open,
    Tree,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_BOARD: &str = r#"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    #[test]
    fn parse_a_game_board() {
        let raw = EXAMPLE_BOARD;

        let got: Board = raw.parse().unwrap();

        assert_eq!(got.width, 11);
        assert_eq!(got.height, 11);
        assert_eq!(got.height, got.rows().count());
        let second_row_should_be = &[
            Tile::Tree,
            Tile::Open,
            Tile::Open,
            Tile::Open,
            Tile::Tree,
            Tile::Open,
            Tile::Open,
            Tile::Open,
            Tile::Tree,
            Tile::Open,
            Tile::Open,
        ];
        assert_eq!(got.rows().nth(1).unwrap(), second_row_should_be);
    }

    #[test]
    fn wrap_horizontally() {
        let board = Board::from_str(EXAMPLE_BOARD).unwrap();
        let second_row = &[
            Tile::Tree,
            Tile::Open,
            Tile::Open,
            Tile::Open,
            Tile::Tree,
            Tile::Open,
            Tile::Open,
            Tile::Open,
            Tile::Tree,
            Tile::Open,
            Tile::Open,
        ];
        let row = 1;

        // iterate through in the normal range
        for column in 0..board.width {
            let got = board.tile_at(column, row);
            assert_eq!(got, second_row[column]);
        }

        // and then wrap around to the right
        for column in board.width..2 * board.width {
            let got = board.tile_at(column, row);
            assert_eq!(got, second_row[column - board.width]);
        }
    }
}
