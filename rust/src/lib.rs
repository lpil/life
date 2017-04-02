use std::collections::HashSet;
use std::collections::HashMap;


pub type Cell = (isize, isize);


#[derive(Debug, PartialEq, Default)]
pub struct Game {
    grid: HashSet<Cell>,
}


impl Game {
    /// Create a new Game with no alive cells
    ///
    pub fn new() -> Self {
        Default::default()
    }


    /// Insert an alive cell at a position
    ///
    pub fn insert_cell(&mut self, x: isize, y: isize) {
        self.grid.insert((x, y));
    }


    /// Advance the game by one round.
    ///
    /// - Any live cell with fewer than two live neighbours
    ///   dies, as if caused by underpopulation.
    /// - Any live cell with two or three live neighbours
    ///   lives on to the next generation.
    /// - Any live cell with more than three live neighbours
    ///   dies, as if by overpopulation.
    /// - Any dead cell with exactly three live neighbours
    ///   becomes a live cell, as if by reproduction.
    ///
    pub fn tick(&self) -> Self {
        let mut dead_counters = HashMap::new();
        let mut next = Self::new();

        for &(x, y) in self.grid.iter() {
            self.tick_for_alive_cell(&mut next, &mut dead_counters, x, y)
        }
        for (&(x, y), count) in dead_counters.iter() {
            if *count == 3 {
                next.insert_cell(x, y)
            }
        }
        next
    }

    fn tick_for_alive_cell(&self,
                           next: &mut Self,
                           dead_counters: &mut HashMap<Cell, u8>,
                           x: isize,
                           y: isize) {
        let mut neighbours = 0;

        for &pos in neighbour_positions(x, y).iter() {
            if self.grid.contains(&pos) {
                neighbours += 1
            } else {
                // A dead cell has a neighbour.
                inc_counter(dead_counters, pos);
            }
        }
        if neighbours == 2 || neighbours == 3 {
            next.insert_cell(x, y);
        }

    }
}


fn neighbour_positions(x: isize, y: isize) -> [(isize, isize); 8] {
    [// Top
     (x - 1, y - 1),
     (x, y - 1),
     (x + 1, y - 1),
     // Mid
     (x - 1, y),
     (x + 1, y),
     // Bottom
     (x - 1, y + 1),
     (x, y + 1),
     (x + 1, y + 1)]
}


fn inc_counter(map: &mut HashMap<Cell, u8>, cell: Cell) {
    let count = map.entry(cell).or_insert(0);
    *count += 1;
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn game_new() {
        let game = Game::new();
        assert_eq!(game.grid, HashSet::new())
    }


    #[test]
    fn game_inserting_cells() {
        let mut game = Game::new();
        assert_eq!(alive_cells(&game), vec![]);
        game.insert_cell(3, 0);
        assert_eq!(alive_cells(&game), vec![(3, 0)]);
        game.insert_cell(0, 0);
        assert_eq!(alive_cells(&game), vec![(0, 0), (3, 0)]);
        game.insert_cell(3, 3);
        assert_eq!(alive_cells(&game), vec![(0, 0), (3, 0), (3, 3)]);
    }


    #[test]
    fn game_tick_0() {
        let game = Game::new();
        let next = game.tick();
        assert_eq!(game, next);
    }


    #[test]
    fn game_tick_0_neighbours() {
        //
        // x__
        // ___
        // ___
        //
        let mut game = Game::new();
        game.insert_cell(0, 0);
        let next = game.tick();
        assert_eq!(alive_cells(&next), vec![]);
    }


    #[test]
    fn game_tick_1() {
        //
        // xx_
        // ___
        // ___
        //
        let mut game = Game::new();
        game.insert_cell(0, 0);
        game.insert_cell(1, 0);
        let next = game.tick();
        assert_eq!(alive_cells(&next), vec![]);
    }


    #[test]
    fn game_tick_2() {
        //
        // ___
        // xxx
        // ___
        //
        let mut game = Game::new();
        game.insert_cell(0, 1);
        game.insert_cell(1, 1);
        game.insert_cell(2, 1);
        let next = game.tick();
        assert_eq!(alive_cells(&next), vec![(1, 0), (1, 1), (1, 2)]);
    }

    #[test]
    fn game_tick_3() {
        //
        // x__
        // xx_
        // ___
        //
        let mut game = Game::new();
        game.insert_cell(0, 0);
        game.insert_cell(0, 1);
        game.insert_cell(1, 1);
        let next = game.tick();
        assert_eq!(alive_cells(&next), vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
    }


    fn alive_cells(game: &Game) -> Vec<Cell> {
        let mut cells = game.grid.iter().map(|c| c.clone()).collect::<Vec<Cell>>();
        cells.sort();
        cells
    }
}
