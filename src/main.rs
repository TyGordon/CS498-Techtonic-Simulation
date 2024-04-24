// Libraries/Crates/Packages
use bevy::
{
    app::AppExit, prelude::*,
    render::
    {
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::{PrimitiveTopology, Extent3d, TextureDimension, TextureFormat},
        camera::RenderTarget,
    },
    window::WindowRef,
};

fn main()
{
    // Create a vector of structs to hold the height values
    let pub mut h = Vec::<Vec<HeightValues>>::new();

    // Create the main menu app
    let mut app = App::new();

    // Insert the heights into the app
    app.insert_resource(tiles);

    // Add systems to the main app
    app.add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::MainMenu), (camera_setup, setup, setup2))
        .add_systems(Update, (button_system.run_if(in_state(AppState::MainMenu)), input_handler.run_if(in_state(AppState::MainMenu))))
        .add_systems(OnEnter(AppState::Simulate), setup2)
        .add_systems(Update, input_handler.run_if(in_state(AppState::Simulate)));

    // Run the main app
    app.run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum AppState
{
    #[default]
    MainMenu,
    Simulate,
}

#[derive(Component)]
struct Shape;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

// A unit struct to identify which menu button was pressed
#[derive(Component)]
enum MenuAction
{
    Play,
    LoadFile,
    SelectFolder,
    Quit,
}

#[derive(Resource)]
struct HeightValues {
        water : f32
        suspended_sediment : f32
        bed_sediment : f32
        bedrock : f32
        n_volume : f32
        n_speed : f32
        e_volume : f32
        e_speed : f32
        s_volume : f32
        s_speed : f32
        w_volume : f32
        w_speed : f32
    }



// This function creates a camera (can be used for main app and subapp)
fn camera_setup(mut commands: Commands)
{
    // Create the UI window (camera)
    commands.spawn
    (
        Camera3dBundle
        {
            transform: Transform::from_translation(Vec3::new(0., 0., 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera
            {
                target: RenderTarget::default(),

                // Change the background color of the window
                clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.2, 0.6274509)),

                ..default()
            },

            ..default()
        }
    );
}

