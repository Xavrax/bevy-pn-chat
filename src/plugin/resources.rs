use std::ops::Deref;

use crate::TextStyle;
use bevy::prelude::Resource;
use pubnub::{
    transport::{middleware::PubNubMiddleware, reqwest::blocking::TransportReqwest},
    PubNubClient,
};

#[derive(Debug, Clone, Resource)]
pub struct InputBoxStyle(pub TextStyle);

impl Deref for InputBoxStyle {
    type Target = TextStyle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Resource)]
pub struct PubNubClientResource(pub PubNubClient<PubNubMiddleware<TransportReqwest>>);

impl Deref for PubNubClientResource {
    type Target = PubNubClient<PubNubMiddleware<TransportReqwest>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: it has to be kept in memory because of lack of subscription implementation
#[derive(Clone, Resource)]
pub struct PubNubSubscribeResource {
    pub tt: String,
    pub tr: String,
    pub subscribe_key: String,
    pub channel: String,
    pub user_id: String,
}

#[derive(Resource)]
pub struct ChannelResource(pub String);

impl Deref for ChannelResource {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
