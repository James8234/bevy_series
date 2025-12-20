use bevy::prelude::*;

use crate::player::{Velocity, AccumulatedInput};

pub struct AccumulateInputPlugin;

impl Plugin for AccumulateInputPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, spawn_accumulateinput);
	}
}

// Since Bevy assumes SI units, this has the unit of meters per second
// Note that aout 1.5 is teh average walking speed of humans
const SPEED: f32 = 0.09;

fn spawn_accumulateinput(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	player: Single<(&mut AccumulatedInput, &mut Velocity, &mut Transform)>,	

) {

	let (mut input, mut velocity, mut movement) = player.into_inner();
	 
//	let (mut input, mut velocity) = player.into_inner(); 
// -> into_inner is a wrapper type for QueryItem and searches for the components
// -> listed under player: Single<(&mut AccumulatedInput, &mut Velocity)>

// Reset the input to zero before reading the new input. As mentioned above, we can only do this
// because this is continuously pressed by the user. Do not reset e.g. whether the user wants to boost.
    input.movement = Vec2::ZERO;
	if keyboard_input.pressed(KeyCode::KeyW) {
		input.movement.y += 1.0;
	}

    if keyboard_input.pressed(KeyCode::KeyS) {
		input.movement.y -= 1.0;
    }

	if keyboard_input.pressed(KeyCode::KeyA) {
		input.movement.x -= 1.0;
	}

    if keyboard_input.pressed(KeyCode::KeyD) {
        input.movement.x += 1.0;
    }

//Remap the 2D input to Bevy 3D coordinate system
// input.y is up, z is forward
    let input_3d = Vec3 {
        x: input.movement.x,
        y: 0.0,
        z: -input.movement.y,
    };

	// We need to normalize and scale because otherwise
    // diagonal movement would be faster than horizontal or vertical movement.
    // We use `clamp_length_max` instead of `.normalize_or_zero()` because gamepad input
    // may be smaller than 1.0 when the player is pushing the stick just a little bit.
	// velocity and .0 is used to access the vector, like de-referencing a point
	velocity.0 = input_3d * SPEED;
	movement.translation += velocity.0;

//    velocity.0 = rotated_input.clamp_length_max(1.0) * SPEED;

	//.just_released can be used to check one input;
}



//===== Commands =====
/*
Res<ButtonInput<KeyCode>>
- Tells bevy "Give me read access to the current keyboard state this frame
Res<T>
- Read only access to a resource
ButtonInput<KeyCode>
- Bevy resource that stores current state of keyboard

*/

// ===== const SPEED: f32 = 4.0;
/*
Bevy 3D render assumes SI units
- Position and distance are measued in meteres
- Time is in seconds
- 4.0 is much faster than walking

*/

// ===== let (mut input, mut velocity) = player.into_inner(); =====
/*
    Query Tuples in Bevy ECS

    In Bevy, a query tuple lets a system access multiple components of entities 
    that match certain criteria.
	Provided by
	- player: Single<(&mut AccumulatedInput, &mut Velocity, &mut Transform)>,
 
    Example:
        Query<(&mut AccumulatedInput, &mut Velocity), With<Player>>

    Explanation:
    - (&mut AccumulatedInput, &mut Velocity): the tuple of components you want.
    - With<Player>: a filter that only includes entities with the Player component.
    - Bevy ECS returns components in the same order as the tuple. Order matters.
    - The query does not rely on variable names, only component types and order.
    - Each entity that matches the query gets one tuple of references.

    Usage:
        for mut player in query.iter_mut() {
            let (mut input, mut velocity) = player.into_inner();
            // input -> &mut AccumulatedInput
            // velocity -> &mut Velocity
        }

    Notes:
    - Adding more components is easy; just extend the tuple:
        Query<(&mut AccumulatedInput, &mut Velocity, &Transform), With<Player>>
    - This system will then unpack the components in the same order.
*/

// ===== commands.spawn(accumulateinput); =====
/*
You spawn things when you want to create new entities.
You do NOT spawn when you want to update or control existing entities.
*/


