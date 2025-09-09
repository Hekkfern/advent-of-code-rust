use aoc_geometry::Grid2D;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum GridCell {
    Empty,
    RoundedRock,
    CubeRock,
}

impl From<&str> for GridCell {
    fn from(s: &str) -> Self {
        let c = s.chars().next().unwrap();
        GridCell::from(c)
    }
}

impl From<char> for GridCell {
    fn from(c: char) -> Self {
        match c {
            '.' => GridCell::Empty,
            'O' => GridCell::RoundedRock,
            '#' => GridCell::CubeRock,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Rocks {
    grid: Grid2D<GridCell>,
}

impl Rocks {
    pub fn new(grid: Grid2D<GridCell>) -> Self {
        Self { grid }
    }

    // Shared helper to shift a mutable line of GridCell
    fn shift_line(line: &mut [&mut GridCell]) {
        let mut dst = 0;
        let mut src;
        while dst < line.len() {
            // Find next empty space
            while dst < line.len() && *line[dst] != GridCell::Empty {
                dst += 1;
            }
            if dst == line.len() {
                break;
            }
            src = dst;
            // Find next non-empty space
            while src < line.len() && *line[src] == GridCell::Empty {
                src += 1;
            }
            if src == line.len() {
                break;
            }
            match *line[src] {
                GridCell::RoundedRock => {
                    //swap values
                    let tmp = std::mem::replace(line[dst], GridCell::Empty);
                    *line[dst] = std::mem::replace(line[src], tmp);
                    dst += 1;
                }
                GridCell::CubeRock => {
                    dst = src;
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn shift_north(&mut self) {
        let width = self.grid.get_width();
        for col in 0..width {
            let mut mut_refs: Vec<_> = self.grid.get_column_mut(col).map(|x| x.1).rev().collect();
            Self::shift_line(mut_refs.as_mut_slice());
        }
    }

    pub fn shift_south(&mut self) {
        let width = self.grid.get_width();
        for col in 0..width {
            let mut mut_refs: Vec<_> = self.grid.get_column_mut(col).map(|x| x.1).collect();
            Self::shift_line(mut_refs.as_mut_slice());
        }
    }

    pub fn shift_west(&mut self) {
        let height = self.grid.get_height();
        for row in 0..height {
            let mut mut_refs: Vec<_> = self.grid.get_row_mut(row).map(|x| x.1).collect();
            Self::shift_line(mut_refs.as_mut_slice());
        }
    }

    pub fn shift_east(&mut self) {
        let height = self.grid.get_height();
        for row in 0..height {
            let mut mut_refs: Vec<_> = self.grid.get_row_mut(row).map(|x| x.1).rev().collect();
            Self::shift_line(mut_refs.as_mut_slice());
        }
    }

    pub fn calculate_load(&self) -> u64 {
        let total: u64 = self
            .grid
            .iter_all()
            .filter(|(_, val)| val == &&GridCell::RoundedRock)
            .map(|(coord, _)| coord[1] as u64 + 1)
            .sum();
        total
    }
}
