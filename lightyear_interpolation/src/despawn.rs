use crate::interpolation_history::ConfirmedHistory;
use bevy_ecs::prelude::*;
use lightyear_core::interpolation::{Interpolated, InterpolatedDespawnedMarker};
use lightyear_replication::prelude::Confirmed;

/// Remove the component from interpolated entities when the confirmed component gets removed
// TODO: should the removal also be applied with interpolation delay?
pub(crate) fn removed_components<C: Component>(
    trigger: On<Remove, Confirmed<C>>,
    mut commands: Commands,
    query: Query<(), (With<Interpolated>, With<C>)>,
) {
    if query.get(trigger.entity).is_ok()
        && let Ok(mut entity) = commands.get_entity(trigger.entity)
    {
        entity.try_remove::<(C, ConfirmedHistory<C>)>();
    }
}

pub(crate) fn interpolated_despawn<C: Component>(
    mut commands: Commands,
    query: Query<
        (Entity, &ConfirmedHistory<C>),
        (With<Interpolated>, With<InterpolatedDespawnedMarker>),
    >,
) {
    for (entity, history) in query.iter() {
        if history.len() <= 1 {
            commands.entity(entity).try_despawn();
        }
    }
}
