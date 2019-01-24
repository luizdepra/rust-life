use crate::cell::Cell;

/// Represents a cell generation.
#[derive(Debug, Clone, PartialEq)]
pub struct Generation {
    /// The generation width.
    pub width: u16,

    /// The generation height.
    pub height: u16,

    /// Cells vector.
    pub cells: Box<[Cell]>,
}

impl Generation {
    /// Creates a new generation.
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width * height) as usize;

        Generation {
            width,
            height,
            cells: vec![Default::default(); size].into_boxed_slice(),
        }
    }

    pub fn cell(&self, x: u16, y: u16) -> &Cell {
        let position = (y * self.width + x) as usize;
        &self.cells[position]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_generation() {
        let width: u16 = 42;
        let height: u16 = 13;
        let size = (width * height) as usize;

        let expected = Generation {
            width,
            height,
            cells: vec![Default::default(); size].into_boxed_slice(),
        };

        let generation = Generation::new(width, height);

        assert_eq!(generation, expected);
    }
}
