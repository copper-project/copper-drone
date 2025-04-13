pub mod tasks;
mod world;

use avian3d::prelude::Physics;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use cu29::prelude::debug;
use cu29::prelude::*;
use cu29_helpers::basic_copper_setup;
use std::fs;
use std::path::{Path, PathBuf};

// To enable sim, it is just your regular macro with sim_mode true
#[copper_runtime(config = "copperconfig.ron", sim_mode = true)]
struct DroneSim {}

// Encapsulate the Copper mock clock as a Bevy resource
#[derive(Resource)]
struct CopperMockClock {
    clock: RobotClockMock,
}

// Encapsulate the Copper runtime as a Bevy resource
// #[derive(Resource)] <- should not be a resource as it is a not send because of the AprilTag detector
struct Copper {
    _copper_ctx: CopperContext,
    copper_app: DroneSim,
}

fn default_callback(step: SimStep) -> SimOverride {
    match step {
        // Don't let the real task execute process and override with our logic.
        SimStep::Video(CuTaskCallbackState::Process(_, output)) => {
            SimOverride::ExecutedBySim
        },
        SimStep::Mspsink(CuTaskCallbackState::Process(_, _)) => SimOverride::ExecutedBySim,
        SimStep::Mspsrc(CuTaskCallbackState::Process(_, _)) => SimOverride::ExecutedBySim,
        _ => SimOverride::ExecuteByRuntime,
    }
}

fn setup_copper(world: &mut World) {
    #[allow(clippy::identity_op)]
    const LOG_SLAB_SIZE: Option<usize> = Some(1 * 1024 * 1024 * 1024);
    let logger_path = "logs/drone.copper";
    if let Some(parent) = Path::new(logger_path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create logs directory");
        }
    }

    // here we set up a mock clock so the simulation can take control of it.
    let (robot_clock, mock) = RobotClock::mock();
    let copper_ctx = basic_copper_setup(
        &PathBuf::from(logger_path),
        LOG_SLAB_SIZE,
        true,
        Some(robot_clock.clone()),
    )
    .expect("Failed to setup logger.");
    debug!(
        "Logger created at {}. This is a simulation.",
        path = logger_path
    );

    let mut copper_app = DroneSimBuilder::new()
        .with_context(&copper_ctx)
        .with_sim_callback(&mut default_callback)
        .build()
        .expect("Failed to create runtime.");

    copper_app
        .start_all_tasks(&mut default_callback)
        .expect("Failed to start all tasks.");
    
    // save all this in resources so we can grab them later during the simulation.
    world.insert_resource(CopperMockClock { clock: mock });
    world.insert_non_send_resource(Copper {
        _copper_ctx: copper_ctx,
        copper_app,
    });
}


/// This is a bevy system to trigger the Copper application to run one iteration.
/// We can query the state of the world from here and pass it to the Copper application.
#[allow(clippy::type_complexity)]
fn run_copper_callback(
    // mut query_set: ParamSet<(
    //     Query<(&mut Transform, &mut ExternalForce), With<Cart>>,
    //     Query<&Transform, With<Rod>>,
    // )>,
    physics_time: Res<Time<Physics>>,
    robot_clock: ResMut<CopperMockClock>,
    mut copper_ctx: NonSendMut<Copper>,
) {

    // Sync the copper clock to the simulated physics clock.
    robot_clock
        .clock
        .set_value(physics_time.elapsed().as_nanos() as u64);
    let mut sim_callback = move |step: SimStep<'_>| -> SimOverride {
        match step {
            SimStep::Video(CuTaskCallbackState::Process(_, output)) => {
                SimOverride::ExecutedBySim
            }
            SimStep::Mspsink(CuTaskCallbackState::Process(_, _)) => SimOverride::ExecutedBySim,
            SimStep::Mspsrc(CuTaskCallbackState::Process(_, _)) => SimOverride::ExecutedBySim,

            _ => SimOverride::ExecuteByRuntime,
        }
    };
    copper_ctx
        .copper_app
        .run_one_iteration(&mut sim_callback)
        .expect("Failed to run application.");
}

fn stop_copper_on_exit(mut exit_events: EventReader<AppExit>, mut copper_ctx: NonSendMut<Copper>) {
    for _ in exit_events.read() {
        println!("Exiting copper");
        copper_ctx
            .copper_app
            .stop_all_tasks(&mut default_callback) // let the tasks clean themselves up
            .expect("Failed to stop all tasks.");
    }
}

fn main() {
    let mut world = App::new();

    #[cfg(target_os = "macos")]
    let render_plugin = RenderPlugin::default(); // This let macos pick their own backend.

    #[cfg(not(target_os = "macos"))]
    let render_plugin = RenderPlugin {
        render_creation: bevy::render::settings::WgpuSettings {
            backends: Some(bevy::render::settings::Backends::VULKAN), // Force Vulkan backend when we know it is good.
            // This is to avoid some bugs when bevy tries out all the possible backends.
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };

    let default_plugin = DefaultPlugins.set(render_plugin).set(WindowPlugin {
        primary_window: Some(Window {
            title: "Copper Simulator".into(),
            ..default()
        }),
        ..default()
    });

    world.add_plugins(default_plugin);

    // setup everything that is simulation specific.
    let world = world::build_world(&mut world);

    // setup all the systems related to copper and the glue logic.
    world.add_systems(Startup, setup_copper);
    world.add_systems(Update, run_copper_callback);
    world.add_systems(PostUpdate, stop_copper_on_exit);
    world.run();
}
