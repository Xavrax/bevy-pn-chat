use bevy::{
    prelude::{AssetServer, Commands, Component, Entity, Query, Res, Transform},
    tasks::{AsyncComputeTaskPool, Task},
    text::{Text2dBounds, Text2dBundle, TextStyle},
};
use futures_lite::future;

use crate::error::Result;

use super::{
    messages::{subscribe, ChatMessage, SubscriptionResult},
    resources::{ChatMessageStyle, MessageFormat, PubNubSubscribeResource},
};

#[derive(Component)]
pub struct PublishTask(pub Task<Result<()>>);

#[derive(Component)]
pub struct SubscribeTask(pub Task<Result<SubscriptionResult>>);

pub fn tasks_handler(
    mut commands: Commands,
    subscription_info: Res<PubNubSubscribeResource>,
    mut publish_tasks: Query<(Entity, &mut PublishTask)>,
    mut subscribe_tasks: Query<(Entity, &mut SubscribeTask)>,
    asset_server: Res<AssetServer>,
    message_style: Res<ChatMessageStyle>,
    message_format: Res<MessageFormat>,
) {
    publish_tasks.iter_mut().for_each(|(entity, mut task)| {
        future::block_on(future::poll_once(&mut task.0)).map(|res| {
            res.map_err(|err| log::error!("Error occurred in async publish task: {:?}", err))
                .ok();
            commands.entity(entity).despawn()
        });
    });

    subscribe_tasks.iter_mut().for_each(|(entity, mut task)| {
        future::block_on(future::poll_once(&mut task.0)).map(|res| {
            res.map_err(|err| log::error!("Error occurred in async subscribe task: {:?}", err))
                .map(|result| {
                    let subscribe_key = subscription_info.subscribe_key.clone();
                    let channel = subscription_info.channel.clone();
                    let user_id = subscription_info.user_id.clone();
                    let tt = result.message_info.tt;
                    let tr = result.message_info.tr.to_string();

                    let thread_pool = AsyncComputeTaskPool::get();
                    let task = thread_pool
                        .spawn(async move { subscribe(subscribe_key, channel, tt, tr, user_id) });

                    commands.spawn(SubscribeTask(task));

                    let font = asset_server.load(message_style.font_path.to_str().unwrap_or(""));
                    result.messages.iter().for_each(|message| {
                        commands.spawn((
                            ChatMessage,
                            Text2dBundle {
                                text: bevy::text::Text::from_section(
                                    message_format
                                        .clone()
                                        .replace("{username}", &message.user_id)
                                        .replace("{message}", &message.payload)
                                        .replace("{channel}", &message.channel),
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: message_style.font_size,
                                        color: message_style.color,
                                    },
                                )
                                .with_alignment(bevy::text::TextAlignment::Left),
                                transform: Transform::from_xyz(30.0, 70.0, 0.0),
                                ..Default::default()
                            },
                        ));
                    });
                })
                .ok();
            commands.entity(entity).despawn();
        });
    });
}
