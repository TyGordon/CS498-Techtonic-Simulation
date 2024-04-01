// Libraries/Crates/Packages
use bevy::
{
    app::AppExit, prelude::*,
    render::
    {
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::{PrimitiveTopology, Extent3d, TextureDimension, TextureFormat},
    },
};

fn main()
{
    // Start the main menu app
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup2))
        .add_systems(Update, (button_system, input_handler))
        .run();
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
                        // If the start button was pressed, start the simulation.
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
    // Create the UI window (camera)
    commands.spawn
    (
        Camera3dBundle
        {
            transform: Transform::from_translation(Vec3::new(0., 0., 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera
            {
                // Change the background color of the window
                clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.2, 0.6274509)),

                ..default()
            },

            ..default()
        }
    );

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
    mut images: ResMut<Assets<Image>>,
) {
    //not certain what this is doing, this is probably where we want to start doing visuals
    let debug_material = materials.add(StandardMaterial {
        ..default()
    });

    //this is the call to create the mesh, and where we create what i think is rust's memory safe equivalent of a pointer to it
    let globe_mesh_handle: Handle<Mesh> = meshes.add(create_globe_mesh(3));

    let world_pos = [3.,-1.,0.];
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
    mesh_query: Query<&Handle<Mesh>, With<Shape>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Transform, With<Shape>>,
    time: Res<Time>,
) {
    
    if keyboard_input.pressed(KeyCode::KeyX) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_seconds() / 1.2);
        }
    }
    //if keyboard_input.pressed(KeyCode::KeyY) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_seconds() / 1.2);
        }
    //}
    if keyboard_input.pressed(KeyCode::KeyZ) {
        for mut transform in &mut query {
            transform.rotate_z(time.delta_seconds() / 1.2);
        }
    }
}


