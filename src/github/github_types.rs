use serenity::all::CreateEmbedAuthor;

const GITHUB_BASE_URL: &str = "https://github.com";

pub struct Repository {
    slug: String,
    name: String,
    owner: String,
    url: String,
    owner_avatar_url: String,
}

impl Repository {
    pub fn new(slug: &str) -> Self {
        let (owner, name) = slug.split_once('/').unwrap();

        Self {
            slug: slug.to_string(),
            name: name.to_string(),
            owner: owner.to_string(),
            url: format!("{}/{}", GITHUB_BASE_URL, slug),
            owner_avatar_url: format!("{}/{}.png", GITHUB_BASE_URL, owner),
        }
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn owner_avatar_url(&self) -> &str {
        &self.owner_avatar_url
    }

    pub fn owner_author(&self) -> CreateEmbedAuthor {
        CreateEmbedAuthor::new(self.owner_inline_link())
            .icon_url(self.owner_avatar_url())
            .url(self.url())
    }

    pub fn owner_inline_link(&self) -> String {
        format!("[{}]({})", self.slug, self.url)
    }
}

pub struct Author {
    name: String,
    avatar_url: String,
}

impl Author {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            avatar_url: format!("{}/{}.png", GITHUB_BASE_URL, name),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn avatar_url(&self) -> &str {
        &self.avatar_url
    }
}

pub struct Commit {
    pub author: Author,
    sha: String,
    pub message: String,
    pub branch: String,
}

impl Commit {
    pub fn new(author: Author, message: &str, branch: &str, sha: &str) -> Self {
        Self {
            author,
            sha: sha.to_string(),
            message: message.to_string(),
            branch: branch.to_string(),
        }
    }

    pub fn author(&self) -> &Author {
        &self.author
    }

    pub fn short_sha(&self) -> &str {
        &self.sha[..6]
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn commit_inline_link(&self, repository: &Repository) -> String {
        format!("[{}]({})", self.short_sha(), self.commit_url(repository))
    }

    fn commit_url(&self, repository: &Repository) -> String {
        format!("{}/commit/{}", repository.url(), self.sha)
    }
}
