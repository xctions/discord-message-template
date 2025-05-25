use serenity::all::{CreateEmbed, CreateEmbedFooter};

pub mod event_data;
pub mod event_kind;
pub mod message_template;

pub enum GithubMessageTemplate {
    PullRequest,
}

impl GithubMessageTemplate {
    pub fn commit(
        title: &str,
        description: &str,
        repository: &str,
        sha: &str,
        author: &str,
        author_url: &str,
    ) -> CreateEmbed {
        CreateEmbedFooter::new("").icon_url("");

        CreateEmbed::new()
            .title(title)
            .description(description)
            .field("Repository", repository, false)
            .field("SHA", sha, false)
            .field("Author", author, false)
            .field("Author URL", author_url, false)
    }
}

pub fn test() -> String {
    "test".to_string()
}
