mod node;
mod state;

use crate::node::Node;
use crate::state::State;
use aoc_geometry::{CardinalDirection2D, Grid2D, GridCoordinate2D};
use std::collections::{BinaryHeap, HashMap};

type HeatLossGrid = Grid2D<u8>;

fn parse_input(input: &str) -> HeatLossGrid {
    let matrix = aoc_utils::string_utils::convert_to_matrix_of_digits(input);
    let mut grid = HeatLossGrid::from_double_vec(matrix);
    grid.flip_vertical();
    grid
}

fn get_next_steps(
    grid: &HeatLossGrid,
    current_state: &State,
    min_steps: u8,
    max_steps: u8,
) -> Vec<(GridCoordinate2D, CardinalDirection2D)> {
    let mut next_steps = Vec::with_capacity(3);
    if current_state.steps < max_steps {
        // Continue in the same direction
        if let Some(next_position) = grid.try_move(
            &current_state.position,
            &current_state.direction.to_vector(),
        ) {
            next_steps.push((next_position, current_state.direction));
        }
    }
    if current_state.steps >= min_steps {
        // Turn left
        let left_direction = current_state.direction.rotate_counter_clockwise();
        if let Some(next_position) =
            grid.try_move(&current_state.position, &left_direction.to_vector())
        {
            next_steps.push((next_position, left_direction));
        }
        // Turn right
        let right_direction = current_state.direction.rotate_clockwise();
        if let Some(next_position) =
            grid.try_move(&current_state.position, &right_direction.to_vector())
        {
            next_steps.push((next_position, right_direction));
        }
    }
    next_steps
}

fn get_least_heat_loss_path(grid: &HeatLossGrid, min_steps: u8, max_steps: u8) -> u32 {
    assert!(min_steps <= max_steps, "min_steps must be less than or equal to max_steps");
    let destination = GridCoordinate2D::new([grid.get_width() - 1, 0]);
    let origin = GridCoordinate2D::new([0, grid.get_height() - 1]);
    let starting_through_east = State {
        position: origin,
        direction: CardinalDirection2D::Right,
        steps: 0,
        heat_loss: 0,
    };
    let starting_through_south = State {
        position: origin,
        direction: CardinalDirection2D::Down,
        steps: 0,
        heat_loss: 0,
    };
    let mut pq = BinaryHeap::<State>::new();
    let mut best = HashMap::<Node, u32>::new();

    /// Helper that adds a given state to the queue if it is better than our best
    /// known for that position, direction and remaining steps
    fn push_if_better(best: &mut HashMap<Node, u32>, pq: &mut BinaryHeap<State>, s: State) {
        let node = Node {
            position: s.position,
            direction: s.direction,
            steps: s.steps,
        };
        if best.get(&node).map_or(true, |&b| s.heat_loss < b) {
            best.insert(node, s.heat_loss);
            pq.push(s);
        }
    }
    /* Two starting states, {0,0}, going right and down. Note that steps == 0 and heat_loss == 0 */
    push_if_better(&mut best, &mut pq, starting_through_east);
    push_if_better(&mut best, &mut pq, starting_through_south);

    while let Some(current_state) = pq.pop() {
        /* If we are at our destination, report the heat_loss, because of the priority queue we
        have a guarantee this is the minimum value */
        if current_state.position == destination {
            if current_state.steps < min_steps {
                continue;
            }
            return current_state.heat_loss;
        }

        /* check whether this state is already worse than the best known */
        let current_node = Node {
            position: current_state.position,
            direction: current_state.direction,
            steps: current_state.steps,
        };
        if let Some(lowest_heat_loss) = best.get(&current_node) {
            if current_state.heat_loss > *lowest_heat_loss {
                continue;
            }
        }

        let next_steps = get_next_steps(grid, &current_state, min_steps, max_steps);
        for (next_position, next_direction) in next_steps {
            let next_heat_loss = current_state.heat_loss + grid[&next_position] as u32;
            let next_steps = if next_direction == current_state.direction {
                current_state.steps + 1
            } else {
                1
            };
            let next_state = State {
                position: next_position,
                direction: next_direction,
                steps: next_steps,
                heat_loss: next_heat_loss,
            };
            push_if_better(&mut best, &mut pq, next_state);
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
    let grid = parse_input(params.input_data);
    let result = get_least_heat_loss_path(&grid, 1, 3);
    result.to_string()
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
    let grid = parse_input(params.input_data);
    let result = get_least_heat_loss_path(&grid, 4, 10);
    result.to_string()
}
