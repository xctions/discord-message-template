use serenity::all::CreateEmbed;

use crate::base::discord_message::{Message, MessageBuilder};

use super::{event_data::GithubEventData, event_kind::GithubEventKind};

impl Message {
    pub fn github_pr_opened(event: GithubEventKind) -> Self {
        let embed = CreateEmbed::new();
        Self {
            content: None,
            embeds: vec![embed],
        }
    }
}
