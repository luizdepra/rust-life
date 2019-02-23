use itertools::iproduct;

use crate::generation::Generation;

pub struct Ruleset;

impl Ruleset {
    pub fn should_live(x: u16, y: u16, gen: &Generation) -> bool {
        let anc = Ruleset::alive_neighbors(x, y, gen);
        (gen.cell(x, y).is_alive() && anc == 2) || anc == 3
    }

    fn alive_neighbors(x: u16, y: u16, gen: &Generation) -> usize {
        let x = x as i16;
        let y = y as i16;

        iproduct!((x - 1)..=(x + 1), (y - 1)..=(y + 1))
            .filter(|(ix, iy)| {
                let fixed_x = Ruleset::fix_coord(*ix, 0, gen.width - 1);
                let fixed_y = Ruleset::fix_coord(*iy, 0, gen.height - 1);
                !(fixed_x == x as u16 && fixed_y == y as u16)
                    && gen.cell(fixed_x, fixed_y).is_alive()
            })
            .count()
    }

    fn fix_coord(c: i16, lower: u16, upper: u16) -> u16 {
        if c < lower as i16 {
            upper
        } else if c > upper as i16 {
            lower
        } else {
            c as u16
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::Generation;

    #[test]
    fn test_fix_coord() {
        let values: Vec<(i16, u16, u16, u16)> = vec![
            (1, 0, 2, 1),
            (0, 0, 2, 0),
            (2, 0, 2, 2),
            (-1, 0, 2, 2),
            (3, 0, 2, 0),
        ];

        for (c, l, u, e) in &values {
            assert_eq!(Ruleset::fix_coord(*c, *l, *u), *e);
        }
    }

    #[test]
    fn test_alive_neighbors() {
        let mut gen = Generation::new(3, 3);

        gen.mut_cell(1, 1).spawn();
        gen.mut_cell(1, 2).spawn();
        gen.mut_cell(2, 2).spawn();

        assert_eq!(Ruleset::alive_neighbors(1, 1, &gen), 2);
        assert_eq!(Ruleset::alive_neighbors(0, 0, &gen), 3);

        gen.mut_cell(1, 2).kill();
        gen.mut_cell(2, 2).kill();

        assert_eq!(Ruleset::alive_neighbors(1, 1, &gen), 0);
        assert_eq!(Ruleset::alive_neighbors(2, 2, &gen), 1);
    }

    #[test]
    fn test_should_live() {
        let mut gen = Generation::new(5, 5);

        gen.mut_cell(1, 1).spawn();
        gen.mut_cell(1, 2).spawn();
        gen.mut_cell(2, 2).spawn();

        assert_eq!(Ruleset::should_live(0, 0, &gen), false);
        assert_eq!(Ruleset::should_live(2, 2, &gen), true);
        assert_eq!(Ruleset::should_live(1, 2, &gen), true);
    }
}
