use crate::mesh::Mesh;
use crate::module::ModuleName;
use crate::module::broadcaster::Broadcaster;
use crate::module::conjunction::Conjunction;
use crate::module::flip_flop::FlipFlop;
use crate::module::module_base::ModuleTrait;
use crate::signal::Signal;
use crate::signal_value::SignalValue;
use num_integer::Integer;
use std::collections::{HashMap, VecDeque};

mod mesh;
mod module;
mod signal;
mod signal_value;

// -----------------------------------------------------------
// ------------------------ Common ---------------------------
// -----------------------------------------------------------

fn get_destinations_iter(line: &str) -> impl Iterator<Item = &str> {
    line[line.find("->").unwrap() + 3..].split(", ")
}

fn add_module_to_mesh(mesh: &mut Mesh, line: &str, mut module: Box<dyn ModuleTrait>) -> () {
    for destination in get_destinations_iter(line) {
        let dest_name = ModuleName::from(destination);
        module.add_destination(&dest_name);
    }
    mesh.add_module(module);
}

/// Parse a line and create the corresponding module. Then, it is added to the mesh.
fn parse_input_line(mesh: &mut Mesh, line: &str) -> () {
    match &line[0..1] {
        "%" => {
            /* it is a flip-flop */
            let name = ModuleName::from(&line[1..line.find(' ').unwrap()]);
            let flip_flop = FlipFlop::new(name);
            add_module_to_mesh(mesh, line, Box::new(flip_flop));
        }
        "&" => {
            /* it is a conjunction */
            let name = ModuleName::from(&line[1..line.find(' ').unwrap()]);
            let conjunction = Conjunction::new(name);
            add_module_to_mesh(mesh, line, Box::new(conjunction));
        }
        _ => {
            /* it is a broadcaster */
            let broadcaster = Broadcaster::new(String::from("broadcaster"));
            add_module_to_mesh(mesh, line, Box::new(broadcaster));
        }
    }
}

fn parse_input(input: &str) -> Mesh {
    let mut mesh = Mesh::new();
    input.trim().lines().for_each(|line| {
        parse_input_line(&mut mesh, line.trim());
    });
    mesh.setup();
    mesh
}

// -----------------------------------------------------------
// ------------------------ Part 1 ---------------------------
// -----------------------------------------------------------

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
    let mut mesh = parse_input(params.input_data);
    let (low_pulses, high_pulses): (u64, u64) = (0..1_000)
        .map(|_| {
            let mut low_pulses: u64 = 0;
            let mut high_pulses: u64 = 0;
            let mut signals_queue = VecDeque::<Signal>::new();
            /* process the button signal */
            let button_signal = Signal::new(
                String::from("button"),
                String::from("broadcaster"),
                SignalValue::Low,
            );
            signals_queue.push_front(button_signal);
            low_pulses += 1;
            /* process the signals */
            while let Some(signal_to_process) = signals_queue.pop_front() {
                let output_signals = mesh.process(&signal_to_process);
                for output_signal in output_signals {
                    /* increase signal counters */
                    match output_signal.value() {
                        SignalValue::Low => low_pulses += 1,
                        SignalValue::High => high_pulses += 1,
                    }
                    /* add the signal to the queue */
                    signals_queue.push_back(output_signal);
                }
            }
            (low_pulses, high_pulses)
        })
        .fold((0, 0), |(acc_low, acc_high), (low, high)| {
            (acc_low + low, acc_high + high)
        });
    (low_pulses * high_pulses).to_string()
}

// -----------------------------------------------------------
// ------------------------ Part 2 ---------------------------
// -----------------------------------------------------------

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
    let mut mesh = parse_input(params.input_data);
    /* find the conjunctions connected to the conjunction connected to rx node */
    let previous_node_to_rx = mesh.get_modules_connected_to(&ModuleName::from("rx"))[0].clone();
    let more_previous_node_to_rx = mesh.get_modules_connected_to(&previous_node_to_rx);
    let mut button_presses: u64 = 0;
    let mut last: HashMap<ModuleName, u64> = HashMap::new();
    let mut loops: HashMap<ModuleName, u64> = HashMap::new();
    while loops.len() < more_previous_node_to_rx.len() {
        let mut signals_queue = VecDeque::<Signal>::new();
        /* process the button signal */
        let button_signal = Signal::new(
            String::from("button"),
            String::from("broadcaster"),
            SignalValue::Low,
        );
        signals_queue.push_front(button_signal);
        button_presses += 1;
        /* process the signals */
        while let Some(signal_to_process) = signals_queue.pop_front() {
            let output_signals = mesh.process(&signal_to_process);
            for output_signal in output_signals {
                /* check if we have reached one of the nodes we are tracking */
                if previous_node_to_rx == *output_signal.destination()
                    && output_signal.value() == SignalValue::High
                {
                    if last.contains_key(&*output_signal.origin()) {
                        loops.insert(
                            output_signal.origin().clone(),
                            button_presses - last[&*output_signal.origin()],
                        );
                    }
                    last.insert(output_signal.origin().clone(), button_presses);
                }
                /* add the signal to the queue */
                signals_queue.push_back(output_signal);
            }
        }
    }
    let result = loops.values().fold(1, |accum, value| accum.lcm(value));
    result.to_string()
}