#[rustfmt::skip]
fn create_globe_mesh(subdivisions: u32) -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.

    //vertex positions vector calculation:

    
    //useful numbers: p represents phi, d is the magnitude of vert on a major triangle 
    let p: f32 = (1.0 + (5.0_f32).sqrt()) / 2.0;
    let d: f32 = (1.0 + p *p).sqrt();

    //created to use later
    let mut mainVerts = vec![
        [0., d, 0.],

        [0., 0.85065, -1.7013],
        [p, 0.85065, -(d*d - p * p - 0.85065 * 0.85065).sqrt()],
		[1., 0.85065, (d*d - 1. - 0.85065*0.85065).sqrt()],
		[-1., 0.85065, (d*d - 1. - 0.85065*0.85065).sqrt()],
		[-p, 0.85065, -(d*d - p*p - 0.85065*0.85065).sqrt()],

        [-1., -0.85065, -(d*d - 1. - 0.85065*0.85065).sqrt()],
        [1., -0.85065, -(d*d - 1. - 0.85065*0.85065).sqrt()],
        [p, -0.85065, (d*d - p*p - 0.85065*0.85065).sqrt()],
        [0., -0.85065, 1.7013],
        [-p, -0.85065, (d*d - p*p - 0.85065*0.85065).sqrt()],

        [0., -d, 0.]
    ];

    //create vector of coordinates that we will iteratively add each major triangle to, and then finally we will set this as the vertex positions
    let mut verts = Vec::new();
    let res = 2_i32.pow(subdivisions); //short for resolution, this is just an important value used many places
    let mut f = 0; //which face
    while f<5 { //iterates over the 5 major triangles which share the topmost vert
        let v0 = mainVerts[0];
        let v1 = mainVerts[f+1];
        let v2 = mainVerts[((f+1)%5)+1];

        let mut colIndex = 0;
        while colIndex < res{
            let mut rowIndex = 0;
            while rowIndex < (colIndex+1).pow(2) - (colIndex).pow(2){
                
                let mut iter = 0;
                let mut vert0 = [0., 0., 0.];
                let mut vert1 = [0., 0., 0.];
                let mut vert2 = [0., 0., 0.];

                if rowIndex%2==0{
                    while iter<3{
                        vert0[iter] = ((2_i32.pow(subdivisions)-colIndex) as f32 * v0[iter] + (colIndex - rowIndex/2) as f32 * v1[iter] + (rowIndex/2) as f32 * v2[iter])/res as f32;
                        vert1[iter] = ((2_i32.pow(subdivisions)-colIndex - 1) as f32 * v0[iter] + (colIndex + 1 - rowIndex/2) as f32 * v1[iter] + (rowIndex/2) as f32 * v2[iter])/res as f32;
                        vert2[iter] = ((2_i32.pow(subdivisions)-colIndex - 1) as f32 * v0[iter] + (colIndex - rowIndex/2) as f32 * v1[iter] + (rowIndex/2 + 1) as f32 * v2[iter])/res as f32;

                        iter = iter+1;
                    }

                    
                } else {
                    while iter<3{
                        vert0[iter] = ((2_i32.pow(subdivisions)-colIndex - 1) as f32 * v0[iter] + (colIndex - (rowIndex-1)/2) as f32 * v1[iter] + ((rowIndex-1)/2 + 1) as f32 * v2[iter])/res as f32;
                        vert1[iter] = ((2_i32.pow(subdivisions)-colIndex) as f32 * v0[iter] + (colIndex - (rowIndex+1)/2) as f32 * v1[iter] + ((rowIndex+1)/2) as f32 * v2[iter])/res as f32;
                        vert2[iter] = ((2_i32.pow(subdivisions)-colIndex) as f32 * v0[iter] + (colIndex - (rowIndex-1)/2) as f32 * v1[iter] + ((rowIndex-1)/2) as f32 * v2[iter])/res as f32;
                        
                        iter = iter+1;
                    }
                }

                let len0 = (vert0[0]*vert0[0] + vert0[1]*vert0[1] + vert0[2]*vert0[2]).sqrt();
                let len1 = (vert1[0]*vert1[0] + vert1[1]*vert1[1] + vert1[2]*vert1[2]).sqrt();
                let len2 = (vert2[0]*vert2[0] + vert2[1]*vert2[1] + vert2[2]*vert2[2]).sqrt();
                
                //order is weird but important, not certain why
                verts.push([vert1[0] * d/len1, vert1[1]*d/len1, vert1[2]*d/len1]);
                verts.push([vert0[0] * d/len0, vert0[1]*d/len0, vert0[2]*d/len0]);
                verts.push([vert2[0] * d/len2, vert2[1]*d/len2, vert2[2]*d/len2]);


                rowIndex = rowIndex+1;
            }
            colIndex=colIndex+1;
            
        }


        f = f+1;
    }

    let vertOrder = [//which vertices to use for each major triangle not on the top or bottom
        [6,1,7],
        [7,1,2],
        [7,2,8],
		[8,2,3],
		[8,3,9],
		[9,3,4],
		[9,4,10],
		[10,4,5],
		[10,5,6],
		[6,5,1]
    ];
    f = 0;

    while f<10{ //iterate through the 10 major triangles that do not have the topmost or bottommost vert
        let v0 = mainVerts[vertOrder[f][0]];
        let v1 = mainVerts[vertOrder[f][1]];
        let v2 = mainVerts[vertOrder[f][2]];

        let mut colIndex = 0;
        while colIndex < res{
            let mut rowIndex = 0;
            while rowIndex < (colIndex+1).pow(2) - (colIndex).pow(2){
                
                let mut iter = 0;
                let mut vert0 = [0., 0., 0.];
                let mut vert1 = [0., 0., 0.];
                let mut vert2 = [0., 0., 0.];

                if rowIndex%2==0{
                    while iter<3{
                        vert0[iter] = ((2_i32.pow(subdivisions)-colIndex) as f32 * v0[iter] + (colIndex - rowIndex/2) as f32 * v1[iter] + (rowIndex/2) as f32 * v2[iter])/res as f32;
                        vert1[iter] = ((2_i32.pow(subdivisions)-colIndex - 1) as f32 * v0[iter] + (colIndex + 1 - rowIndex/2) as f32 * v1[iter] + (rowIndex/2) as f32 * v2[iter])/res as f32;
                        vert2[iter] = ((2_i32.pow(subdivisions)-colIndex - 1) as f32 * v0[iter] + (colIndex - rowIndex/2) as f32 * v1[iter] + (rowIndex/2 + 1) as f32 * v2[iter])/res as f32;

                        iter = iter+1;
                    }

                    
                } else {
                    while iter<3{
                        vert0[iter] = ((2_i32.pow(subdivisions)-colIndex - 1) as f32 * v0[iter] + (colIndex - (rowIndex-1)/2) as f32 * v1[iter] + ((rowIndex-1)/2 + 1) as f32 * v2[iter])/res as f32;
                        vert1[iter] = ((2_i32.pow(subdivisions)-colIndex) as f32 * v0[iter] + (colIndex - (rowIndex+1)/2) as f32 * v1[iter] + ((rowIndex+1)/2) as f32 * v2[iter])/res as f32;
                        vert2[iter] = ((2_i32.pow(subdivisions)-colIndex) as f32 * v0[iter] + (colIndex - (rowIndex-1)/2) as f32 * v1[iter] + ((rowIndex-1)/2) as f32 * v2[iter])/res as f32;
                        
                        iter = iter+1;
                    }
                }

                let len0 = (vert0[0]*vert0[0] + vert0[1]*vert0[1] + vert0[2]*vert0[2]).sqrt();
                let len1 = (vert1[0]*vert1[0] + vert1[1]*vert1[1] + vert1[2]*vert1[2]).sqrt();
                let len2 = (vert2[0]*vert2[0] + vert2[1]*vert2[1] + vert2[2]*vert2[2]).sqrt();
                
                verts.push([vert0[0] * d/len0, vert0[1]*d/len0, vert0[2]*d/len0]);
                verts.push([vert1[0] * d/len1, vert1[1]*d/len1, vert1[2]*d/len1]);
                verts.push([vert2[0] * d/len2, vert2[1]*d/len2, vert2[2]*d/len2]);


                rowIndex = rowIndex+1;
            }
            colIndex=colIndex+1;
            
        }


        f = f+1;
    }
    
    f = 0;
    while f<5 {//iterate over 5 major tris that share the bottommost vert
        let v0 = mainVerts[11];
        let v1 = mainVerts[10-f];
        let v2 = mainVerts[10-((f+1)%5)];

        let mut colIndex = 0;
        while colIndex < res{
            let mut rowIndex = 0;
            while rowIndex < (colIndex+1).pow(2) - (colIndex).pow(2){
                
                let mut iter = 0;
                let mut vert0 = [0., 0., 0.];
                let mut vert1 = [0., 0., 0.];
                let mut vert2 = [0., 0., 0.];

                if rowIndex%2==0{
                    while iter<3{
                        vert0[iter] = ((2_i32.pow(subdivisions)-colIndex) as f32 * v0[iter] + (colIndex - rowIndex/2) as f32 * v1[iter] + (rowIndex/2) as f32 * v2[iter])/res as f32;
                        vert1[iter] = ((2_i32.pow(subdivisions)-colIndex - 1) as f32 * v0[iter] + (colIndex + 1 - rowIndex/2) as f32 * v1[iter] + (rowIndex/2) as f32 * v2[iter])/res as f32;
                        vert2[iter] = ((2_i32.pow(subdivisions)-colIndex - 1) as f32 * v0[iter] + (colIndex - rowIndex/2) as f32 * v1[iter] + (rowIndex/2 + 1) as f32 * v2[iter])/res as f32;

                        iter = iter+1;
                    }

                    
                } else {
                    while iter<3{
                        vert0[iter] = ((2_i32.pow(subdivisions)-colIndex - 1) as f32 * v0[iter] + (colIndex - (rowIndex-1)/2) as f32 * v1[iter] + ((rowIndex-1)/2 + 1) as f32 * v2[iter])/res as f32;
                        vert1[iter] = ((2_i32.pow(subdivisions)-colIndex) as f32 * v0[iter] + (colIndex - (rowIndex+1)/2) as f32 * v1[iter] + ((rowIndex+1)/2) as f32 * v2[iter])/res as f32;
                        vert2[iter] = ((2_i32.pow(subdivisions)-colIndex) as f32 * v0[iter] + (colIndex - (rowIndex-1)/2) as f32 * v1[iter] + ((rowIndex-1)/2) as f32 * v2[iter])/res as f32;
                        
                        iter = iter+1;
                    }
                }

                let len0 = (vert0[0]*vert0[0] + vert0[1]*vert0[1] + vert0[2]*vert0[2]).sqrt();
                let len1 = (vert1[0]*vert1[0] + vert1[1]*vert1[1] + vert1[2]*vert1[2]).sqrt();
                let len2 = (vert2[0]*vert2[0] + vert2[1]*vert2[1] + vert2[2]*vert2[2]).sqrt();
                
                //order is weird but important, not certain why
                verts.push([vert1[0] * d/len1, vert1[1]*d/len1, vert1[2]*d/len1]);
                verts.push([vert0[0] * d/len0, vert0[1]*d/len0, vert0[2]*d/len0]);
                verts.push([vert2[0] * d/len2, vert2[1]*d/len2, vert2[2]*d/len2]);


                rowIndex = rowIndex+1;
            }
            colIndex=colIndex+1;
            
        }


        f = f+1;
    }

    let mut norms = Vec::new();//create vector to hold calculated normals

    let mut i = 0;
    while i < verts.len(){//TODO: neither of these approaches will work once verts have variable heights
        //if you want to use the other type of normal comment out all code relevant to the current type
        //unsmoothed normals
        
        let x = (verts[i][0] + verts[i+1][0] + verts[i+2][0])/3.;
        let y = (verts[i][1] + verts[i+1][1] + verts[i+2][1])/3.;
        let z = (verts[i][2] + verts[i+1][2] + verts[i+2][2])/3.;
        let d1 = (x*x+y*y+z*z).sqrt();
        norms.push([x/d1, y/d1, z/d1]);
        norms.push([x/d1, y/d1, z/d1]);
        norms.push([x/d1, y/d1, z/d1]);
        i = i+3;
        
        //smoothed normals
        //norms.push([verts[i][0]/d, verts[i][1]/d, verts[i][2]/d]);
        //i=i+1;
    }

    //put verts into mesh
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        verts.clone(),
    )

    //put normals into mesh
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        norms,
    )
    
    //tell mesh which verts are connected
    .with_inserted_indices(Indices::U32((0u32..(verts.len() as u32)).collect::<Vec<_>>()))
}