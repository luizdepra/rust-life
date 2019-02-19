/// Represents the state of a Cell.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum State {
    Alive,
    Dead,
}

/// Represents a cell.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    state: State,
}

impl Cell {
    /// Returns if the Cell is alive.
    pub fn is_alive(&self) -> bool {
        self.state == State::Alive
    }

    pub fn spawn(&mut self) {
        self.state = State::Alive;
    }

    pub fn kill(&mut self) {
        self.state = State::Dead;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell { state: State::Dead }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_default() {
        let expected = Cell { state: State::Dead };

        let result: Cell = Default::default();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_cell_is_alive() {
        let alive_cell = Cell {
            state: State::Alive,
        };
        let dead_cell = Cell { state: State::Dead };

        assert!(alive_cell.is_alive());
        assert!(!dead_cell.is_alive());
    }
}