// This function handles
//  1. When a button is pressed
//  2. When the mouse hovers over a button
//  3. When the mouse is not hovering over a button
fn button_system
(
    mut interaction_query: Query<
        (
            &Interaction,
            &MenuAction,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,

    mut exit_app: EventWriter<AppExit>,

    mut commands: Commands,

    mut entity_query: Query<(Entity, &Transform), With<Shape>>,
    //mut window_query: Query<(Entity, &Camera), With<Transform>>,
    mut next_state: ResMut<NextState<AppState>>,
)
{
    for (interaction, menu_action, mut border_color) in &mut interaction_query
    {

        match *interaction
        {
            // If the button is pressed, determine which one was pressed
            Interaction::Pressed =>
            {
                match *menu_action
                {
                    MenuAction::Play =>
                    {
                        // If the start button was pressed, close the main menu and start the simulation.

                        // Delete the icosahedron
                        let (entity, _) = entity_query.single_mut();
                        commands.entity(entity).despawn();

                        //let (window_entity, mut test) = window_query.single_mut();

                        // Create the second window
                        let second_window = commands.spawn
                        (
                            Window
                            {
                                title: "Simulation".to_owned(),
                                ..default()
                            }
                        ).id();

                        // Create a camera for the new window
                        commands.spawn
                        (
                            Camera3dBundle
                            {
                                transform: Transform::from_translation(Vec3::new(0., 0., 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
                                camera: Camera
                                {
                                    target: RenderTarget::Window(WindowRef::Entity(second_window)),

                                    // Change the background color of the window
                                    clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.2, 0.6274509)),

                                    ..default()
                                },

                                ..default()
                            }
                        );

                        // Switch app states to start the simulation
                        next_state.set(AppState::Simulate);
                    }

                    MenuAction::LoadFile =>
                    {
                        // If the load file button was pressed, allow the user to select a simulation file to start.
                    }

                    MenuAction::SelectFolder =>
                    {
                        // If the select folder button was pressed, allow the user to select a folder to save simulation files.
                    }
                    
                    MenuAction::Quit =>
                    {
                        // If the quit button was pressed, close the window.
                        exit_app.send(AppExit);
                    }
                }
            }

            // Highlight the border of the button when the mouse is hovering over it
            Interaction::Hovered =>
            {
                border_color.0 = Color::LIME_GREEN;
            }

            // Set the border color of the button to a default color when no mouse is over it
            Interaction::None =>
            {
                border_color.0 = Color::WHITE;
            }
        }
    }
}

// This function handles setting up the main menu window and other components.
fn setup(mut commands: Commands)
{

    // Spawn in a text field for the title
    commands.spawn
    (
        (
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section
            (

                // The string to be displayed.
                "Tectonic Plates Simulation",

                // Sets the properties of the text.
                TextStyle
                {
                    // Set the font of the text to default
                    font: default(),

                    // Set the font size of the text
                    font_size: 75.0,
                    
                    ..default()
                },
            )

            // Set the justification of the Text to be centered
            .with_text_justify(JustifyText::Center)

            // Set the style of the TextBundle itself.
            .with_style
            (
                Style
                {
                    // Set whether the node should consider its siblings when trying to position itself.
                    // When set to absolute, the node will place itself over sibling nodes.
                    position_type: PositionType::Absolute,

                    // Set the number of pixels above the text field
                    top: Val::Px(10.0),

                    // Set the width of the text field to 50% of the width of the screen
                    min_width: Val::Percent(50.0),

                    // Horizontally align the text field
                    justify_self: JustifySelf::Center,

                    ..default()
                }
            ),

            ColorText,
        )
    );

    // Spawn menu buttons
    commands.spawn
    (
        // Create the node that contains the buttons
        NodeBundle
        {
            style: Style
            {
                // Set the ideal width of the buttons column in pixels
                width: Val::Px(300.0),

                // Horizontally align buttons node
                align_items: AlignItems::Start,

                // Vertically align buttons node
                justify_content: JustifyContent::End,

                // Align the buttons in a column
                flex_direction: FlexDirection::Column,

                // Add a 10 pixel gap between buttons
                row_gap: Val::Px(10.0),

                // Set a 20 pixel gap between the buttons column and the left of the window
                left: Val::Px(20.0),

                // Set a 350 pixel gap between the top of the buttons column and the top of the window
                top: Val::Px(350.0),

                ..default()
            },

            ..default()
        }
    )

    .with_children
    (
        |parent|
        {
            parent.spawn
            (
                (
                    // Create the start button within the node
                    ButtonBundle
                    {
                        style: Style
                        {
                            // Width of the textbox of the button in pixels
                            width: Val::Px(300.0),

                            // Height of the textbox of the button in pixels
                            height: Val::Px(65.0),

                            // Add the border to the button and set the thickness in pixels
                            border: UiRect::all(Val::Px(3.0)),

                            // Horizontally align child text
                            justify_content: JustifyContent::Center,

                            // Vertically align child text
                            align_items: AlignItems::Center,

                            ..default()
                        },

                        // Set the border color of the button
                        border_color: BorderColor(Color::WHITE),

                        // Set the background color of the button
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),

                        ..default()
                    },

                    MenuAction::Play,
                )
            )

            .with_children
            (
                |parent|
                {
                    parent.spawn
                    (
                        // Create the text within the button
                        TextBundle::from_section
                        (
                            // Set the text of the button
                            "Start Simulation",

                            // Set the style of the text of the button
                            TextStyle
                            {
                                // Set the font of the text to default
                                font: default(),

                                // Set the font size of the text
                                font_size: 30.0,

                                // Set the color of the text
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                    );
                }
            );

            parent.spawn
            (
                (
                    // Create the load file button within the node
                    ButtonBundle
                    {
                        style: Style
                        {
                            // Width of the textbox of the button in pixels
                            width: Val::Px(300.0),

                            // Height of the textbox of the button in pixels
                            height: Val::Px(65.0),

                            // Add the border to the button and set the thickness in pixels
                            border: UiRect::all(Val::Px(3.0)),

                            // Horizontally center child text
                            justify_content: JustifyContent::Center,

                            // Vertically center child text
                            align_items: AlignItems::Center,

                            ..default()
                        },

                        // Set the border color of the button
                        border_color: BorderColor(Color::WHITE),

                        // Set the background color of the button
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),

                        ..default()
                    },

                    MenuAction::LoadFile,
                )
            )

            .with_children
            (
                |parent|
                {
                    parent.spawn
                    (
                        // Create the text within the button
                        TextBundle::from_section
                        (
                            // Set the text of the button
                            "Load Simulation",

                            // Set the style of the text of the button
                            TextStyle
                            {
                                // Set the font of the text to default
                                font: default(),

                                // Set the font size of the text
                                font_size: 30.0,

                                // Set the color of the text
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                    );
                }
            );

            parent.spawn
            (
                (
                    // Create the folder selection button within the node
                    // The button lets the user select what folder to save simulations to.
                    ButtonBundle
                    {
                        style: Style
                        {
                            // Width of the textbox of the button in pixels
                            width: Val::Px(300.0),

                            // Height of the textbox of the button in pixels
                            height: Val::Px(65.0),

                            // Add the border to the button and set the thickness in pixels
                            border: UiRect::all(Val::Px(3.0)),

                            // Horizontally align child text
                            justify_content: JustifyContent::Center,

                            // Vertically align child text
                            align_items: AlignItems::Center,

                            ..default()
                        },

                        // Set the border color of the button
                        border_color: BorderColor(Color::WHITE),

                        // Set the background color of the button
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),

                        ..default()
                    },

                    MenuAction::SelectFolder,
                )
            )

            .with_children
            (
                |parent|
                {
                    parent.spawn
                    (
                        // Create the text within the button
                        TextBundle::from_section
                        (
                            // Set the text of the button
                            "Choose Save Folder",

                            // Set the style of the text of the button
                            TextStyle
                            {
                                // Set the font of the text to default
                                font: default(),

                                // Set the font size of the text
                                font_size: 30.0,

                                // Set the color of the text
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                    );
                }
            );

            parent.spawn
            (
                (
                    // Create the quit button within the node
                    ButtonBundle
                    {
                        style: Style
                        {
                            // Width of the textbox of the button in pixels
                            width: Val::Px(300.0),

                            // Height of the textbox of the button in pixels
                            height: Val::Px(65.0),

                            // Add the border to the button and set the thickness in pixels
                            border: UiRect::all(Val::Px(3.0)),

                            // Horizontally align child text
                            justify_content: JustifyContent::Center,

                            // Vertically align child text
                            align_items: AlignItems::Center,

                            ..default()
                        },

                        // Set the border color of the button
                        border_color: BorderColor(Color::WHITE),

                        // Set the background color of the button
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),

                        ..default()
                    },

                    MenuAction::Quit,
                )
            )

            .with_children
            (
                |parent|
                {
                    parent.spawn
                    (
                        // Create the text within the button
                        TextBundle::from_section
                        (
                            // Set the text of the button
                            "Quit",

                            // Set the style of the text of the button
                            TextStyle
                            {
                                // Set the font of the text to default
                                font: default(),

                                // Set the font size of the text
                                font_size: 30.0,

                                // Set the color of the text
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )
                    );
                }
            );
        }
    );
}

fn setup2(
	mut commands: Commands,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut meshes: ResMut<Assets<Mesh>>,
    //mut images: ResMut<Assets<Image>>,
    h: ResMut<HeightValues>,
    current_state: ResMut<State<AppState>>,
) {
    //not certain what this is doing, this is probably where we want to start doing visuals
    let debug_material = materials.add(StandardMaterial {
        ..default()
    });


    //let mut heights = &h.values;
    //let mut heights :std::vec::Vec<Vec<Vec<f32>>> = Vec::<Vec<Vec<f32>>>::new();
    //this is the call to create the mesh, and where we create what i think is basically a pointer to it
    let globe_mesh_handle: Handle<Mesh> = meshes.add(create_globe_rect_mesh(100, 100, &mut h.into_inner().values));

    let world_pos: [f32; 3];

    match current_state.get()
    {
        AppState::MainMenu =>
        {
            world_pos = [3., -1., 0.];
        }
        
        AppState::Simulate =>
        {
            world_pos = [0., 0., 0.];
        }
    }
    //loads mesh into scene
	commands.spawn((
        PbrBundle {
		    mesh: globe_mesh_handle,
		    material: debug_material.clone(),
            transform: Transform::from_xyz(world_pos[0], world_pos[1], world_pos[2]),
		    ..Default::default()
        },
        Shape,
	));
    //load light source into scene
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 2., 8.0),
        ..default()
    });

    //load camera into scene
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_translation(Vec3::new(0., 0., 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..Default::default()
    // });

}

//lets you spin the mesh with X/Y/Z keys
fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mesh_query: Query<&Handle<Mesh>, With<Shape>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Transform, With<Shape>>,
    h: ResMut<HeightValues>,
    time: Res<Time>,
) {
    
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_seconds() / 1.2);
        }
    }
    //if keyboard_input.pressed(KeyCode::KeyY) {
    //    for mut transform in &mut query {
    //        transform.rotate_y(time.delta_seconds() / 1.2);
    //    }
    //}
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_seconds() / 1.2);
        }
    }

    let heights = &mut h.into_inner().values;
    let mut changed: bool = false;
    if keyboard_input.just_pressed(KeyCode::ArrowUp){
        changed = true;
        for i in 0..heights.len(){
            for j in 0..heights[i].len(){
                heights[i][j] = heights[i][j] * 1.1;
		    }
	    }
	}

    if keyboard_input.just_pressed(KeyCode::ArrowDown){
        changed = true;
        for i in 0..heights.len(){
            for j in 0..heights[i].len(){
			    heights[i][j] = heights[i][j] * 1.1;
		    }
	    }
	}

    if changed {
        for mesh in &mut mesh_query{
            let mesh_mut = meshes.get_mut(mesh);
            mesh_mut.unwrap().insert_attribute(Mesh::ATTRIBUTE_POSITION, tris_from_rect_heights(heights));
        }
    }
				//vs[i][0] = vs[i][0] * (1. + 0.25 * time.delta_seconds().cos());
			
        
    

}

