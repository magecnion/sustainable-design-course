use crate::cell;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]

pub struct Position(pub i32, pub i32);

impl Position {
    fn right(&self) -> Position {
        Position(self.0 + 1, self.1)
    }

    fn right_top(&self) -> Position {
        Position(self.0 + 1, self.1 + 1)
    }

    fn right_bottom(&self) -> Position {
        Position(self.0 + 1, self.1 - 1)
    }

    fn left(&self) -> Position {
        Position(self.0 - 1, self.1)
    }

    fn left_top(&self) -> Position {
        Position(self.0 - 1, self.1 + 1)
    }

    fn left_bottom(&self) -> Position {
        Position(self.0 - 1, self.1 - 1)
    }

    fn top(&self) -> Position {
        Position(self.0, self.1 + 1)
    }

    fn bottom(&self) -> Position {
        Position(self.0, self.1 - 1)
    }
}

#[derive(Debug)]
pub struct World {
    cells: HashMap<Position, cell::Cell>,
    pub generation_count: u32,
}

impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
}

impl World {
    pub fn new(initial_state: Vec<Vec<cell::Status>>) -> Result<World, &'static str> {
        if initial_state.len() == 0 {
            return Err("World cannot be empty");
        }

        let mut cells = HashMap::new();
        for (x, row) in initial_state.iter().enumerate() {
            for (y, cell_status) in row.iter().enumerate() {
                cells.insert(Position(x as i32, y as i32), cell::Cell::new(*cell_status));
            }
        }

        Ok(World {
            cells,
            generation_count: 0,
        })
    }

    pub fn calculate_next_generation(&self) -> Result<World, &'static str> {
        let mut next_generation = HashMap::new();
        for (position, cell) in &self.cells {
            let alive_neighbours = self.calculate_alive_neighbours(&position);
            let cell = cell.evolve(alive_neighbours);
            next_generation.insert(*position, cell);
        }

        Ok(World {
            cells: next_generation,
            generation_count: self.generation_count + 1,
        })
    }

    fn calculate_alive_neighbours(&self, position: &Position) -> u8 {
        if self.cells.get(&position).is_none() {
            return 0;
        }

        let positions = [
            position.right(),
            position.right_top(),
            position.right_bottom(),
            position.left(),
            position.left_top(),
            position.left_bottom(),
            position.top(),
            position.bottom(),
        ];

        let alive_neighbours = positions
            .iter()
            .filter_map(|pos| self.cells.get(pos))
            .filter(|cell| cell.status == cell::Status::Alive)
            .count() as u8;

        alive_neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const D: cell::Status = cell::Status::Dead;
    const A: cell::Status = cell::Status::Alive;

    #[test]
    fn given_a_position_right() {
        let position = Position(1, 2);
        assert_eq!(position.right(), Position(2, 2));
    }

    #[test]
    fn given_a_position_left() {
        let position = Position(1, 2);
        assert_eq!(position.left(), Position(0, 2));
    }

    #[test]
    fn given_a_position_top() {
        let position = Position(1, 2);
        assert_eq!(position.top(), Position(1, 3));
    }

    #[test]
    fn given_a_position_bottom() {
        let position = Position(1, 2);
        assert_eq!(position.bottom(), Position(1, 1));
    }

    #[test]
    fn given_empty_initial_state_raises_error() {
        assert_eq!(World::new(Vec::new()), Err("World cannot be empty"));
    }

    #[test]
    fn given_an_initial_state_i_can_create_a_world() {
        let world = World::new(vec![vec![A]]).unwrap();
        assert_eq!(world.cells.len(), 1);
    }

    #[test]
    fn given_a_world_i_can_calculate_next_generation() {
        let world = World::new(vec![vec![A]])
            .unwrap()
            .calculate_next_generation()
            .unwrap();
        assert_eq!(world.generation_count, 1);
    }

    #[test]
    fn given_a_world_i_can_calculate_alive_neighbours_for_a_dead_cell() {
        let mut world = World::new(vec![vec![A, A]]).unwrap();
        world
            .cells
            .insert(Position(0, 0), cell::Cell::new(cell::Status::Dead));
        assert_eq!(world.calculate_alive_neighbours(&Position(0, 0)), 1);
    }

    #[test]
    fn given_a_1d_world_i_can_calculate_alive_neighbours() {
        let mut world = World::new(vec![vec![A, A, A]]).unwrap();
        assert_eq!(world.calculate_alive_neighbours(&Position(0, 0)), 1);
        assert_eq!(world.calculate_alive_neighbours(&Position(0, 1)), 2);

        // after killing one neighbour the number of alive neighbours is 1 again
        world
            .cells
            .insert(Position(0, 0), cell::Cell::new(cell::Status::Dead));
        assert_eq!(world.calculate_alive_neighbours(&Position(0, 1)), 1);
    }

    #[test]
    fn given_a_2d_world_i_can_calculate_alive_neighbours() {
        // A A A
        // A A A
        // A A A
        let world = World::new(vec![vec![A, A, A], vec![A, A, A], vec![A, A, A]]).unwrap();
        assert_eq!(world.calculate_alive_neighbours(&Position(1, 1)), 8);
        assert_eq!(world.calculate_alive_neighbours(&Position(2, 2)), 3);
        assert_eq!(world.calculate_alive_neighbours(&Position(0, 1)), 5);

        // A A A
        // D A D
        // A A A
        let world = World::new(vec![vec![A, A, A], vec![D, A, D], vec![A, A, A]]).unwrap();
        assert_eq!(world.calculate_alive_neighbours(&Position(1, 1)), 6);
        assert_eq!(world.calculate_alive_neighbours(&Position(2, 2)), 2);
        assert_eq!(world.calculate_alive_neighbours(&Position(0, 1)), 3);
    }

    #[test]
    fn given_a_2d_world_with_empty_spots_i_can_calculate_alive_neighbours() {
        // A A A
        // A A X
        // A A X
        let mut world = World::new(vec![vec![A, A, A], vec![A, A, A], vec![A, A, A]]).unwrap();
        world.cells.remove(&Position(2, 2));
        world.cells.remove(&Position(1, 2));
        assert_eq!(world.calculate_alive_neighbours(&Position(1, 1)), 6);
        assert_eq!(world.calculate_alive_neighbours(&Position(2, 2)), 0);
        assert_eq!(world.calculate_alive_neighbours(&Position(0, 1)), 4);

        // after killing two neighbours the number of alive neighbours change
        world
            .cells
            .insert(Position(1, 0), cell::Cell::new(cell::Status::Dead));
        // A A A
        // D A X
        // A A X
        assert_eq!(world.calculate_alive_neighbours(&Position(1, 1)), 5);
        assert_eq!(world.calculate_alive_neighbours(&Position(2, 2)), 0);
        assert_eq!(world.calculate_alive_neighbours(&Position(0, 1)), 3);
    }
}
