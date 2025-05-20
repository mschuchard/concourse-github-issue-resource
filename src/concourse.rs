//! # Concourse
//!
//! `concourse` contains the structs for serialization to concourse outputs and deserialization from concourse inputs. Ordinarily more functionality is required here, but this crate leverages the concourse rust bindings to automatically provide functionality through trait implementations.
use serde::{Deserialize, Serialize};

use concourse_resource::IntoMetadataKV;

// standard concourse structs
// check input and (vec seralized to list) output, out output
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
pub(super) struct Version {
    state: String,
}

impl Version {
    /// Constructor
    /// ```
    /// let version = Version::new(String::from("open"));
    /// ```
    pub(super) fn new(state: String) -> Self {
        Version { state }
    }
}

// check and out input
#[derive(Eq, PartialEq, Deserialize, Debug)]
pub(super) struct Source {
    // client and issues; owner and repo later converted to &str
    pat: Option<String>,
    owner: String,
    repo: String,
    // read
    number: Option<u64>,
    // list
    state: Option<String>,
    milestone: Option<u64>,
    assignee: Option<String>,
    labels: Option<Vec<String>>,
    // for skipping check during e.g. put+create
    skip_check: Option<bool>,
}

impl Source {
    /// Readers
    pub(super) fn pat(&self) -> Option<&str> {
        return self.pat.as_deref();
    }
    pub(super) fn owner(&self) -> &str {
        return &self.owner;
    }
    pub(super) fn repo(&self) -> &str {
        return &self.repo;
    }
    pub(super) fn number(&self) -> Option<u64> {
        return self.number;
    }
    pub(super) fn state(&self) -> Option<&str> {
        return self.state.as_deref();
    }
    pub(super) fn milestone(&self) -> Option<u64> {
        return self.milestone;
    }
    pub(super) fn assignee(&self) -> Option<Vec<String>> {
        // convert assignee to single element string vector for compatibility with github issue constructor
        return match self.assignee.clone() {
            Some(assignee) => Some(vec![assignee]),
            None => None,
        };
    }
    pub(super) fn labels(&self) -> Option<Vec<String>> {
        return self.labels.clone();
    }
    // return unwrapped value with default false for ease of use
    pub(super) fn skip_check(&self) -> bool {
        return self.skip_check.unwrap_or(false);
    }
}

// out input
#[derive(Eq, PartialEq, Deserialize, Debug, Default)]
pub(super) struct OutParams {
    // title and state later converted to &str
    title: Option<String>,
    body: Option<String>,
    labels: Option<Vec<String>>,
    assignees: Option<Vec<String>>,
    milestone: Option<u64>,
    // update only
    state: Option<String>,
}

impl OutParams {
    /// Readers
    pub(super) fn title(&self) -> Option<&str> {
        return self.title.as_deref();
    }
    pub(super) fn body(&self) -> Option<&str> {
        return self.body.as_deref();
    }
    pub(super) fn labels(&self) -> Option<Vec<String>> {
        return self.labels.clone();
    }
    pub(super) fn assignees(&self) -> Option<Vec<String>> {
        return self.assignees.clone();
    }
    pub(super) fn milestone(&self) -> Option<u64> {
        return self.milestone;
    }
    pub(super) fn state(&self) -> Option<&str> {
        return self.state.as_deref();
    }
}

// out output
#[derive(Eq, PartialEq, Serialize, Debug, IntoMetadataKV)]
pub(super) struct OutMetadata {
    number: u64,
    url: String,
    title: String,
    state: octocrab::models::IssueState,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    labels: Vec<octocrab::models::Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    assignees: Vec<octocrab::models::Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone: Option<octocrab::models::Milestone>,
    created: String,
    updated: String,
}

