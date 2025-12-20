use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, spawn_character);
	}
}

/// A vector representing the player's velocity in the physics simulation.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput {
    // The player's movement input (WASD).
	pub movement: Vec2,
    // Other input that could make sense would be e.g.
    // boost: bool
}

/// The actual position of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual representation.
///
/// If you want to make sure that this component is always initialized
/// with the same value as the `Transform`'s translation, you can
/// use a [component lifecycle hook](https://docs.rs/bevy/0.14.0/bevy/ecs/component/struct.ComponentHooks.html)
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(Vec3);


/// The value [`PhysicalTranslation`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PreviousPhysicalTranslation(Vec3);

fn spawn_character(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let player = (
		Name::new("Player"), // Create a component Name and stores value Player
		Mesh3d(meshes.add(Cuboid::new(1.0,1.0,1.0))),
		MeshMaterial3d(materials.add(Color::srgb_u8(124,144,255))),
		AccumulatedInput::default(),
		Velocity::default(),
		PhysicalTranslation::default(),
		PreviousPhysicalTranslation::default(),
		Transform {
			translation: Vec3::new(0.0,1.5,0.0),
			rotation: Quat::IDENTITY, //Placeholder for default
			scale: Vec3::splat(3.5),
		},
		);

	commands.spawn(player);
}


// ===== Explanation: Transform =====
/*
//Format:
pub struct Transform {
    pub translation: Vec3, // position
    pub rotation: Quat,    // rotation
    pub scale: Vec3,       // size
}

Position:
- Is based on a 3 dimensional axis - xyz
with the origin at (0,0,0) 
size:
- This scales the size of the object in directions given by each vector component
*/

/*
Function: Vec3::new(x, y, z)
Purpose:
- Creates a 3D vector with specific x, y, z components
- Used for positions, directions, velocities, or any 3D data in Bevy

Example:
- translation: Vec3::new(0.0, 0.5, 0.0)
  - x = 0.0 → horizontal center
  - y = 0.5 → slightly above the origin
  - z = 0.0 → depth center

Difference from Vec3::splat(value):
- Vec3::splat sets all three components to the same value
- Vec3::new lets you control each component individually
*/


/* ===== expletations of derives list =====
/// A vector representing the player's velocity in the physics simulation.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct Velocity(Vec3);
Debug:
	Allows you to println!("{:?}", velocity) for debugging
Component:
	Tells bevy velocity is attached to entities
	so you can use query::<(&Velocity, & Transform)>()
Clone:
	Allows you to make a copy of velocity 
Copy:
	Lets velocity copying be a simple assignment v1 = v2 instead of .clone()
PartialEq
	Allows you compare vectors v1 = v2, useful to check for change in velocity
Default:
	Allows you to create "Velocity::default()" for spawning player at rest
	Vec3 default is (0,0,0).
Deref and DerefMut:
	Lets you treat velocity like a Vec3 Vector directly
	velocity.x += 1.0; //works because of DerefMut
	let speed = velocity.length(); //Works because Deref
*/

/*
    Stores the entity's position from the previous physics tick.

    Used together with CurrentPhysicalTranslation to smoothly interpolate
    the entity's rendered position between physics updates. This prevents
    choppy movement when the render framerate is higher than the physics timestep.

    Example workflow:
        prev_position = PreviousPhysicalTranslation
        curr_position = CurrentPhysicalTranslation
        alpha = time_since_last_physics_tick / fixed_timestep
        rendered_position = prev_position.lerp(curr_position, alpha)

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PreviousPhysicalTranslation(Vec3);
*/
