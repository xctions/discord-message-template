use serenity::all::CreateEmbed;

use crate::github::event_kind::GithubEventKind;

pub enum MessageContext {
    // #[cfg(feature = "github")]
    Github(GithubEventKind),
}

pub struct Message {
    pub content: Option<String>,
    pub embeds: Vec<CreateEmbed>,
}

// impl Message {
//     pub fn content(&self) -> Option<&String> {
//         self.content.as_ref()
//     }

//     pub fn embeds(&self) -> &Vec<CreateEmbed> {
//         &self.embeds
//     }
// }

pub trait MessageBuilder {
    fn build(&self, ctx: MessageContext) -> Message;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_commit_pushed_event_kind_fields() {
        let ctx = MessageContext::Github(GithubEventKind::CommitPushed {
            title: "테스트 타이틀".to_string(),
            owner: "zdpk".to_string(),
            repository: "discord-message-template".to_string(),
            branch: "main".to_string(),
            author: "zdpk".to_string(),
            sha: "abc123".to_string(),
        });

        // enum이 잘 생성되는지, 필드 값이 잘 들어가는지 확인
        if let MessageContext::Github(GithubEventKind::CommitPushed { title, .. }) = ctx {
            assert_eq!(title, "테스트 타이틀");
        } else {
            panic!("CommitPushed variant가 아님");
        }
    }
}