impl OutMetadata {
    /// Constructor
    /// ```
    /// let metadata = OutMetadata::new(
    ///     5,
    ///     String::from("http://does.not.exist"),
    ///     String::from("some issue"),
    ///     octocrab::models::IssueState::Open,
    ///     vec![],
    ///     vec![],
    ///     None,
    ///     String::from("yesterday"),
    ///     String::from("today"),
    /// ),
    /// ```
    pub(super) fn new(
        // ref: https://docs.rs/octocrab/latest/octocrab/models/issues/struct.Issue.html
        number: u64,
        url: impl Into<String>,
        title: String,
        state: octocrab::models::IssueState,
        labels: Vec<octocrab::models::Label>,
        assignees: Vec<octocrab::models::Author>,
        milestone: Option<octocrab::models::Milestone>,
        created: String,
        updated: String,
    ) -> Self {
        // type conversion traits
        let url = url.into();
        // return instantiated out metadata
        OutMetadata {
            number,
            url,
            title,
            state,
            labels,
            assignees,
            milestone,
            created,
            updated,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_new() {
        assert_eq!(
            Version::new(String::from("open")),
            Version {
                state: String::from("open")
            },
            "version could not be constructed with the correct issue state",
        );
    }
    #[test]
    fn test_version_deserialize() {
        let version = serde_json::from_str::<Version>("{\"state\": \"closed\"}")
            .expect("version could not be deserialized");
        assert_eq!(
            version,
            Version {
                state: String::from("closed")
            },
            "version did not contain the expected member values",
        )
    }

    #[test]
    fn test_source_owner() {
        assert_eq!(
            Source {
                pat: None,
                owner: String::from("myorg"),
                repo: String::from("myrepo"),
                state: Some(String::from("all")),
                number: None,
                milestone: None,
                assignee: None,
                labels: None,
                skip_check: None,
            }
            .owner,
            String::from("myorg"),
            "reader for source owner did not return expected member value"
        )
    }
    #[test]
    fn test_source_deserialize() {
        let json_input = r#"
{
    "owner": "mitodl",
    "repo": "ol-infrastructure",
    "number": 1,
    "state": "open",
    "skip_check": false
}"#;
        let source =
            serde_json::from_str::<Source>(json_input).expect("source could not be deserialized");
        assert_eq!(
            source,
            Source {
                pat: None,
                owner: String::from("mitodl"),
                repo: String::from("ol-infrastructure"),
                number: Some(1),
                state: Some(String::from("open")),
                milestone: None,
                assignee: None,
                labels: None,
                skip_check: Some(false),
            },
            "source did not contain the expected member values",
        )
    }

    #[test]
    fn test_outparams_title() {
        assert_eq!(
            OutParams {
                title: Some(String::from("mytitle")),
                body: None,
                labels: None,
                assignees: None,
                milestone: None,
                state: None,
            }
            .title,
            Some(String::from("mytitle")),
            "reader for outparams title did not return expected member value"
        )
    }
    #[test]
    fn test_outparams_deserialize() {
        let json_input = r#"
{
    "title": "my_issue",
    "body": "approve the concourse step",
    "assignees": ["my_user_one", "my_user_two"],
    "milestone": 2,
    "state": "closed"
}"#;
        let out_params = serde_json::from_str::<OutParams>(json_input)
            .expect("outparams could not be deserialized");
        assert_eq!(
            out_params,
            OutParams {
                title: Some(String::from("my_issue")),
                body: Some(String::from("approve the concourse step")),
                labels: None,
                assignees: Some(vec![
                    String::from("my_user_one"),
                    String::from("my_user_two")
                ]),
                milestone: Some(2),
                state: Some(String::from("closed")),
            },
            "out params did not contain the expected member values",
        )
    }

    #[test]
    fn test_outmetadata_new() {
        assert_eq!(
            OutMetadata::new(
                5,
                String::from("http://does.not.exist"),
                String::from("some issue"),
                octocrab::models::IssueState::Open,
                vec![],
                vec![],
                None,
                String::from("yesterday"),
                String::from("today"),
            ),
            OutMetadata {
                number: 5,
                url: String::from("http://does.not.exist"),
                title: String::from("some issue"),
                state: octocrab::models::IssueState::Open,
                labels: vec![],
                assignees: vec![],
                milestone: None,
                created: String::from("yesterday"),
                updated: String::from("today"),
            },
            "outmetadata could not be constructed with the correct values"
        )
    }

    #[test]
    fn test_outmetadata_serialize() {
        let out_metadata = OutMetadata::new(
            5,
            String::from("http://does.not.exist"),
            String::from("some issue"),
            octocrab::models::IssueState::Open,
            // cannot test next three since no constructors and non-exhaustive structs
            vec![],
            vec![],
            None,
            String::from("yesterday"),
            String::from("today"),
        );
        assert_eq!(
            serde_json::to_string(&out_metadata).expect("out metadata could not be serialized"),
            r#"{"number":5,"url":"http://does.not.exist","title":"some issue","state":"open","created":"yesterday","updated":"today"}"#,
            "out metadata did not contain the correct values"
        )
    }
}
