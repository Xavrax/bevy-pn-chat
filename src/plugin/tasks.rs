use bevy::{
    prelude::{Commands, Component, Entity, Query},
    tasks::Task,
};
use futures_lite::future;

use crate::error::Result;

#[derive(Component)]
pub struct PublishTask(pub Task<Result<()>>);

pub fn tasks_handler(mut commands: Commands, mut tasks: Query<(Entity, &mut PublishTask)>) {
    tasks.iter_mut().for_each(|(entity, mut task)| {
        future::block_on(future::poll_once(&mut task.0)).map(|res| {
            res.map_err(|err| log::error!("Error occurred in async publish task: {:?}", err))
                .ok();
            commands.entity(entity).despawn()
        });
    });
}
