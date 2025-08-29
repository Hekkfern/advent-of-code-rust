use aoc_geometry::point::Point;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct GalaxyMap {
    galaxies: Vec<Point<usize, 2>>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

impl GalaxyMap {
    pub fn new(
        galaxies: Vec<Point<usize, 2>>,
        empty_rows: Vec<usize>,
        empty_columns: Vec<usize>,
    ) -> Self {
        Self {
            galaxies,
            empty_rows,
            empty_columns,
        }
    }

    pub fn get_galaxies(&self) -> &Vec<Point<usize, 2>> {
        &self.galaxies
    }

    pub fn get_empty_rows(&self) -> &Vec<usize> {
        &self.empty_rows
    }

    pub fn get_empty_columns(&self) -> &Vec<usize> {
        &self.empty_columns
    }
}
