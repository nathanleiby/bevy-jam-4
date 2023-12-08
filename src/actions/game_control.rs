use bevy::prelude::{Input, KeyCode, Res};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    RotateLeft,
    RotateRight,
}

impl GameControl {
    pub fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
            GameControl::RotateLeft => {
                keyboard_input.pressed(KeyCode::Q) || keyboard_input.pressed(KeyCode::J)
            }
            GameControl::RotateRight => {
                keyboard_input.pressed(KeyCode::E) || keyboard_input.pressed(KeyCode::L)
            }
        }
    }
}

pub fn get_movement(control: GameControl, input: &Res<Input<KeyCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}
