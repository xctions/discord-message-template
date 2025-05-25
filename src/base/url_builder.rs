pub struct LinkGenerator {
    pub base_url: String,
}

impl LinkGenerator {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
    pub fn url<I, S>(&self, paths: I) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut url = self.base_url.clone();
        for path in paths {
            if !url.ends_with('/') {
                url.push('/');
            }
            url.push_str(path.as_ref());
        }
        url
    }

    pub fn inline_link<I, S>(&self, name: &str, paths: I) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let url = self.url(paths);
        format!("[{}]({})", name, url)
    }
}
