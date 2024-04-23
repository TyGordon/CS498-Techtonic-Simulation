// Libraries/Crates/Packages
use bevy::
{
    app::AppExit, math::quat, prelude::*, render::
    {
        camera::{self, RenderTarget}, mesh::{Indices, VertexAttributeValues}, render_asset::RenderAssetUsages, render_resource::{Extent3d, PrimitiveTopology, TextureDimension, TextureFormat}
    }, window::WindowRef
};

use bevy_save::prelude::*;

fn main()
{
    //subdivided triangle coordinate reference - largely unused for now, but in future will likely be used to lookup height values when generating mesh
    //
    //           c
    //          /\
    //         /__\
    //        /\  /\
    //       /__\/__\
    //      /\  /\  /\
    //     /__\/__\/__\
    //    /\  /\  /\  /\
    //   /__\/__\/__\/__\
    //  b                a
    //
    //
    //
    //  b________________a
    //   \  /\  /\  /\  /
    //    \/__\/__\/__\/
    //     \  /\  /\  /
    //      \/__\/__\/
    //       \  /\  /
    //        \/__\/
    //         \  /
    //          \/
    //           c
    //
    //
    //
    //because it is 2d, we can track each point on a supertriangle uniquely with only 2 coordinates 
    //a is distance from left edge, b is distance from right edge, c won't be stored, but represents dist from the horizontal edge and can be calculated as 2^subdivisions - a - b
    //if any of the coords are negative, we are on a different supertri.
    //if any of the coords are 0, we will have an edge (haha get it) case that we need to figure out because two (on edge) or five (on corner) supertris share this point
    //the more i think about this the more i think it should be ok, because it will only ever be a problem if the same point on two supertris has two different values, and we should be able to prevent that
    //
    //
    //
    //icosahedron net array reference
    //                                 /\  /\  /\  /\  /\   [0,0]-[0,4]
    //      /\  /\  /\  /\  /\        /__\/__\/__\/__\/__\
    //     /__\/__\/__\/__\/__\       \  /\  /\  /\  /\  /
    //    /\  /\  /\  /\  /\  /        \/  \/  \/  \/  \/   [1,0]-[1,4]
    //   /__\/__\/__\/__\/__\/  ==>  /\  /\  /\  /\  /\     [2,0]-[2,4]
    //   \  /\  /\  /\  /\  /       /__\/__\/__\/__\/__\
    //    \/  \/  \/  \/  \/        \  /\  /\  /\  /\  /
    //                               \/  \/  \/  \/  \/     [3,0]-[3,4]
    //
    //
    //coords of adjacent supertri to super tri with index [i,j] by i (aka row):
    // i  left          right         vertical
    // 0: [0, (j-1)%5], [0, (j+1)%5], [1, j]
    // 1: [2, j],       [2, (j+1)%5], [0, j]
    // 2: [1, (j-1)%5], [1, j],       [3, j]
    // 3: [3, (j-1)%5], [3, (j+1)%5], [2, j]
    //
    //
    // phi = (1 + sqrt(5))/2)
    // d = sqrt(1 + phi^2) = sqrt( 2 * ( 5 + sqrt(5)))/2 ~= 1.902113
    //
    //
    // coords of top/bottom point: (0, d, 0), (0, -d, 0)
    //
    // coords of top ring: (0, 0.85065, -1.7013),  (phi, 0.85065, -sqrt(d^2 - phi^2 - 0.85065^2), (1, 0.85065, sqrt(d^2 - 1 - 0.85065^2)), (-1, 0.85065, sqrt(d^2 - 1 - 0.85065^2)), (-phi, 0.85065, -sqrt(d^2 - phi^2 - 0.85065^2))
    //
    // coords of bottom ring: (-1, -0.85065, -sqrt(d^2 - 1 - 0.85065^2)), (1, -0.85065, -sqrt(d^2 - 1 - 0.85065^2)), (phi, -0.85065, sqrt(d^2 - phi^2 - 0.85065^2)), (0, -0.85065, 1.7013), (-phi, -0.85065, sqrt(d^2 - phi^2 - 0.85065^2))

    //let mut 
    
    let h = HeightValues { values: Vec::<Vec<f32>>::new() };

    // Create the main menu app
    let mut app = App::new();

    // Insert the heights into the app
    app.insert_resource(h);

    // Add systems to the main app
    app.add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, camera_setup)
        .add_systems(OnEnter(AppState::MainMenu), (menu_setup, render_setup))
        .add_systems(Update, (main_button_system.run_if(in_state(AppState::MainMenu)), input_handler.run_if(in_state(AppState::MainMenu))))
        .add_systems(OnEnter(AppState::Simulate), (simulate_gui, render_setup))
        .add_systems(Update, (simulate_button_system.run_if(in_state(AppState::Simulate)), input_handler.run_if(in_state(AppState::Simulate))));

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

#[derive(Component)]
enum SimulateAction
{
    Pause,
    StepBack,
    Save,
    Quit,
}

#[derive(Resource)]
struct HeightValues {
    values: Vec<Vec<f32>>,
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
}

