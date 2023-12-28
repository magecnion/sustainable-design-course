use std::collections::HashMap;

mod cell;

#[derive(PartialEq, Eq, Hash, Debug)]

struct Position {
    pos_x: i32,
    pos_y: i32,
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
        self.number_of_generations += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_empty_initial_state_raises_error() {
        assert_eq!(World::new(HashMap::new()), Err("World cannot be empty"));
    }

    #[test]
    fn given_an_initial_state_i_can_create_a_world() {
        let mut initial_state = HashMap::new();
        initial_state.insert(
            Position { pos_x: 1, pos_y: 2 },
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
            Position { pos_x: 1, pos_y: 2 },
            cell::Cell::new(cell::Status::Alive),
        );
        let mut world = World::new(initial_state).unwrap();
        world.calculate_next_generation().unwrap();
        assert_eq!(world.number_of_generations, 1)
    }
}
