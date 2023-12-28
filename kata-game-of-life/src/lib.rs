use std::collections::HashMap;

mod cell;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn get_right(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn get_right_top(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    fn get_right_bottom(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    fn get_left(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn get_left_top(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn get_left_bottom(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    fn get_top(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn get_bottom(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }
}

#[derive(Debug, PartialEq)]
struct World {
    cells: HashMap<Position, cell::Cell>,
    generation_count: u32,
}

impl World {
    fn new(initial_state: HashMap<Position, cell::Cell>) -> Result<World, &'static str> {
        if initial_state.len() == 0 {
            return Err("World cannot be empty");
        }
        let mut cells = HashMap::new();
        for (position, cell) in initial_state {
            cells.insert(position, cell);
        }
        Ok(World {
            cells,
            generation_count: 0,
        })
    }

    fn calculate_next_generation(&self) -> Result<World, &'static str> {
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
        let mut alive_neightbours: u8 = 0;
        if let Some(cell) = self.cells.get(&position.get_right()) {
            if cell.status == cell::Status::Alive {
                alive_neightbours += 1;
            }
        }
        if let Some(cell) = self.cells.get(&position.get_right_top()) {
            if cell.status == cell::Status::Alive {
                alive_neightbours += 1;
            }
        }
        if let Some(cell) = self.cells.get(&position.get_right_bottom()) {
            if cell.status == cell::Status::Alive {
                alive_neightbours += 1;
            }
        }
        if let Some(cell) = self.cells.get(&position.get_left()) {
            if cell.status == cell::Status::Alive {
                alive_neightbours += 1;
            }
        }
        if let Some(cell) = self.cells.get(&position.get_left_top()) {
            if cell.status == cell::Status::Alive {
                alive_neightbours += 1;
            }
        }
        if let Some(cell) = self.cells.get(&position.get_left_bottom()) {
            if cell.status == cell::Status::Alive {
                alive_neightbours += 1;
            }
        }
        if let Some(cell) = self.cells.get(&position.get_top()) {
            if cell.status == cell::Status::Alive {
                alive_neightbours += 1;
            }
        }
        if let Some(cell) = self.cells.get(&position.get_bottom()) {
            if cell.status == cell::Status::Alive {
                alive_neightbours += 1;
            }
        }
        alive_neightbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const D: cell::Status = cell::Status::Dead;
    const A: cell::Status = cell::Status::Alive;

    fn create_initial_state_from_matrix(
        state: Vec<Vec<cell::Status>>,
    ) -> HashMap<Position, cell::Cell> {
        let mut initial_state = HashMap::new();
        for (x, row) in state.iter().enumerate() {
            for (y, cell_status) in row.iter().enumerate() {
                initial_state.insert(
                    Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    cell::Cell::new(*cell_status),
                );
            }
        }
        initial_state
    }

    #[test]
    fn given_a_position_get_right() {
        let position = Position { x: 1, y: 2 };
        assert_eq!(position.get_right(), Position { x: 2, y: 2 });
    }

    #[test]
    fn given_a_position_get_left() {
        let position = Position { x: 1, y: 2 };
        assert_eq!(position.get_left(), Position { x: 0, y: 2 });
    }

    #[test]
    fn given_a_position_get_top() {
        let position = Position { x: 1, y: 2 };
        assert_eq!(position.get_top(), Position { x: 1, y: 3 });
    }

    #[test]
    fn given_a_position_get_bottom() {
        let position = Position { x: 1, y: 2 };
        assert_eq!(position.get_bottom(), Position { x: 1, y: 1 });
    }

    #[test]
    fn given_empty_initial_state_raises_error() {
        assert_eq!(World::new(HashMap::new()), Err("World cannot be empty"));
    }

    #[test]
    fn given_an_initial_state_i_can_create_a_world() {
        let mut initial_state = HashMap::new();
        initial_state.insert(
            Position { x: 1, y: 2 },
            cell::Cell::new(cell::Status::Alive),
        );
        let world = World::new(initial_state).unwrap();
        assert_eq!(world.cells.len(), 1);
    }

    #[test]
    fn given_a_world_i_can_calculate_next_generation() {
        let mut initial_state = HashMap::new();
        initial_state.insert(
            Position { x: 1, y: 2 },
            cell::Cell::new(cell::Status::Alive),
        );
        let world = World::new(initial_state).unwrap();
        let world = world.calculate_next_generation().unwrap();
        assert_eq!(world.generation_count, 1);
    }

    #[test]
    fn given_a_world_i_can_calculate_alive_neighbours_for_a_dead_cell() {
        let initial_state = create_initial_state_from_matrix(vec![vec![A, A]]);

        let mut world = World::new(initial_state).unwrap();
        world
            .cells
            .insert(Position { x: 0, y: 0 }, cell::Cell::new(cell::Status::Dead));
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 0, y: 0 }),
            1
        );
    }

    #[test]
    fn given_a_1d_world_i_can_calculate_alive_neighbours() {
        let initial_state = create_initial_state_from_matrix(vec![vec![A, A, A]]);
        let mut world = World::new(initial_state).unwrap();
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 0, y: 0 }),
            1
        );
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 0, y: 1 }),
            2
        );

        // after killing one neighbour the number of alive neighbours is 1 again
        world
            .cells
            .insert(Position { x: 0, y: 0 }, cell::Cell::new(cell::Status::Dead));
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 0, y: 1 }),
            1
        );
    }

    #[test]
    fn given_a_2d_world_i_can_calculate_alive_neighbours() {
        // A A A
        // A A A
        // A A A
        let initial_state =
            create_initial_state_from_matrix(vec![vec![A, A, A], vec![A, A, A], vec![A, A, A]]);
        let world = World::new(initial_state).unwrap();
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 1, y: 1 }),
            8
        );
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 2, y: 2 }),
            3
        );
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 0, y: 1 }),
            5
        );

        // A A A
        // D A D
        // A A A
        let initial_state =
            create_initial_state_from_matrix(vec![vec![A, A, A], vec![D, A, D], vec![A, A, A]]);
        let world = World::new(initial_state).unwrap();
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 1, y: 1 }),
            6
        );
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 2, y: 2 }),
            2
        );
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 0, y: 1 }),
            3
        );
    }

    #[test]
    fn given_a_2d_world_with_empty_spots_i_can_calculate_alive_neighbours() {
        // A A A
        // A A X
        // A A X
        let initial_state =
            create_initial_state_from_matrix(vec![vec![A, A, A], vec![A, A, A], vec![A, A, A]]);
        let mut world = World::new(initial_state).unwrap();
        world.cells.remove(&Position { x: 2, y: 2 });
        world.cells.remove(&Position { x: 1, y: 2 });
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 1, y: 1 }),
            6
        );
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 2, y: 2 }),
            0
        );
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 0, y: 1 }),
            4
        );

        // after killing two neighbours the number of alive neighbours change
        world
            .cells
            .insert(Position { x: 1, y: 0 }, cell::Cell::new(cell::Status::Dead));
        // A A A
        // D A X
        // A A X
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 1, y: 1 }),
            5
        );
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 2, y: 2 }),
            0
        );
        assert_eq!(
            world.calculate_alive_neighbours(&Position { x: 0, y: 1 }),
            3
        );
    }

    #[test]
    fn generates_the_next_state_of_the_world() {
        // D A D
        // D A D
        // D A D
        let initial_state =
            create_initial_state_from_matrix(vec![vec![D, A, D], vec![D, A, D], vec![D, A, D]]);
        let world = World::new(initial_state).unwrap();

        // D D D
        // A A A
        // D D D
        let initial_state =
            create_initial_state_from_matrix(vec![vec![D, D, D], vec![A, A, A], vec![D, D, D]]);
        let new_world = World::new(initial_state).unwrap();
        assert_eq!(
            world.calculate_next_generation().unwrap().cells,
            new_world.cells
        );
    }

    #[test]
    fn never_changes_for_a_given_initial_block_pattern() {
        let initial_state = create_initial_state_from_matrix(vec![
            vec![A, A, D, D, D],
            vec![A, A, D, D, D],
            vec![D, D, D, D, D],
            vec![D, D, D, D, D],
            vec![D, D, D, D, D],
        ]);
        let initial_world = World::new(initial_state).unwrap();
        let current_world = initial_world
            .calculate_next_generation()
            .unwrap()
            .calculate_next_generation()
            .unwrap()
            .calculate_next_generation()
            .unwrap();
        assert_eq!(initial_world.cells, current_world.cells);
        assert_eq!(current_world.generation_count, 3);
    }

    #[test]
    fn reestablishes_the_same_state_after_two_generations_when_a_given_oscillator_pattern_is_provided(
    ) {
        let initial_state = create_initial_state_from_matrix(vec![
            vec![D, D, D, D, D],
            vec![D, D, A, D, D],
            vec![D, D, A, D, D],
            vec![D, D, A, D, D],
            vec![D, D, D, D, D],
        ]);
        let initial_world = World::new(initial_state).unwrap();
        let current_world = initial_world
            .calculate_next_generation()
            .unwrap()
            .calculate_next_generation()
            .unwrap();
        assert_eq!(initial_world.cells, current_world.cells);
        assert_eq!(current_world.generation_count, 2);
    }
}
