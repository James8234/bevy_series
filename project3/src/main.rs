use bevy::prelude::*;

mod player;
//mod floor;
//mod light;
mod camera;
mod world;
mod accumulate_input; 

use player::PlayerPlugin;
use world::floor::FloorPlugin;
use world::light::LightPlugin;
use camera::CameraPlugin;
use accumulate_input::AccumulateInputPlugin;

fn spawn_text(mut commands: Commands) {
    let font = TextFont {
        font_size: 25.0,
        ..default()
    };
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (Text::new("Move the player with WASD"), font.clone()),
            (Text::new("Rotate the camera with the mouse"), font)
        ],
    ));
}

fn main()
{
	App::new()
		.add_plugins((DefaultPlugins, PlayerPlugin, FloorPlugin, LightPlugin, CameraPlugin, AccumulateInputPlugin))
		.add_systems(Startup, spawn_text)
//		.add_systems(Update,
		.run();
}

/*
Note:
main.rs is like our header file, where we declear our functions
though plugins.
We then can include each function to other files using "use create::prelude"
just like in c++ "#include "stuff.h" in our .rs or .cpp files.
| c++           												  | Rust							   |
| --------------------------------------------------------------- | ---------------------------------- | ----------------------------------------------------------- |
| `header.h`                                                      | `main.rs`                          | Central place that declares what parts of the program exist |
| Function declaration in `.h`<br>`void printCreateAccount(...);` | `mod player;`                      | Declares that a module exists                               |
| `#include "player.h"`                                           | `use crate::player::PlayerPlugin;` | Imports symbols so they can be used                         |
| `player::printCreateAccount()`                                  | `crate::player::PlayerPlugin`      | Fully qualified name                                        |
| `using namespace player;`                                       | `use crate::player::*;`            | Brings all names into scope                                 |
| `.cpp` implementation                                           | `player.rs`                        | Actual code implementation                                  |
| Multiple headers                                                | Multiple `mod` lines               | Project structure definition                                |

*/
