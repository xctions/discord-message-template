use serenity::all::{Color, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};

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
    pub fn commit_pushed(repository: &Repository, commit: &Commit, author: &Author) -> CreateEmbed {
        CreateEmbed::new()
            .author(repository.owner_author())
            .url(repository.url())
            .title(commit.message())
            .description(commit.message())
            .field("Repository", repository.owner_inline_link(), true)
            .field("Commit", commit.commit_inline_link(&repository), true)
            .footer(CreateEmbedFooter::new(author.name()).icon_url(author.avatar_url()))
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
