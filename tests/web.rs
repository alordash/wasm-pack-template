//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::println;

use wasm_bindgen_test::*;

use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new(6, 6, false);
    universe.set_cells(true, &[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new(6, 6, false);
    universe.set_cells(true, &[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut input_universe = input_spaceship();
    println!("input universe:\n{}", input_universe);

    let expected_universe = expected_spaceship();
    let exp = format!("{}", expected_universe);
    println!("expected_universe:\n{}", expected_universe);

    input_universe.tick();
    let inp = format!("{}", input_universe);
    assert_eq!(inp, exp);
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