//creates a mesh of a globe with a rectangular grid
fn create_globe_rect_mesh(h_verts: u32, v_verts: u32, heights: &mut Vec<Vec<f32>>) -> Mesh {
    for _row_index in 0..v_verts{ //represents which row we are in
		let mut row_vec = vec![];
		for _col_index in 0..h_verts{		
			row_vec.push(1.);
            //println!("height at row {} and col {}", _row_index, _col_index);
		}
		heights.push(row_vec);
	}

    let verts = tris_from_rect_heights(heights);
    let mut norms = Vec::new();

    for i in 0..verts.len(){
		norms.push(verts[i]);
        //println!("vertex at {}: {}, {}, {}", i, verts[i][0], verts[i][1], verts[i][2]);
	}


    let mut indices_by_tri = Vec::new();

    for i in 0..h_verts{ //top row only has 1 tri each
        indices_by_tri.push(((1 + i)%h_verts) + 1);
        indices_by_tri.push(i + 1);
        indices_by_tri.push(0);
        //println!("linking vertices: {}, {}, {}", ((1 + i)%h_verts) + 1, i + 1, 0);
    }

    for i in 0..(v_verts-3){
		for j in 0..h_verts{
            indices_by_tri.push((i+1)*h_verts + ((j+1)%h_verts) + 1);
            indices_by_tri.push((i+1)*h_verts + j + 1);
			indices_by_tri.push(i*h_verts + j + 1);
            //println!("linking vertices: {}, {}, {}", (i+1)*h_verts + ((j+1)%h_verts) + 1, (i+1)*h_verts + j + 1, i*h_verts + j + 1);

			indices_by_tri.push(i*h_verts + ((j+1)%h_verts) + 1);
			indices_by_tri.push((i+1)*h_verts + ((j+1)%h_verts) + 1);
			indices_by_tri.push(i*h_verts + j + 1);
            //println!("linking vertices: {}, {}, {}", i*h_verts + ((j+1)%h_verts) + 1, (i+1)*h_verts + ((j+1)%h_verts) + 1, i*h_verts + j + 1);
		}
	}

    for i in 0..h_verts{//bottom row only has 1 tri each
		indices_by_tri.push(h_verts*(v_verts-3) + ((1 + i)%h_verts) + 1); 
		indices_by_tri.push(h_verts*(v_verts-2) + 1); 
		indices_by_tri.push(h_verts*(v_verts-3) + i + 1); 
        //println!("linking vertices: {}, {}, {}", h_verts*(v_verts-2) + ((1 + i)%h_verts) + 1, h_verts*(v_verts-2) + i + 1, h_verts*(v_verts-1) + 1);
	}

    

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        verts.clone(),
    )

    //put normals into mesh
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        norms,
    )
    
    //tell mesh which verts are connected
    .with_inserted_indices(Indices::U32(indices_by_tri))
}


