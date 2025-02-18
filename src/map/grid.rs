// Copyright 2025 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Floor,
    Wall,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Floor => write!(f, "."),
            Tile::Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    width: usize,
    height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Tile::Wall);
            }
            tiles.push(row);
        }
        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_tile() {
        assert_eq!(".", format!("{}", Tile::Floor));
        assert_eq!("#", format!("{}", Tile::Wall));
    }

    #[test]
    fn test_new_map() {
        let grid = Grid::new(3, 2);
        assert_eq!(3, grid.width());
        assert_eq!(2, grid.height());
        assert_eq!(
            vec![
                vec![Tile::Wall, Tile::Wall, Tile::Wall],
                vec![Tile::Wall, Tile::Wall, Tile::Wall],
            ],
            grid.tiles
        );
    }

    #[test]
    fn test_display_map() {
        let grid = Grid::new(3, 2);
        assert_eq!("###\n###\n", format!("{}", grid));
    }
}
