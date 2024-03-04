use std::collections::HashMap;
use std::collections::VecDeque;
use bevy::render::mesh::shape::RegularPolygon;
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    //sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};


struct Tile {
    pos: (i32, i32, i32),
    height: f32,
}

#[derive(Component)]
struct CameraMarker;

type Tiles = HashMap<(i32, i32, i32), Tile>;
const EDGE_LENGTH: f32 = 20.0;

//#[derive(Resource)]
struct TileMap {
    tiles: Tiles,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();

}

/// The initial setup function
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query_camera: Query<&mut OrthographicProjection, With<CameraMarker>>,
    //mut tile_map: ResMut<TileMap>,
) {
    
    let mut tiles: Tiles = HashMap::new();
    //tile_map.tiles = tiles;
    //grid(&mut tile_map.tiles);
    grid(&mut tiles);

    //use bevy::render::camera::ScalingMode;
    commands.spawn((Camera2dBundle::default(), CameraMarker));
    //let mut projection = query_camera.single_mut();
    //projection.scale = 1.5;
    //projection.scaling_mode = ScalingMode::WindowSize(4.0);
    
    const angle_up: f32 = 0.0;
    const angle_down: f32 = 180.0;
    let mut angle: f32 = 0.0;

    for triangle in tiles.keys() {
        let temp_pos = cart_coords(*triangle);

        if is_up(*triangle){ angle = angle_up } else { angle = angle_down }

        commands.spawn(MaterialMesh2dBundle {
            mesh: bevy::sprite::Mesh2dHandle(meshes.add(Mesh::from(RegularPolygon::new(10.0, 3)))),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(
                temp_pos.0,
                temp_pos.1,
                0.0,
            ).with_rotation(Quat::from_rotation_z((angle).to_radians())),
            ..default()
        });


    }


}

/// Generate triangular-hex grid
fn grid(tiles: &mut HashMap<(i32, i32, i32), Tile>) {
    const GRID_SIZE: i32 = 12;

    #[derive(Copy, Clone)]
    enum Dir {
        NW, N, NE,
        SW, S, SE,
    }

    let mut process_q: VecDeque<(Dir, (i32, i32, i32))> = VecDeque::new();
    let mut next_q: VecDeque<(Dir, (i32, i32, i32))> = VecDeque::new();

    process_q.push_back((Dir::NW, (0, 1, 1)));
    process_q.push_back((Dir::N, (0, 1, 0)));
    process_q.push_back((Dir::NE, (1, 1, 0)));
    process_q.push_back((Dir::SW, (0, 0, 1)));
    process_q.push_back((Dir::S, (1, 0, 1)));
    process_q.push_back((Dir::SE, (1, 0, 0)));

    let mut q_itr = 0;
    let mut next = true;

    while q_itr < GRID_SIZE {

        while !process_q.is_empty() {

            let temp_tuple = process_q.front().unwrap().1;

            if (q_itr + 1 != GRID_SIZE) {

                // Instanciate neighbors
                match process_q.front().unwrap().0 {
                    Dir::NW => if is_up(temp_tuple) {
                        // Is_up: add NW point
                        next_q.push_back((Dir::NW, (temp_tuple.0 - 1, temp_tuple.1, temp_tuple.2)));
                    } else {
                        // Is_down: add N and SW points
                        next_q.push_back((Dir::N, (temp_tuple.0, temp_tuple.1 + 1, temp_tuple.2)));
                        next_q.push_back((Dir::SW, (temp_tuple.0, temp_tuple.1, temp_tuple.2 + 1)));
                    },
                    Dir::N => if is_up(temp_tuple) {
                        // Is_up: add NW and NE points
                        next_q.push_back((Dir::NW, (temp_tuple.0 - 1, temp_tuple.1, temp_tuple.2)));
                        next_q.push_back((Dir::NE, (temp_tuple.0, temp_tuple.1, temp_tuple.2 - 1)));
                    } else {
                        // Is_down: add N point
                        next_q.push_back((Dir::N, (temp_tuple.0, temp_tuple.1 + 1, temp_tuple.2)));
                    },
                    Dir::NE => if is_up(temp_tuple) {
                        // Is_up: add NE point
                        next_q.push_back((Dir::NE, (temp_tuple.0, temp_tuple.1, temp_tuple.2 - 1)));
                    } else {
                        // Is_down: add N and SE points
                        next_q.push_back((Dir::N, (temp_tuple.0, temp_tuple.1 + 1, temp_tuple.2)));
                        next_q.push_back((Dir::SE, (temp_tuple.0 + 1, temp_tuple.1, temp_tuple.2)));
                    },
                    Dir::SW => if is_up(temp_tuple) {
                        // Is_up: add NW and S points
                        next_q.push_back((Dir::NW, (temp_tuple.0 - 1, temp_tuple.1, temp_tuple.2)));
                        next_q.push_back((Dir::S, (temp_tuple.0, temp_tuple.1 - 1, temp_tuple.2)));
                    } else {
                        // Is_down: add SW point
                        next_q.push_back((Dir::SW, (temp_tuple.0, temp_tuple.1, temp_tuple.2 + 1)));
                    },
                    Dir::S => if is_up(temp_tuple) {
                        // Is_up: add S point
                        next_q.push_back((Dir::S, (temp_tuple.0, temp_tuple.1 - 1, temp_tuple.2)));
                    } else {
                        // Is_down: add SW and SE points
                        next_q.push_back((Dir::SW, (temp_tuple.0, temp_tuple.1, temp_tuple.2 + 1)));
                        next_q.push_back((Dir::SE, (temp_tuple.0 + 1, temp_tuple.1, temp_tuple.2)));
                    },
                    Dir::SE => if is_up(temp_tuple) {
                        // Is_up: add NE and S points
                        next_q.push_back((Dir::NE, (temp_tuple.0 - 1, temp_tuple.1, temp_tuple.2)));
                        next_q.push_back((Dir::S, (temp_tuple.0, temp_tuple.1 - 1, temp_tuple.2)));
                    } else {
                        // Is_down: add SE point
                        next_q.push_back((Dir::SE, (temp_tuple.0 + 1, temp_tuple.1, temp_tuple.2)));
                    },
                }
            }

            //process_q.pop_front().unwrap().1
            let newtile: Tile = Tile {pos: process_q.pop_front().unwrap().1, height: 0.0};
            
            tiles.insert(newtile.pos, newtile);            
        } 
        

        // process_q is empty and next_q has data to be moved
        for x in next_q.iter() {
            process_q.push_back(*x);
        }
        next_q.clear(); // Sanity check

        // Only increment q_itr every two grow operations
        if !next { q_itr += 1; } 
            
        next = !next;
        
    }


}

/// Returns true if triangle points up, and false if it points down
fn is_up(pos: (i32, i32, i32)) -> bool {
    return pos.0 + pos.1 + pos.2 == 2;
}

/// Returns the cartesian coordinates, given triangular coordinates
fn cart_coords(pos: (i32, i32, i32)) -> (f32, f32) {
    let sqrt3: f32 = 3.0f32.sqrt();
    return ((0.5 * pos.0 as f32 + -0.5 * pos.2 as f32) * EDGE_LENGTH,
        (-sqrt3 / 6.0 * pos.0 as f32 + sqrt3 / 3.0 * pos.1 as f32 - sqrt3 / 6.0 * pos.2 as f32) * EDGE_LENGTH);
}