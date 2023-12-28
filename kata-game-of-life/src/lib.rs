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
}

impl World {
    fn new() -> World {
        World {
            cells: HashMap::new(),
        }
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
}

#[cfg(test)]
mod tests {
    use std::f32::consts::E;

    use super::*;

    #[test]
    fn given_pos_x_and_pos_y_i_can_create_a_cell() {
        let cell = Cell::new(1, 2);
        assert_eq!(cell.pos_x, 1);
        assert_eq!(cell.pos_y, 2);
    }

    #[test]
    fn when_create_a_world_is_empty() {
        let world = World::new();
        assert_eq!(world.cells.len(), 0);
    }

    #[test]
    fn after_adding_a_cell_to_world_size_increases() {
        let mut world = World::new();
        world.add_cell(Cell::new(1, 2)).unwrap();
        assert_eq!(world.cells.len(), 1);
    }

    #[test]
    fn after_adding_a_cell_to_world_i_can_query_for_that_position() {
        let mut world = World::new();
        world.add_cell(Cell::new(1, 2)).unwrap();
        assert_eq!(world.is_there_live(1, 2), true);
    }

    #[test]
    fn i_cannot_create_a_cell_in_a_position_where_there_is_already_a_cell() {
        let mut world = World::new();
        world.add_cell(Cell::new(1, 2)).unwrap();
        assert_eq!(world.add_cell(Cell::new(1, 2)), Err("Cell already exists"));
    }
}
