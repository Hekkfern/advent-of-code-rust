/// Executes an action on an item repeatedly until a maximum number of iterations is reached or a loop is detected.
///
/// Loops are identified and managed to prevent infinite iterations, using internal caching mechanisms.
///
/// # Arguments
///
/// * `item` - The item to be modified through `iteration_action`.
/// * `max_num_iterations` - The maximum number of iterations to perform.
/// * `iteration_action` - The action to perform in each iteration. It takes a mutable reference to T and modifies it.
///
/// # Type Parameters
///
/// * `T` - The type of the item to be modified. Must implement `PartialEq` and `Clone`.
pub fn run<T, F>(item: &mut T, max_num_iterations: u64, mut iteration_action: F)
where
    T: PartialEq + Clone,
    F: FnMut(&mut T),
{
    assert!(
        max_num_iterations > 0,
        "max_num_iterations must be positive"
    );
    let mut cycle_counter: u64 = 0;
    let mut history: Vec<T> = Vec::with_capacity(256);
    while cycle_counter < max_num_iterations && !history.contains(item) {
        history.push(item.clone());
        iteration_action(item);
        cycle_counter += 1;
    }
    if let Some(offset) = history.iter().position(|x| x == item) {
        let offset = offset as u64;
        let cycle_length = cycle_counter - offset;
        if max_num_iterations > offset as u64 {
            let idx = offset as usize + ((max_num_iterations - offset) % cycle_length) as usize;
            *item = history[idx].clone();
        } else {
            *item = history.last().unwrap().clone();
        }
    } else if let Some(last) = history.last() {
        *item = last.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_simple() {
        let mut value = 0;
        run(&mut value, 1000, |v| {
            *v += 1;
            if *v > 7 {
                *v = 0;
            }
        });
        assert_eq!(value, 0);
    }

    #[test]
    fn run_no_loop() {
        let mut value = 0;
        run(&mut value, 5, |v| *v += 1);
        assert_eq!(value, 5);
    }

    #[test]
    fn run_with_loop() {
        let mut value = 0;
        run(&mut value, 10, |v| *v = (*v + 1) % 3);
        assert_eq!(value, 1);
    }

    #[test]
    fn run_exact_loop() {
        let mut value = 0;
        run(&mut value, 6, |v| *v = (*v + 1) % 3);
        assert_eq!(value, 0);
    }

    #[test]
    fn test_run_no_iterations() {
        let mut value = 42;
        run(&mut value, 0, |v| *v += 1);
        assert_eq!(value, 42);
    }
}