fn tris_from_rect_heights(heights: &mut Vec<Vec<f32>>) -> Vec<[f32; 3]>{
    let mut verts: Vec<[f32; 3]> = Vec::new();

    verts.push([0., heights[0][0], 0.]); //index 0
     
    for i in 1..(heights.len()-1){ //i is which row, y coord. indeces 1 to h_verts * (v_verts - 2)
        let v_val: f32 = (i as f32)/(heights.len() as f32 - 1.);
        //println!("Adding row {}", i);
		for j in 0..heights[i].len(){ //j is which column, x coord
            let h_angle: f32 = 2. * std::f32::consts::PI * (j as f32)/(heights[i].len() as f32);
			verts.push([heights[i][j] * h_angle.cos() * (0.25 - (v_val-0.5) * (v_val-0.5)).sqrt() * 2., heights[i][j] * (1. - 2. * v_val), heights[i][j] * h_angle.sin() * (0.25 - (v_val-0.5) * (v_val-0.5)).sqrt() * 2.]);
            //println!("adding vert with coords ({}, {}, {})", verts[verts.len()-1][0], verts[verts.len()-1][1], verts[verts.len()-1][2]);
		}
	}

    verts.push([0., -heights[heights.len()-1][0], 0.]); // index h_verts * (v_verts - 1) + 1

    //format of tris:
    //[row 0 vert 0, row1 vert 0, ... row1 vert h_verts-1, row2 vert 0 ... row v_verts-2 vert h_verts-1, row v_verts-1 vert 0]

    return verts;
}

    //Erosion code

    //returns the height of the water, bed sediment, and bedrock. The suspended sediment is not included in this calculation because it is dispereced throughout the water
    fn totalHeight (x : i32, y : i32) -> f32 {
        return h[x][y].water + h[x][y].bed_sediment + h[x][y].bedrock;
    }

    //returns the total volume of water flowing from the tile
    fn totalOutflowVolume (x: i32, y : i32) -> f32 {
        return h[x][y].flow.n_volume + h[x][y].flow.e_volume + h[x][y].flow.s_volume + h[x][y].flow.w_volume;
    }

    //adds the amount of percipitation the tile experiences to the water level
    fn precipitation(x : i32, y : i32, percipitation: f32) {
        h[x][y].water += percipitation;
    }

    //evaluates the flow of water from the tile to its neighbors
    fn waterFlow (x : i32, y : i32, n_height: f32, e_height: f32, s_height: f32, w_height: f32) {
        let mut newFlow = h[x][y].flow.n_volume + h[x][y].flow.n_speed * h[x][y].totalHeight - n_height;
        if (newFlow > 0) {
            flow.n_volume = newFlow;
        }
        else {
            flow.n_volume = 0;
        }
        newFlow = h[x][y].flow.e_volume + h[x][y].flow.e_speed * h[x][y].totalHeight - e_height;
        if (newFlow > 0) {
            flow.e_volume = newFlow;
        }
        else {
            flow.e_volume = 0;
        }
        newFlow = h[x][y].flow.s_volume + h[x][y].flow.s_speed * h[x][y].totalHeight - s_height;
        if (newFlow > 0) {
            flow.s_volume = newFlow;
        }
        else {
            flow.s_volume = 0;
        }
        newFlow = h[x][y].flow.w_volume + h[x][y].flow.w_speed * h[x][y].totalHeight - w_height;
        if (newFlow > 0) {
            flow.w_volume = newFlow;
        }
        else {
            flow.w_volume = 0;
        }
    }

    //calculates the scaler value for the flowScaling function
    fn scaler (x : i32, y : i32) -> f32 {
        let newScaler = h[x][y].water / (h[x][y].flow.n_volume + h[x][y].flow.e_volume + h[x][y].flow.s_volume + h[x][y].flow.w_volume);
        if (newScaler > 1) {
            h[x][y].scaler = 1; 
        } else {
            h[x][y].scaler = newScaler;
        }
    }

    //makes sure that the amount of water flowing out of the tile is proportional to the amount of water flowing in
    fn flowScaling (x : i32, y : i32) {
        let scaler = h[x][y].scaler();
        flow.n_volume = scaler * h[x][y].flow.n_volume;
        flow.e_volume = scaler * h[x][y].flow.e_volume;
        flow.s_volume = scaler * h[x][y].flow.s_volume;
        flow.w_volume = scaler * h[x][y].flow.w_volume;
    }

    //changes the water level of the tile based on the flow of water to and from its neighbors
    fn waterLevelUpdate (x : i32, y : i32, north: f32, east: f32, south: f32, west: f32) {
        h[x][y].water = h[x][y].water + north + east + south + west - h[x][y].flow.n_volume - h[x][y].flow.e_volume - h[x][y].flow.s_volume - h[x][y].flow.w_volume;
    }

    //average speed of the water flowing in and out of the tile
    fn waterSpeed (x : i32, y : i32, north: f32, east: f32, south: f32, west: f32) -> f32{
        return (h[x][y].flowspeed = h[x][y].flowspeed + north + east + south + west - h[x][y].flow.n_speed - h[x][y].flow.e_speed - h[x][y].flow.s_speed - h[x][y].flow.w_speed) / 2;
    }

    //finds the slope of the tile based on the height of the tile and its neighbors
    fn slope (x : i32, y : i32, north: f32, east: f32, south: f32, west: f32) -> f32 {
        let slope = h[x][y].getTotalHeight - north;

        if (slope < h[x][y].getTotalHeight - east) {
            slope = h[x][y].getTotalHeight - east;
        }

        if (slope < h[x][y].getTotalHeight - south) {
            slope = h[x][y].getTotalHeight - south;
        }

        if (slope < h[x][y].getTotalHeight - west) {
            slope = h[x][y].getTotalHeight - west;
        }
        return slope;
    }

    //represents how much power the water has to erode sediment, higher values should result in more erosion and dissolution and less deposition
    fn power (x : i32, y : i32, waterSpeed: f32, slope: f32) -> f32 {
        return waterSpeed.sqrtf() * slope;
    }

    //the amount of sediment the water can carry
    fn capacity (x : i32, y : i32, waterSpeed: f32, slope: f32) -> f32{
        return 1 * waterSpeed.sqrtf() * slope;
    }

    //deposit suspended sediment into the bed sediment
    fn depositSediment (x : i32, y : i32, pScaler: f32) {
        let transferredSediment = (h[x][y].suspended_sediment - h[x][y].capacity) * (1 / h[x][y].power.sqrtf() * pScaler);
        h[x][y].bed_sediment += transferredSediment;
        h[x][y].suspended_sediment -= transferredSediment;
    }

    //erode bed sediment into the suspended sediment
    fn disolveSediment (x : i32, y : i32, pScaler: f32){
        let transferredSediment = (h[x][y].capacity - h[x][y].suspended_sediment) * (h[x][y].power.sqrtf() * pScaler);
        h[x][y].suspended_sediment += transferredSediment;
        h[x][y].bed_sediment -= transferredSediment;
    }

    //erode bedrock into the suspended sediment
    fn erodeBedrock (x : i32, y : i32, target: f32) {
        let transferredSediment = (h[x][y].capacity - h[x][y].suspended_sediment) * (h[x][y].power.sqrtf() * pScaler);
        h[x][y].suspended_sediment += transferredSediment;
        h[x][y].bedrock -= transferredSediment;
    }

    //handles the erosion, deposition, and dissolution of sediment
    fn erosion (x : i32, y : i32, target: f32) {
        //deposition
        if (h[x][y].suspended_sediment > h[x][y].capacity * target) {
            depositSediment(1);
        }
        //erosion (if there insufficient bed sediment)
        else if (h[x][y].bed_sediment < capacity * .05;) {
            erodeSediment(1);
        }
        //dissolution
        else {
            disolveSediment(1);
        }
    }

    //TODO: implement this function
    fn sedimentTransfer(x : i32, y : i32, north: tile, east: tile, south: tile, west: tile) {
        /*
        waterLevelUpdate = h[x][y].waterLevelUpdate();
        totalFlowVolume = h[x][y].totalFlowVolume();
        water = h[x][y].water;
        suspended_sediment = h[x][y].suspended_sediment;
        h[x][y].suspended_sediment = suspended_sediment + north.waterLevelUpdate / north.water * north.flow.s_volume / north.getTotalFlowVolume * north.suspended_sediment -
        waterLevelUpdate / water * h[x][y].flow.n_volume / totalFlowVolume * suspended_sediment +
        east.waterLevelUpdate / east.water * east.flow.w_volume / east.getTotalFlowVolume * east.suspended_sediment -
        waterLevelUpdate / water * h[x][y].flow.e_volume / totalFlowVolume * suspended_sediment +
        south.waterLevelUpdate / south.water * south.flow.n_volume / south.getTotalFlowVolume * south.suspended_sediment -
        waterLevelUpdate / water * h[x][y].flow.s_volume / totalFlowVolume * suspended_sediment +
        west.waterLevelUpdate / west.water * west.flow.e_volume / west.getTotalFlowVolume * west.suspended_sediment -
        waterLevelUpdate / water * h[x][y].flow.w_volume / totalFlowVolume * suspended_sediment;
        */
    }

    //finds the tile directly north of the given tile coordinates, if the tile is at the north pole, it will return the tile opposite the pole.
    fn getNorth (x : i32, y : i32, worldHeight : i32, worldWidth : i32) -> tile {
        if (y == worldHeight - 1) {
            return h[x+(worldWidth/2)%worldWidth][y];
        }
        else {
            return h[x][y + 1];
        }
    }

    fn getEast (x : i32, y : i32, worldWidth : i32) -> tile {
        return h[(x + 1)%worldWidth][y];
    }

    fn getSouth (x : i32, y : i32, worldHeight : i32, worldWidth : i32) -> tile {
        if (y == 0) {
            return h[x+(worldWidth/2)%worldWidth][y];
        }
        else {
            return h[x][y - 1];
        }
    }

    fn getWest (x : i32, y : i32, worldWidth : i32) -> tile {
        return h[(x - 1)%worldWidth][y];
    }

    //preforms all the functions needed to update the tile for one time step
    fn step(x : i32, y : i32, north: tile, east: tile, south: tile, west: tile) {
        h[x][y].precipitation();
        h[x][y].waterFlow(north.height, east.height, south.height, west.height);
        h[x][y].scaler();
        h[x][y].flowScaling();
        h[x][y].waterLevelUpdate(north.flow.s_volume, east.flow.w_volume, south.flow.n_volume, west.flow.e_volume);
        waterSpeed = h[x][y].waterSpeed(north.flow.s_speed, east.flow.w_speed, south.flow.n_speed, west.flow.e_speed);
        slope = h[x][y].slope(north.height, east.height, south.height, west.height);
        h[x][y].power(slope, waterSpeed);
        h[x][y].capacity(slope, waterSpeed);
        h[x][y].erosion(.05);
    }