// This function handles
//  1. When a button is pressed
//  2. When the mouse hovers over a button
//  3. When the mouse is not hovering over a button
fn main_button_system
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

    mut entity_query: Query<(Entity, &Transform), With<Handle<Mesh>>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut button_query: Query<(Entity, &Style)>,
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

                        // Delete all buttons and labels
                        for (entity, _) in &mut button_query
                        {
                            commands.entity(entity).despawn();
                        }

                        // Remove the height values resource
                        commands.remove_resource::<HeightValues>();

                        // Create a new height values resource
                        let h = HeightValues { values: Vec::<Vec<f32>>::new() };

                        // Insert the new height values resource
                        commands.insert_resource(h);

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

fn simulate_button_system
(
    mut interaction_query: Query<
        (
            &Interaction,
            &SimulateAction,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,

    mut commands: Commands,

    mut entity_query: Query<(Entity, &Transform), With<Shape>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut button_query: Query<(Entity, &Style)>,
)
{
    for (interaction, simulate_action, mut border_color) in &mut interaction_query
    {

        match *interaction
        {
            // If the button is pressed, determine which one was pressed
            Interaction::Pressed =>
            {
                match *simulate_action
                {
                    SimulateAction::Pause =>
                    {
                        // If the pause button was pressed, stop the simulation and replace it with a play button.
                    }

                    SimulateAction::StepBack =>
                    {
                        // If the step back button was pressed, move the simulation back one state.
                    }

                    SimulateAction::Save =>
                    {
                        // If the save button was pressed, save all the data of the simulation's current state.
                    }
                    
                    SimulateAction::Quit =>
                    {
                        // If the quit button was pressed, go back to the main menu

                        // Delete the icosahedron
                        let (entity, _) = entity_query.single_mut();
                        commands.entity(entity).despawn();

                        // Delete all buttons and labels
                        for (entity, _) in &mut button_query
                        {
                            commands.entity(entity).despawn();
                        }

                        // Remove the height values resource
                        commands.remove_resource::<HeightValues>();

                        // Create a new height values resource
                        let h = HeightValues { values: Vec::<Vec<f32>>::new() };

                        // Insert the new height values resource
                        commands.insert_resource(h);

                        next_state.set(AppState::MainMenu);
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
fn menu_setup(mut commands: Commands)
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

// This function handles setting up the gui hud during the simulation.
fn simulate_gui
(
    mut commands: Commands,
)
{

    // Spawn control buttons
    commands.spawn
    (
        // Create the node that contains the buttons
        (
            NodeBundle
            {
                style: Style
                {
                    // Set the ideal height of the buttons row in pixels
                    top: Val::Px(0.0),

                    // Horizontally align buttons node
                    align_items: AlignItems::Center,

                    // Horizontally align buttons node
                    justify_self: JustifySelf::End,

                    // Align the buttons in a row
                    flex_direction: FlexDirection::Row,

                    ..default()
                },

                ..default()
            },
        )
    )

    .with_children
    (
        |parent|
        {
            parent.spawn
            (
                (
                    // Create the pause button within the node
                    ButtonBundle
                    {
                        style: Style
                        {
                            // Width of the textbox of the button in pixels
                            width: Val::Px(50.0),

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

                    SimulateAction::Pause,
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
                            "||",

                            // Set the style of the text of the button
                            TextStyle
                            {
                                // Set the font of the text to default
                                font: default(),

                                // Set the font size of the text
                                font_size: 20.0,

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
                    // Create the step back button within the node
                    ButtonBundle
                    {
                        style: Style
                        {
                            // Width of the textbox of the button in pixels
                            width: Val::Px(50.0),

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

                    SimulateAction::StepBack,
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
                            "<||",

                            // Set the style of the text of the button
                            TextStyle
                            {
                                // Set the font of the text to default
                                font: default(),

                                // Set the font size of the text
                                font_size: 20.0,

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
                            width: Val::Px(65.0),

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

                    SimulateAction::Save,
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
                            "Save",

                            // Set the style of the text of the button
                            TextStyle
                            {
                                // Set the font of the text to default
                                font: default(),

                                // Set the font size of the text
                                font_size: 20.0,

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
                            width: Val::Px(65.0),

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

                    SimulateAction::Quit,
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
                                font_size: 20.0,

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

fn render_setup(
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

struct SavePipeline;

// Save Pipeline
impl Pipeline for SavePipeline {
    type Backend = DefaultDebugBackend;
    type Format = DefaultDebugFormat;

    type Key<'a> = &'a str;

    fn key(&self) -> Self::Key<'_> {
        "saves"
    }

    fn capture(builder: SnapshotBuilder) -> Snapshot {
        builder
            //.deny::<Mesh2dHandle>()
            .deny::<Handle<ColorMaterial>>()
            .extract_resource::<HeightValues>()
            .extract_rollbacks()
            .build()
    }

    fn apply(world: &mut World, snapshot: &Snapshot) -> Result<(), bevy_save::Error> {
        snapshot
            .applier(world)
            .apply()
    }
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