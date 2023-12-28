use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Cell {
    pos_x: i32,
    pos_y: i32,
}

impl Cell {
    fn new(pos_x: i32, pos_y: i32) -> Cell {
        Cell { pos_x, pos_y }
    }
}

struct World {
    cells: HashMap<Cell, bool>,
    iteration: u32,
}

impl World {
    fn new() -> World {
        World {
            cells: HashMap::new(),
            iteration: 0,
        }
    }

    fn init(&mut self, initial_state: Vec<Cell>) -> Result<(), &'static str> {
        if initial_state.len() == 0 {
            return Err("Invalid initial state");
        }
        for cell in initial_state {
            match self.add_cell(cell) {
                Err(e) => {
                    println!("Error: {}", e);
                    return Err("Invalid initial state");
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn add_cell(&mut self, cell: Cell) -> Result<(), &'static str> {
        if self.is_there_live(cell.pos_x, cell.pos_y) {
            return Err("Cell already exists");
        }
        self.cells.insert(cell, true);
        Ok(())
    }

    fn is_there_live(&self, pos_x: i32, pos_y: i32) -> bool {
        self.cells.contains_key(&Cell::new(pos_x, pos_y))
    }

    fn iterate(&mut self) -> Result<(), &'static str> {
        if self.cells.len() == 0 {
            return Err("World not initialized");
        }
        self.iteration += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_pos_x_and_pos_y_i_can_create_a_cell() {
        let cell = Cell::new(1, 2);
        assert_eq!(cell.pos_x, 1);
        assert_eq!(cell.pos_y, 2);
    }

    #[test]
    fn after_create_a_world_initial_state_by_default_is_empty() {
        // TODO add helper fn
        let world = World::new();
        assert_eq!(world.cells.len(), 0);
        assert_eq!(world.iteration, 0);
    }

    #[test]
    fn world_has_to_be_initalized_before() {
        // TODO add helper fn
        let mut world = World::new();
        assert_eq!(world.iterate(), Err("World not initialized"));
    }

    #[test]
    fn when_create_a_world_initial_state_has_to_be_valid() {
        let mut world = World::new();
        let initial_state = vec![Cell::new(2, 2), Cell::new(2, 2)];
        assert_eq!(world.init(initial_state), Err("Invalid initial state"));
    }

    #[test]
    fn after_adding_a_cell_to_world_i_can_query_for_that_position() {
        let mut world = World::new();
        let initial_state = vec![Cell::new(1, 2), Cell::new(2, 2)];
        world.init(initial_state).unwrap();
        assert_eq!(world.cells.len(), 2);
        assert_eq!(world.is_there_live(1, 2), true);
        assert_eq!(world.is_there_live(3, 2), false);
    }

    #[test]
    fn iterate_over_an_empty_world_is_not_allowed() {
        let mut world = World::new();
        assert_eq!(world.iterate(), Err("World not initialized"));
    }

    #[test]
    fn given_an_initialized_world_i_can_create_a_iteration() {
        let mut world = World::new();
        let initial_state = vec![Cell::new(1, 2), Cell::new(2, 2)];
        world.init(initial_state).unwrap();
        assert_eq!(world.iteration, 0);
        world.iterate().unwrap();
        assert_eq!(world.iteration, 1);
    }
}
