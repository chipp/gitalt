use crate::repo_id::SERVER_URL;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub id: u16,
    pub from_ref: Ref,
    pub to_ref: Ref,
    pub state: PullRequestState,
}

#[derive(Deserialize, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "UPPERCASE")]
pub enum PullRequestState {
    Open,
    Declined,
    Merged,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ref {
    pub display_id: String,
    pub id: String,
    pub latest_commit: String,
    pub repository: Repository,
}

#[derive(Deserialize)]
pub struct Repository {
    pub slug: String,
    pub id: u16,
    pub project: Project,
}

#[derive(Deserialize)]
pub struct Project {
    pub key: String,
    pub id: u16,
}

impl PullRequest {
    pub fn url(&self) -> Url {
        let mut url = Url::parse(SERVER_URL).unwrap();

        {
            let mut segments = url.path_segments_mut().unwrap();
            segments.push("projects");
            segments.push(&self.to_ref.repository.project.key.to_uppercase());
            segments.push("repos");
            segments.push(&self.to_ref.repository.slug);
            segments.push("pull-requests");
            segments.push(&self.id.to_string());
        }

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url() {
        let pr = PullRequest {
            id: 42,
            from_ref: Ref {
                display_id: "_".to_string(),
                id: "_".to_string(),
                latest_commit: "1".to_string(),
                repository: Repository {
                    slug: "gitbucket".to_string(),
                    id: 42,
                    project: Project {
                        key: "~vburduko".to_string(),
                        id: 1,
                    },
                },
            },
            to_ref: Ref {
                display_id: "_".to_string(),
                id: "_".to_string(),
                latest_commit: "1".to_string(),
                repository: Repository {
                    slug: "gitbucket".to_string(),
                    id: 42,
                    project: Project {
                        key: "VB".to_string(),
                        id: 42,
                    },
                },
            },
            state: PullRequestState::Open,
        };

        assert_eq!(
            pr.url().as_str(),
            "https://bitbucket.company.com/projects/VB/repos/gitbucket/pull-requests/42"
        )
    }
}
