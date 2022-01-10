//! https://developer.apple.com/documentation/usernotifications/setting_up_a_remote_notification_server/generating_a_remote_notification#2943365

use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Alert {
    title: Option<String>,
    subtitle: Option<String>,
    body: Option<String>,

    title_loc_key: Option<String>,
    title_loc_args: Option<Vec<String>>,

    subtitle_loc_key: Option<String>,
    subtitle_loc_args: Option<Vec<String>>,

    loc_key: Option<String>,
    loc_args: Option<Vec<String>>,

    launch_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlertPayload {
    Alert(Alert),
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SoundVolume {
    Silent,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Sound {
    Name(String),

    Critical {
        critical: bool,
        name: String,
        volume: SoundVolume,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterruptionLevel {
    Passive,
    Active,
    TimeSensitive,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    alert: AlertPayload,
    badge: Option<u64>,

    sound: Option<Sound>,

    category: Option<String>,
    thread_id: Option<String>,

    content_available: Option<bool>,
    mutable_content: Option<bool>,

    target_content_id: Option<String>,
    interruption_level: Option<InterruptionLevel>,

    relevance_score: Option<f64>,
}
