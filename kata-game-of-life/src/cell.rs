struct Cell {
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    Alive,
    Dead,
}

impl Cell {
    fn new(status: Status) -> Cell {
        Cell { status }
    }

    fn evolve(&mut self, neighbours: u8) {
        match self.status {
            Status::Alive => {
                if neighbours < 2 || neighbours > 3 {
                    self.status = Status::Dead;
                }
            }
            Status::Dead => {
                if neighbours == 3 {
                    self.status = Status::Alive;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_zero_neighbours_cell_dies() {
        let mut cell = Cell::new(Status::Alive);
        assert_eq!(Status::Alive, cell.status);
        cell.evolve(0);
        assert_eq!(Status::Dead, cell.status);
    }

    #[test]
    fn given_one_neighbours_cell_dies() {
        let mut cell = Cell::new(Status::Alive);
        assert_eq!(Status::Alive, cell.status);
        cell.evolve(1);
        assert_eq!(Status::Dead, cell.status);
    }

    #[test]
    fn given_a_dead_cell_it_remains_dead() {
        let mut cell = Cell::new(Status::Dead);
        assert_eq!(Status::Dead, cell.status);
        cell.evolve(1);
        assert_eq!(Status::Dead, cell.status);
        cell.evolve(2);
        assert_eq!(Status::Dead, cell.status);
        cell.evolve(4);
        assert_eq!(Status::Dead, cell.status);
    }

    #[test]
    fn given_three_neighbours_cell_resurrects() {
        let mut cell = Cell::new(Status::Dead);
        assert_eq!(Status::Dead, cell.status);
        cell.evolve(3);
        assert_eq!(Status::Alive, cell.status);
    }

    #[test]
    fn given_two_neighbours_cell_remains_alive() {
        let mut cell = Cell::new(Status::Alive);
        assert_eq!(Status::Alive, cell.status);
        cell.evolve(2);
        assert_eq!(Status::Alive, cell.status);
    }

    #[test]
    fn given_three_neighbours_cell_remains_alive() {
        let mut cell = Cell::new(Status::Alive);
        assert_eq!(Status::Alive, cell.status);
        cell.evolve(3);
        assert_eq!(Status::Alive, cell.status);
    }

    #[test]
    fn given_more_than_three_neighbours_cell_deads() {
        let mut cell = Cell::new(Status::Alive);
        assert_eq!(Status::Alive, cell.status);
        cell.evolve(4);
        assert_eq!(Status::Dead, cell.status);
    }
}
