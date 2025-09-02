mod pipe_type;

use crate::pipe_type::PipeType;
use aoc_geometry::Grid2D;
use aoc_geometry::OrthogonalPolygon2D;
use aoc_geometry::Point;
use aoc_geometry::Vector;

type Field = Grid2D<PipeType>;
type VectorDirection = Vector<i8, 2>;
type GridPosition = Point<usize, 2>;

fn parse_input(input: &str) -> (Field, GridPosition) {
    let mut grid_data = Vec::new();
    let mut start = GridPosition::new([0, 0]);
    input.trim().lines().enumerate().for_each(|(y, line)| {
        let line = line.trim();
        let mut row: Vec<PipeType> = Vec::with_capacity(line.len());
        line.chars().enumerate().for_each(|(x, c)| {
            row.insert(x, PipeType::from(c));
            if c == 'S' {
                start = GridPosition::new([x, y]);
            }
        });
        grid_data.push(row);
    });
    // Flip the grid vertically to have (0,0) at the bottom-left
    let mut field = Grid2D::from_double_vec(grid_data);
    field.flip_vertical();
    // Adjust the start position accordingly
    start = Point::new([
        start.get(0).clone(),
        (field.get_height() - 1) - start.get(1).clone(),
    ]);

    (field, start)
}

fn get_pipe_translation(pipe: PipeType) -> (VectorDirection, VectorDirection) {
    match pipe {
        PipeType::SouthEast => (VectorDirection::new([0, -1]), VectorDirection::new([1, 0])),
        PipeType::Horizontal => (VectorDirection::new([-1, 0]), VectorDirection::new([1, 0])),
        PipeType::NorthWest => (VectorDirection::new([0, 1]), VectorDirection::new([-1, 0])),
        PipeType::Vertical => (VectorDirection::new([0, 1]), VectorDirection::new([0, -1])),
        PipeType::NorthEast => (VectorDirection::new([0, 1]), VectorDirection::new([1, 0])),
        PipeType::SouthWest => (VectorDirection::new([0, -1]), VectorDirection::new([-1, 0])),
        _ => unreachable!(),
    }
}

fn get_pipe_type_at(field: &Field, position: &GridPosition) -> PipeType {
    field.get(&position).unwrap().clone()
}

fn move_across_field(
    field: &Field,
    pipe_position: &GridPosition,
    previous_position: &GridPosition,
) -> GridPosition {
    let translation = get_pipe_translation(get_pipe_type_at(&field, &pipe_position));
    let new_position_1 = pipe_position.move_by(&translation.0).unwrap();
    let new_position_2 = pipe_position.move_by(&translation.1).unwrap();
    if new_position_1 == *previous_position {
        new_position_2
    } else {
        new_position_1
    }
}

fn get_starting_neighbor(field: &Field, start: &GridPosition) -> GridPosition {
    for neighbor in start.get_neighbours() {
        let pipe_type = get_pipe_type_at(field, &neighbor);
        if matches!(pipe_type, PipeType::None) {
            continue;
        }
        let translation = get_pipe_translation(pipe_type);
        if let Some(candidate1) = neighbor.move_by(&translation.0) {
            if candidate1 == *start {
                return neighbor;
            }
        }
        if let Some(candidate2) = neighbor.move_by(&translation.1) {
            if candidate2 == *start {
                return neighbor;
            }
        }
    }
    unreachable!()
}

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

/// Solves Part 1 of the puzzle
///
/// # Arguments
///
/// * `params` - Parameters for the solver
///
/// # Returns
///
/// The solution as a string
pub fn solve_part1(params: Part1Parameters) -> String {
    let (field, start) = parse_input(params.input_data);
    let mut current_position = get_starting_neighbor(&field, &start);
    let mut previous_position = start;

    let mut count: u64 = 1;
    while current_position != start {
        let next_position = move_across_field(&field, &current_position, &previous_position);
        previous_position = current_position;
        current_position = next_position;
        count += 1;
    }
    (count / 2).to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

/// Solves Part 2 of the puzzle
///
/// # Arguments
///
/// * `params` - Parameters for the solver
///
/// # Returns
///
/// The solution as a string
pub fn solve_part2(params: Part2Parameters) -> String {
    let (field, start) = parse_input(params.input_data);
    let mut current_position = get_starting_neighbor(&field, &start);
    let mut previous_position = start;

    let mut points: Vec<GridPosition> = vec![start];
    while current_position != start {
        let next_position = move_across_field(&field, &current_position, &previous_position);
        previous_position = current_position;
        current_position = next_position;
        points.push(previous_position);
    }
    let polygon = OrthogonalPolygon2D::<usize>::from_vertices(points);
    polygon.number_of_intrinsic_points().to_string()
}
