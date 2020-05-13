use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct GitRef {
    sha: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PullRequest {
    base: GitRef,
    head: GitRef,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PullRequestWebhookEvent {
    pull_request: PullRequest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pull_request_event() {
        let expected = PullRequestWebhookEvent {
            pull_request: PullRequest {
                base: GitRef {
                    sha: String::from("f95f852bd8fca8fcc58a9a2d6c842781e32a215e"),
                },
                head: GitRef {
                    sha: String::from("ec26c3e57ca3a959ca5aad62de7213c562f8c821"),
                },
            },
        };
        let actual: PullRequestWebhookEvent = {
            use std::fs;
            let example = fs::File::open("testdata/example_pull_request_event.json").unwrap();
            serde_json::from_reader(example).unwrap()
        };
        assert_eq!(expected, actual);
    }
}

