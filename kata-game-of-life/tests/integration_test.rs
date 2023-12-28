use kata_game_of_life::{cell, world::World};

const D: cell::Status = cell::Status::Dead;
const A: cell::Status = cell::Status::Alive;
#[test]
fn generates_the_next_state_of_the_world() {
    // D A D
    // D A D
    // D A D
    let world = World::new(vec![vec![D, A, D], vec![D, A, D], vec![D, A, D]]).unwrap();

    // D D D
    // A A A
    // D D D
    let new_world = World::new(vec![vec![D, D, D], vec![A, A, A], vec![D, D, D]]).unwrap();
    assert_eq!(world.calculate_next_generation().unwrap(), new_world);
}

#[test]
fn never_changes_for_a_given_initial_block_pattern() {
    let initial_world = World::new(vec![
        vec![A, A, D, D, D],
        vec![A, A, D, D, D],
        vec![D, D, D, D, D],
        vec![D, D, D, D, D],
        vec![D, D, D, D, D],
    ])
    .unwrap();
    let current_world = initial_world
        .calculate_next_generation()
        .unwrap()
        .calculate_next_generation()
        .unwrap()
        .calculate_next_generation()
        .unwrap();
    assert_eq!(initial_world, current_world);
    assert_eq!(current_world.generation_count, 3);
}

#[test]
fn reestablishes_the_same_state_after_two_generations_when_a_given_oscillator_pattern_is_provided()
{
    let initial_world = World::new(vec![
        vec![D, D, D, D, D],
        vec![D, D, A, D, D],
        vec![D, D, A, D, D],
        vec![D, D, A, D, D],
        vec![D, D, D, D, D],
    ])
    .unwrap();
    let current_world = initial_world
        .calculate_next_generation()
        .unwrap()
        .calculate_next_generation()
        .unwrap();
    assert_eq!(initial_world, current_world);
    assert_eq!(current_world.generation_count, 2);
}
