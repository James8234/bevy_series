use bevy::prelude::*;

mod player;
//mod floor;
//mod light;
mod camera;
mod world;
mod accumulate_input;
mod rotate_camera;
mod translate_camera;

use translate_camera::TranslateCameraPlugin;
use player::PlayerPlugin;
use world::floor::FloorPlugin;
use world::light::LightPlugin;
use camera::CameraPlugin;
use accumulate_input::AccumulateInputPlugin;
use rotate_camera::RotateCameraPlugin;

fn main()
{
	App::new()
		.add_plugins((DefaultPlugins, PlayerPlugin, FloorPlugin, LightPlugin, CameraPlugin, AccumulateInputPlugin, RotateCameraPlugin, TranslateCameraPlugin))
		.run();
}


