use serenity::all::{Color, CreateEmbed, CreateEmbedFooter};
use time::{OffsetDateTime, UtcOffset, format_description};

use super::github_types::{Author, Commit, Repository};

pub enum GithubTemplate {
    CommitPushed {
        author: Author,
        repository: Repository,
        commit: Commit,
    },
    PrOpened {},
    PrClosed {},
    IssueOpened {},
}

impl GithubTemplate {
    fn now_formatted() -> String {
        // 2024/06/07 15:04:05.123 +09:00
        let format = format_description::parse(
            "[year]/[month padding:zero]/[day padding:zero] [hour padding:zero]:[minute padding:zero]:[second padding:zero] [offset_hour sign:mandatory]"
        ).unwrap();

        let now = OffsetDateTime::now_utc().to_offset(UtcOffset::current_local_offset().unwrap());
        now.format(&format).unwrap()
    }

    fn footer(author: &Author) -> CreateEmbedFooter {
        let text = format!("Triggered by {} • {}", author.name(), Self::now_formatted());
        CreateEmbedFooter::new(text).icon_url(author.avatar_url())
    }

    pub fn commit_pushed(repository: &Repository, commit: &Commit, author: &Author) -> CreateEmbed {
        let title = format!("Commit is successfully pushed");
        CreateEmbed::new()
            .author(repository.owner_author())
            .title(title)
            .description(commit.message())
            .field("branch", commit.branch_inline_link(&repository), true)
            .field("commit", commit.commit_inline_link(&repository), true)
            .footer(Self::footer(author))
            .color(Color::BLUE)
    }

    pub fn build(&self) -> CreateEmbed {
        match self {
            Self::CommitPushed {
                repository,
                author,
                commit,
            } => Self::commit_pushed(repository, commit, author),
            Self::PrOpened {} => todo!(),
            Self::PrClosed {} => todo!(),
            Self::IssueOpened {} => todo!(),
        }
    }
}
