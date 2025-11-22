use bevy::prelude::*;

mod player;
//mod floor;
//mod light;
mod camera;
mod world;

use player::PlayerPlugin;
use world::floor::FloorPlugin;
use world::light::LightPlugin;
use camera::CameraPlugin;

fn main()
{
	App::new()
		.add_plugins((DefaultPlugins, PlayerPlugin, FloorPlugin, LightPlugin, CameraPlugin))
//		.add_systems(Update,
		.run();
}


