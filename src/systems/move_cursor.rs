use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};

use crate::zuma_blast::Cursor;

#[derive(SystemDesc)]
pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        ReadStorage<'s, Cursor>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, cursor, input): Self::SystemData) {
        let is_left_movement = input.key_is_down(VirtualKeyCode::H);
        println!(is_left_movement);
        // for (paddle, transform) in (&Arena, &mut transforms).join() {
            // let movement = match paddle.side {
            //     Side::Left => input.axis_value("move_x"),
            //     Side::Right => input.axis_value("move_y"),
            // };
            // if let Some(mv_amount) = movement {
            //     if mv_amount != 0.0 {
            //         let side_name = match paddle.side {
            //             Side::Left => "left",
            //             Side::Right => "right",
            //         };
            //         println!("Side {:?} moving {}", side_name, mv_amount);
            //     }
            // }
        // }
    }
}