//#[rustfmt::skip]
//fn create_globe_ico_mesh(subdivisions: u32, heights: &mut Vec<Vec<Vec<f32>>>,) -> Mesh {
//    let res = 2_i32.pow(subdivisions); //short for resolution, this is just an important value used many places
//    //vertex positions vector calculation:
//    let mut i = 0;
//    while i<4{
//        let mut j = 0;
//        let mut row_vec = vec![];
//        while j<5{
//			let mut v = 0;
//            let mut col_vec = vec![];
//            while v<(res+1)*(res+2)/2 {
//                col_vec.push(1.);
//                v+=1;
//            }
//            row_vec.push(col_vec);
//			j+=1;
//		}
//        heights.push(row_vec);
//        i +=1;
//    }
//
//    //println!("Heights: {:?}", heights);
//    
//    
//
//
//
//    let verts = tris_from_ico_heights(heights);
//    
//    let mut norms = Vec::new();//create vector to hold calculated normals
//
//    i = 0;
//    while i < verts.len(){//TODO: neither of these approaches will work once verts have variable heights
//        //if you want to use the other type of normal comment out all code relevant to the current type
//        //unsmoothed normals
//        
//        let x = (verts[i][0] + verts[i+1][0] + verts[i+2][0])/3.;
//        let y = (verts[i][1] + verts[i+1][1] + verts[i+2][1])/3.;
//        let z = (verts[i][2] + verts[i+1][2] + verts[i+2][2])/3.;
//        let d1 = (x*x+y*y+z*z).sqrt();
//        norms.push([x/d1, y/d1, z/d1]);
//        norms.push([x/d1, y/d1, z/d1]);
//        norms.push([x/d1, y/d1, z/d1]);
//        i = i+3;
//        
//        //smoothed normals
//        //norms.push([verts[i][0]/d, verts[i][1]/d, verts[i][2]/d]);
//        //i=i+1;
//    }
//
//    //put verts into mesh
//    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
//    .with_inserted_attribute(
//        Mesh::ATTRIBUTE_POSITION,
//        // Each array is an [x, y, z] coordinate in local space.
//        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
//        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
//        verts.clone(),
//    )
//
//    //put normals into mesh
//    .with_inserted_attribute(
//        Mesh::ATTRIBUTE_NORMAL,
//        norms,
//    )
//    
//    //tell mesh which verts are connected
//    .with_inserted_indices(Indices::U32((0u32..(verts.len() as u32)).collect::<Vec<_>>()))
//}
//
//fn tris_from_ico_heights(heights: &mut Vec<Vec<Vec<f32>>>) -> Vec<[f32; 3]>{
//
//    //useful numbers: p represents phi, d is the magnitude of vert on a major triangle 
//    let p: f32 = (1.0 + (5.0_f32).sqrt()) / 2.0;
//    let d: f32 = (1.0 + p *p).sqrt();
//    let res: i32 = (((1 + 8*heights[0][0].len()) as f64).sqrt() as i32 - 3)/2;
//    //created to use later
//    let main_verts = vec![
//        [0., d, 0.],
//
//        [0., 0.85065, -1.7013],
//        [p, 0.85065, -(d*d - p * p - 0.85065 * 0.85065).sqrt()],
//		[1., 0.85065, (d*d - 1. - 0.85065*0.85065).sqrt()],
//		[-1., 0.85065, (d*d - 1. - 0.85065*0.85065).sqrt()],
//		[-p, 0.85065, -(d*d - p*p - 0.85065*0.85065).sqrt()],
//
//        [-1., -0.85065, -(d*d - 1. - 0.85065*0.85065).sqrt()],
//        [1., -0.85065, -(d*d - 1. - 0.85065*0.85065).sqrt()],
//        [p, -0.85065, (d*d - p*p - 0.85065*0.85065).sqrt()],
//        [0., -0.85065, 1.7013],
//        [-p, -0.85065, (d*d - p*p - 0.85065*0.85065).sqrt()],
//
//        [0., -d, 0.]
//    ];
//
//    let mut verts = Vec::new();
//    
//    let mut f = 0; //which face
//    while f<5 { //iterates over the 5 major triangles which share the topmost vert
//        let v0 = main_verts[0];//c vert
//        let v1 = main_verts[f+1];//b vert
//        let v2 = main_verts[((f+1)%5)+1];//a vert
//
//        let mut col_index = 0;
//        while col_index < res{
//            let mut row_index = 0;
//            while row_index <= col_index*2{
//                
//                let mut iter = 0;
//                let mut vert0 = [0., 0., 0.];//c vert
//                let mut vert1 = [0., 0., 0.];//a vert
//                let mut vert2 = [0., 0., 0.];//b vert
//
//                if row_index%2==0{
//                    while iter<3{
//                        vert0[iter] = ((res-col_index) as f32 * v0[iter] + (col_index - row_index/2) as f32 * v1[iter] + (row_index/2) as f32 * v2[iter])/res as f32;
//                        vert1[iter] = ((res-col_index - 1) as f32 * v0[iter] + (col_index + 1 - row_index/2) as f32 * v1[iter] + (row_index/2) as f32 * v2[iter])/res as f32;
//                        vert2[iter] = ((res-col_index - 1) as f32 * v0[iter] + (col_index - row_index/2) as f32 * v1[iter] + (row_index/2 + 1) as f32 * v2[iter])/res as f32;
//
//                        iter = iter+1;
//                    }
//                } else {
//                    while iter<3{
//                        vert0[iter] = ((res-col_index - 1) as f32 * v0[iter] + (col_index - (row_index-1)/2) as f32 * v1[iter] + ((row_index-1)/2 + 1) as f32 * v2[iter])/res as f32;
//                        vert1[iter] = ((res-col_index) as f32 * v0[iter] + (col_index - (row_index+1)/2) as f32 * v1[iter] + ((row_index+1)/2) as f32 * v2[iter])/res as f32;
//                        vert2[iter] = ((res-col_index) as f32 * v0[iter] + (col_index - (row_index-1)/2) as f32 * v1[iter] + ((row_index-1)/2) as f32 * v2[iter])/res as f32;
//                        
//                        iter = iter+1;
//                    }
//                }
//
//                let len0 = (vert0[0]*vert0[0] + vert0[1]*vert0[1] + vert0[2]*vert0[2]).sqrt();
//                let len1 = (vert1[0]*vert1[0] + vert1[1]*vert1[1] + vert1[2]*vert1[2]).sqrt();
//                let len2 = (vert2[0]*vert2[0] + vert2[1]*vert2[1] + vert2[2]*vert2[2]).sqrt();
//                
//                //order is weird, but important
//                //values correspond to:
//                //right vert
//                //vertical vert
//                //left vert
//                verts.push([vert1[0] * get_height(&heights, f, 1+row_index/2,   col_index-(row_index+1)/2)/len1,   vert1[1]*get_height(&heights, f, 1+row_index/2,   col_index-(row_index+1)/2)/len1,   vert1[2]*get_height(&heights, f, 1+row_index/2,   col_index-(row_index+1)/2)/len1]);
//                verts.push([vert0[0] * get_height(&heights, f, (row_index+1)/2, col_index-row_index/2)/len0,       vert0[1]*get_height(&heights, f, (row_index+1)/2, col_index-row_index/2)/len0,       vert0[2]*get_height(&heights, f, (row_index+1)/2, col_index-row_index/2)/len0]);
//                verts.push([vert2[0] * get_height(&heights, f, row_index/2,     col_index+1-(row_index+1)/2)/len2, vert2[1]*get_height(&heights, f, row_index/2,     col_index+1-(row_index+1)/2)/len2, vert2[2]*get_height(&heights, f, row_index/2,     col_index+1-(row_index+1)/2)/len2]);
//
//
//                row_index = row_index+1;
//            }
//            col_index=col_index+1;
//            
//        }
//
//
//        f = f+1;
//    }
//
//    let vert_order = [//which vertices to use for each major triangle not on the top or bottom
//        [6,1,7],
//        [7,1,2],
//        [7,2,8],
//		[8,2,3],
//		[8,3,9],
//		[9,3,4],
//		[9,4,10],
//		[10,4,5],
//		[10,5,6],
//		[6,5,1]
//    ];
//    f = 0;
//
//    while f< 10{ //iterate through the 10 major triangles that do not have the topmost or bottommost vert
//        let v0 = main_verts[vert_order[f][0]];//f%2==0: b vert, f%2==1: c vert
//        let v1 = main_verts[vert_order[f][1]];//f%2==0: c vert, f%2==1: b vert
//        let v2 = main_verts[vert_order[f][2]];//a vert
//
//        let mut col_index = 0;
//        while col_index < res{
//            let mut row_index = 0;
//            while row_index <= 2*col_index{
//                
//                let mut iter = 0;
//                let mut vert0 = [0., 0., 0.];
//                let mut vert1 = [0., 0., 0.];
//                let mut vert2 = [0., 0., 0.];
//
//                if row_index%2==0{
//                    while iter<3{
//                        vert0[iter] = ((res-col_index) as f32 * v0[iter] + (col_index - row_index/2) as f32 * v1[iter] + (row_index/2) as f32 * v2[iter])/res as f32;
//                        vert1[iter] = ((res-col_index - 1) as f32 * v0[iter] + (col_index + 1 - row_index/2) as f32 * v1[iter] + (row_index/2) as f32 * v2[iter])/res as f32;
//                        vert2[iter] = ((res-col_index - 1) as f32 * v0[iter] + (col_index - row_index/2) as f32 * v1[iter] + (row_index/2 + 1) as f32 * v2[iter])/res as f32;
//
//                        iter = iter+1;
//                    }
//
//                    
//                } else {
//                    while iter<3{
//                        vert0[iter] = ((res-col_index - 1) as f32 * v0[iter] + (col_index - (row_index-1)/2) as f32 * v1[iter] + ((row_index-1)/2 + 1) as f32 * v2[iter])/res as f32;
//                        vert1[iter] = ((res-col_index) as f32 * v0[iter] + (col_index - (row_index+1)/2) as f32 * v1[iter] + ((row_index+1)/2) as f32 * v2[iter])/res as f32;
//                        vert2[iter] = ((res-col_index) as f32 * v0[iter] + (col_index - (row_index-1)/2) as f32 * v1[iter] + ((row_index-1)/2) as f32 * v2[iter])/res as f32;
//                        
//                        iter = iter+1;
//                    }
//                }
//
//                let len0 = (vert0[0]*vert0[0] + vert0[1]*vert0[1] + vert0[2]*vert0[2]).sqrt();
//                let len1 = (vert1[0]*vert1[0] + vert1[1]*vert1[1] + vert1[2]*vert1[2]).sqrt();
//                let len2 = (vert2[0]*vert2[0] + vert2[1]*vert2[1] + vert2[2]*vert2[2]).sqrt();
//                
//                
//                //i am... surprised that this just works for both orientations of mega-tris
//                //but not complaining
//                //left corner
//                //vertical corner
//                //right corner
//                verts.push([vert0[0] * get_height(&heights, f+5, row_index/2,     col_index+1-(row_index+1)/2)/len0, vert0[1]*get_height(&heights, f+5, row_index/2,     col_index+1-(row_index+1)/2)/len0, vert0[2]*get_height(&heights, f+5, row_index/2,     col_index+1-(row_index+1)/2)/len0]);
//                verts.push([vert1[0] * get_height(&heights, f+5, (row_index+1)/2, col_index-row_index/2)/len1, vert1[1]*get_height(&heights, f+5, (row_index+1)/2, col_index-row_index/2)/len1, vert1[2]*get_height(&heights, f+5, (row_index+1)/2, col_index-row_index/2)/len1]);
//                verts.push([vert2[0] * get_height(&heights, f+5, 1+row_index/2,   col_index-(row_index+1)/2)/len2, vert2[1]*get_height(&heights, f+5, 1+row_index/2,   col_index-(row_index+1)/2)/len2, vert2[2]*get_height(&heights, f+5, 1+row_index/2,   col_index-(row_index+1)/2)/len2]);
//                
//
//                row_index = row_index+1;
//            }
//            col_index=col_index+1;
//            
//        }
//
//
//        f = f+1;
//    }
//    
//    f = 0;
//    while f<5 {//iterate over 5 major tris that share the bottommost vert
//        let v0 = main_verts[11];//c vert
//        let v1 = main_verts[10-f];//a vert
//        let v2 = main_verts[10-((f+1)%5)];//b vert
//
//        let mut col_index = 0;
//        while col_index < res{
//            let mut row_index = 0;
//            while row_index <= 2*col_index{
//                
//                let mut iter = 0;
//                let mut vert0 = [0., 0., 0.];
//                let mut vert1 = [0., 0., 0.];
//                let mut vert2 = [0., 0., 0.];
//
//                if row_index%2==0{
//                    while iter<3{
//                        vert0[iter] = ((res-col_index) as f32 * v0[iter] + (col_index - row_index/2) as f32 * v1[iter] + (row_index/2) as f32 * v2[iter])/res as f32;
//                        vert1[iter] = ((res-col_index - 1) as f32 * v0[iter] + (col_index + 1 - row_index/2) as f32 * v1[iter] + (row_index/2) as f32 * v2[iter])/res as f32;
//                        vert2[iter] = ((res-col_index - 1) as f32 * v0[iter] + (col_index - row_index/2) as f32 * v1[iter] + (row_index/2 + 1) as f32 * v2[iter])/res as f32;
//
//                        iter = iter+1;
//                    }
//
//                    
//                } else {
//                    while iter<3{
//                        vert0[iter] = ((res-col_index - 1) as f32 * v0[iter] + (col_index - (row_index-1)/2) as f32 * v1[iter] + ((row_index-1)/2 + 1) as f32 * v2[iter])/res as f32;
//                        vert1[iter] = ((res-col_index) as f32 * v0[iter] + (col_index - (row_index+1)/2) as f32 * v1[iter] + ((row_index+1)/2) as f32 * v2[iter])/res as f32;
//                        vert2[iter] = ((res-col_index) as f32 * v0[iter] + (col_index - (row_index-1)/2) as f32 * v1[iter] + ((row_index-1)/2) as f32 * v2[iter])/res as f32;
//                        
//                        iter = iter+1;
//                    }
//                }
//
//                let len0 = (vert0[0]*vert0[0] + vert0[1]*vert0[1] + vert0[2]*vert0[2]).sqrt();
//                let len1 = (vert1[0]*vert1[0] + vert1[1]*vert1[1] + vert1[2]*vert1[2]).sqrt();
//                let len2 = (vert2[0]*vert2[0] + vert2[1]*vert2[1] + vert2[2]*vert2[2]).sqrt();
//                
//                //order is weird, but important
//                verts.push([vert1[0] * get_height(&heights, f+15, row_index/2,     col_index+1-(row_index+1)/2)/len1,   vert1[1]*get_height(&heights, f+15, row_index/2,     col_index+1-(row_index+1)/2)/len1, vert1[2]*get_height(&heights, f+15, row_index/2,     col_index+1-(row_index+1)/2)/len1]);
//                verts.push([vert0[0] * get_height(&heights, f+15, (row_index+1)/2, col_index-row_index/2)/len0,         vert0[1]*get_height(&heights, f+15, (row_index+1)/2, col_index-row_index/2)/len0, vert0[2]*get_height(&heights, f+15, (row_index+1)/2, col_index-row_index/2)/len0]);
//                verts.push([vert2[0] * get_height(&heights, f+15, 1+row_index/2,   col_index-(row_index+1)/2)/len2,     vert2[1]*get_height(&heights, f+15, 1+row_index/2,   col_index-(row_index+1)/2)/len2, vert2[2]*get_height(&heights, f+15, 1+row_index/2,   col_index-(row_index+1)/2)/len2]);
//
//
//                row_index = row_index+1;
//            }
//            col_index=col_index+1;
//            
//        }
//
//
//        f = f+1;
//    }
//    return verts;
//}
//
//fn get_height(heights: &Vec<Vec<Vec<f32>>>, major_tri: usize, a: i32, b: i32) -> f32{
//    //magic formula !!! who knows why it works but it does!!! i could probably re-derive it but please dont make me
//    return heights[(major_tri/5) as usize][(major_tri%5) as usize][(a+(b+a)*(b+a+1)/2) as usize];
//}