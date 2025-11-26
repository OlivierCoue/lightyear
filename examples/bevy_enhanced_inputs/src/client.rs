//! The client plugin.
//! The client will be responsible for:
//! - connecting to the server at Startup
//! - sending inputs to the server
//! - applying inputs to the locally predicted player (for prediction to work, inputs have to be applied to both the
//!   predicted entity and the server entity)

use std::thread::sleep;
use std::time::Duration;

use crate::protocol::*;
use crate::shared;
use bevy::prelude::*;
use lightyear::input::bei::prelude::{bindings, Action, ActionOf, Bindings, Cardinal, Fire};
use lightyear::prelude::*;

pub struct ExampleClientPlugin;

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_predicted_spawn);
        app.add_observer(handle_interpolated_spawn);
        app.add_observer(player_movement);
        app.add_observer(player_shoot);
        app.add_systems(
            FixedUpdate,
            // simulate some client-side workload increase to 16 to really see rollback due to ticks from future
            || sleep(Duration::from_millis(10)),
        );
    }
}

/// The client input only gets applied to predicted entities that we own
/// This works because we only predict the user's controlled entity.
/// If we were predicting more entities, we would have to only apply movement to the player owned one.
fn player_movement(
    trigger: On<Fire<Movement>>,
    mut position_query: Query<&mut PlayerPosition, With<Predicted>>,
) {
    if let Ok(position) = position_query.get_mut(trigger.context) {
        // NOTE: be careful to directly pass Mut<PlayerPosition>
        // getting a mutable reference triggers change detection, unless you use `as_deref_mut()`
        shared::shared_movement_behaviour(position, trigger.value);
    }
}

fn player_shoot(trigger: On<Fire<Shoot>>, mut commands: Commands) {
    commands.spawn((
        Projectile,
        PlayerPosition(Vec2::ZERO),
        PreSpawned::default(),
    ));
}

/// When the predicted copy of the client-owned entity is spawned, do stuff
/// - assign it a different saturation
/// - keep track of it in the Global resource
pub(crate) fn handle_predicted_spawn(
    trigger: On<Add, (PlayerId, Predicted)>,
    mut predicted: Query<(&mut PlayerColor, Has<Controlled>), With<Predicted>>,
    mut commands: Commands,
) {
    let entity = trigger.entity;
    if let Ok((mut color, controlled)) = predicted.get_mut(entity) {
        let hsva = Hsva {
            saturation: 0.4,
            ..Hsva::from(color.0)
        };
        color.0 = Color::from(hsva);
        warn!("Add InputMarker to entity: {:?}", entity);
        if controlled {
            // add Action entities to the predicted Context
            commands.spawn((
                ActionOf::<Player>::new(entity),
                Action::<Movement>::new(),
                Bindings::spawn(Cardinal::wasd_keys()),
            ));
            commands.spawn((
                ActionOf::<Player>::new(entity),
                Action::<Shoot>::new(),
                bindings![MouseButton::Right],
            ));
        }
    }
}

/// When the predicted copy of the client-owned entity is spawned, do stuff
/// - assign it a different saturation
/// - keep track of it in the Global resource
pub(crate) fn handle_interpolated_spawn(
    trigger: On<Add, PlayerColor>,
    mut interpolated: Query<&mut PlayerColor, With<Interpolated>>,
) {
    if let Ok(mut color) = interpolated.get_mut(trigger.entity) {
        let hsva = Hsva {
            saturation: 0.1,
            ..Hsva::from(color.0)
        };
        color.0 = Color::from(hsva);
    }
}
