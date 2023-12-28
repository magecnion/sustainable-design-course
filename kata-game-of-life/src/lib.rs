use std::collections::HashMap;

mod cell;

#[derive(PartialEq, Eq, Hash, Debug)]

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
    number_of_generations: u32,
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
            number_of_generations: 0,
        })
    }

    fn calculate_next_generation(&mut self) -> Result<(), &'static str> {
        // for
        self.number_of_generations += 1;
        Ok(())
    }

    fn calculate_alive_neighbours(&self, position: Position) -> u8 {
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
        assert_eq!(world.number_of_generations, 0)
    }

    #[test]
    fn given_a_world_i_can_calculate_next_generation() {
        let mut initial_state = HashMap::new();
        initial_state.insert(
            Position { x: 1, y: 2 },
            cell::Cell::new(cell::Status::Alive),
        );
        let mut world = World::new(initial_state).unwrap();
        world.calculate_next_generation().unwrap();
        assert_eq!(world.number_of_generations, 1)
    }

    fn create_inital_state(state: Vec<(i32, i32)>) -> HashMap<Position, cell::Cell> {
        let mut initial_state = HashMap::new();
        for (x, y) in state {
            initial_state.insert(
                Position { x: x, y: y },
                cell::Cell::new(cell::Status::Alive),
            );
        }
        initial_state
    }

    #[test]
    fn given_a_world_i_can_calculate_alive_neighbours_for_a_dead_cell() {
        let initial_state = create_inital_state(vec![(0, 0), (0, 1)]);
        let mut world = World::new(initial_state).unwrap();
        world
            .cells
            .insert(Position { x: 0, y: 0 }, cell::Cell::new(cell::Status::Dead));
        assert_eq!(world.calculate_alive_neighbours(Position { x: 0, y: 0 }), 1);
    }

    #[test]
    fn given_a_1d_world_i_can_calculate_alive_neighbours() {
        let initial_state = create_inital_state(vec![(0, 0), (0, 1), (0, 2)]);
        let mut world = World::new(initial_state).unwrap();
        assert_eq!(world.calculate_alive_neighbours(Position { x: 0, y: 0 }), 1);
        assert_eq!(world.calculate_alive_neighbours(Position { x: 0, y: 1 }), 2);

        // after killing one neighbour the number of alive neighbours is 1 again
        world
            .cells
            .insert(Position { x: 0, y: 0 }, cell::Cell::new(cell::Status::Dead));
        assert_eq!(world.calculate_alive_neighbours(Position { x: 0, y: 1 }), 1);
    }

    #[test]
    fn given_a_2d_world_i_can_calculate_alive_neighbours() {
        // A A A
        // A A A
        // A A A
        let initial_state = create_inital_state(vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ]);
        let mut world = World::new(initial_state).unwrap();
        assert_eq!(world.calculate_alive_neighbours(Position { x: 1, y: 1 }), 8);
        assert_eq!(world.calculate_alive_neighbours(Position { x: 2, y: 2 }), 3);
        assert_eq!(world.calculate_alive_neighbours(Position { x: 0, y: 1 }), 5);

        // after killing two neighbours the number of alive neighbours change
        world
            .cells
            .insert(Position { x: 1, y: 0 }, cell::Cell::new(cell::Status::Dead));
        world
            .cells
            .insert(Position { x: 1, y: 2 }, cell::Cell::new(cell::Status::Dead));
        // A A A
        // D A D
        // A A A
        assert_eq!(world.calculate_alive_neighbours(Position { x: 1, y: 1 }), 6);
        assert_eq!(world.calculate_alive_neighbours(Position { x: 2, y: 2 }), 2);
        assert_eq!(world.calculate_alive_neighbours(Position { x: 0, y: 1 }), 3);
    }

    #[test]
    fn given_a_2d_world_with_empty_spots_i_can_calculate_alive_neighbours() {
        // A A A
        // A A X
        // A A X
        let initial_state =
            create_inital_state(vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (2, 0), (2, 1)]);
        let mut world = World::new(initial_state).unwrap();
        assert_eq!(world.calculate_alive_neighbours(Position { x: 1, y: 1 }), 6);
        assert_eq!(world.calculate_alive_neighbours(Position { x: 2, y: 2 }), 0);
        assert_eq!(world.calculate_alive_neighbours(Position { x: 0, y: 1 }), 4);

        // after killing two neighbours the number of alive neighbours change
        world
            .cells
            .insert(Position { x: 1, y: 0 }, cell::Cell::new(cell::Status::Dead));
        // A A A
        // D A X
        // A A X
        assert_eq!(world.calculate_alive_neighbours(Position { x: 1, y: 1 }), 5);
        assert_eq!(world.calculate_alive_neighbours(Position { x: 2, y: 2 }), 0);
        assert_eq!(world.calculate_alive_neighbours(Position { x: 0, y: 1 }), 3);
    }
}
