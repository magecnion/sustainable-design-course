#[derive(Debug, PartialEq)]
pub struct Cell {
    pub status: Status,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Status {
    Alive,
    Dead,
}

impl Cell {
    pub fn new(status: Status) -> Cell {
        Cell { status }
    }

    pub fn evolve(&self, neighbours: u8) -> Cell {
        match self.status {
            Status::Alive => {
                if neighbours < 2 || neighbours > 3 {
                    return Cell::new(Status::Dead);
                }
                return Cell::new(Status::Alive);
            }
            Status::Dead => {
                if neighbours == 3 {
                    return Cell::new(Status::Alive);
                }
                return Cell::new(Status::Dead);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_zero_neighbours_cell_dies() {
        let cell = Cell::new(Status::Alive);
        assert_eq!(Status::Alive, cell.status);
        let cell = cell.evolve(0);
        assert_eq!(Status::Dead, cell.status);
    }

    #[test]
    fn given_one_neighbours_cell_dies() {
        let cell = Cell::new(Status::Alive);
        assert_eq!(Status::Alive, cell.status);
        let cell = cell.evolve(1);
        assert_eq!(Status::Dead, cell.status);
    }

    #[test]
    fn given_a_dead_cell_it_remains_dead() {
        let cell = Cell::new(Status::Dead);
        assert_eq!(Status::Dead, cell.status);
        let cell = cell.evolve(1);
        assert_eq!(Status::Dead, cell.status);
        let cell = cell.evolve(2);
        assert_eq!(Status::Dead, cell.status);
        let cell = cell.evolve(4);
        assert_eq!(Status::Dead, cell.status);
    }

    #[test]
    fn given_three_neighbours_cell_resurrects() {
        let cell = Cell::new(Status::Dead);
        assert_eq!(Status::Dead, cell.status);
        let cell = cell.evolve(3);
        assert_eq!(Status::Alive, cell.status);
    }

    #[test]
    fn given_two_neighbours_cell_remains_alive() {
        let cell = Cell::new(Status::Alive);
        assert_eq!(Status::Alive, cell.status);
        let cell = cell.evolve(2);
        assert_eq!(Status::Alive, cell.status);
    }

    #[test]
    fn given_three_neighbours_cell_remains_alive() {
        let cell = Cell::new(Status::Alive);
        assert_eq!(Status::Alive, cell.status);
        let cell = cell.evolve(3);
        assert_eq!(Status::Alive, cell.status);
    }

    #[test]
    fn given_more_than_three_neighbours_cell_deads() {
        let cell = Cell::new(Status::Alive);
        assert_eq!(Status::Alive, cell.status);
        let cell = cell.evolve(4);
        assert_eq!(Status::Dead, cell.status);
    }
}
