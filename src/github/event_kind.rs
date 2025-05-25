use serenity::all::{Color, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};

use crate::base::url_builder::LinkGenerator;

pub enum GithubEventKind {
    CommitPushed {
        title: String,
        owner: String,
        repository: String,
        branch: String,
        author: String,
        sha: String,
    },
    PrOpened {},
    PrClosed {},
    IssueOpened {},
}

impl GithubEventKind {
    fn _base_message(
        title: &str,
        owner: &str,
        repository: &str,
        author: &str,
        branch: &str,
        sha: &str,
        color: Color,
    ) -> CreateEmbed {
        let url_builder = LinkGenerator::new("https://github.com");

        let author_avatar_url = url_builder.url([&author, ".png"]);
        let owner_avatar_url = url_builder.url([&owner, ".png"]);
        let repository_url = url_builder.url([&owner, &repository]);
        let branch_url = url_builder.url([&owner, &repository, "tree", &branch]);
        let repository_inline_link = url_builder.inline_link(owner, [&repository]);
        let branch_inline_link = url_builder.inline_link(owner, [&repository, "tree", &branch]);

        let embed_author = CreateEmbedAuthor::new(author)
            .icon_url(&owner_avatar_url)
            .url(&repository_url);

        let embed_footer =
            CreateEmbedFooter::new(format!("Triggered by {}", author)).icon_url(author_avatar_url);

        let repository_inline_link_field = (repository_inline_link, repository_url, false);
        let branch_inline_link_field = (branch_inline_link, branch_url, false);

        CreateEmbed::new()
            .title(title)
            .field("repository", repository, false)
            .field("sha", sha, false)
            .color(color)
            .author(embed_author)
            .footer(embed_footer)
            .fields([repository_inline_link_field, branch_inline_link_field])
    }

    pub fn build(&self) -> CreateEmbed {
        match self {
            GithubEventKind::CommitPushed {
                title,
                owner,
                repository,
                branch,
                author,
                sha,
            } => todo!(),
            GithubEventKind::PrOpened {} => todo!(),
            GithubEventKind::PrClosed {} => todo!(),
            GithubEventKind::IssueOpened {} => todo!(),
        }
    }
}
