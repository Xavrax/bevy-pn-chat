use bevy::{
    prelude::{Commands, Component, Res},
    tasks::AsyncComputeTaskPool,
};
use pubnub::{
    core::{blocking::Transport, TransportMethod, TransportRequest},
    transport::reqwest::blocking::TransportReqwest,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::Result, BevyPNError};

use super::{resources::PubNubSubscribeResource, tasks::SubscribeTask};

#[derive(Component)]
pub struct ChatMessage;

pub fn message_handler(mut commands: Commands, subscription_info: Res<PubNubSubscribeResource>) {
    let thread_pool = AsyncComputeTaskPool::get();

    let subscribe_key = subscription_info.subscribe_key.clone();
    let channel = subscription_info.channel.clone();
    let tt = subscription_info.tt.clone();
    let tr = subscription_info.tr.clone();
    let user_id = subscription_info.user_id.clone();

    let task = thread_pool.spawn(async move { subscribe(subscribe_key, channel, tt, tr, user_id) });

    commands.spawn(SubscribeTask(task));
}

pub fn subscribe(
    subscribe_key: String,
    channel: String,
    tt: String,
    tr: String,
    user_id: String,
) -> Result<SubscriptionResult> {
    let transport = TransportReqwest::new();

    let request = TransportRequest {
        path: format!("v2/subscribe/{}/{}/0", subscribe_key, channel),
        query_parameters: [
            ("tt".into(), tt),
            ("tr".into(), tr),
            ("uuid".into(), user_id),
        ]
        .into(),
        method: TransportMethod::Get,
        headers: [].into(),
        body: None,
    };

    let response = transport.send(request);

    response.map_err(Into::into).and_then(|response| {
        response
            .body
            .ok_or_else(|| BevyPNError::EmptyBody {
                on: "Subscribe".into(),
            })
            .and_then(|body| {
                serde_json::from_slice::<SubscriptionResult>(&body).map_err(Into::into)
            })
    })
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionResult {
    #[serde(rename = "t")]
    pub message_info: SubscriptionInfo,

    #[serde(rename = "m")]
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionInfo {
    #[serde(rename = "t")]
    pub tt: String,

    #[serde(rename = "r")]
    pub tr: i32,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(rename = "c")]
    pub channel: String,

    #[serde(rename = "d")]
    pub payload: String,
}
