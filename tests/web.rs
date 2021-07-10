//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut uni = Universe::new_blank(6, 6);
    uni.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    uni
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut uni = Universe::new_blank(6, 6);
    uni.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    uni
}

#[wasm_bindgen_test]
fn test_tick() {
    let mut input_uni = input_spaceship();
    
    let expected_uni = expected_spaceship();

    input_uni.tick();
    assert_eq!(
	input_uni.get_cells_hash(),
	expected_uni.get_cells_hash()
    );
}
