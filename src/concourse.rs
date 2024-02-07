//! # Concourse
//!
//! `concourse` contains the structs for serialization to concourse outputs and deserialization from concourse inputs. Ordinarily more functionality is required here, but this crate leverages the concourse rust bindings to automatically provide functionality through trait implementations.

use serde::{Deserialize, Serialize};

use concourse_resource::IntoMetadataKV;

// standard concourse structs
// check input and (vec seralized to list) output, out output
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
pub(crate) struct Version {
    state: String,
}

impl Version {
    /// Constructor
    ///
    /// # Examples
    ///
    /// ```
    /// let version = Version::new(String::from("Open"));
    /// ```
    pub(crate) fn new(state: String) -> Self {
        Version { state }
    }
}

// check and out input
#[derive(Eq, PartialEq, Deserialize, Debug)]
pub(crate) struct Source {
    // client and issues
    pat: Option<String>,
    owner: String,
    repo: String,
    // read and update
    number: Option<u64>,
    // list
    milestone: Option<u64>,
}

impl Source {
    /// Readers
    pub(crate) fn pat(&self) -> Option<String> {
        return self.pat.clone();
    }
    pub(crate) fn owner(&self) -> String {
        return self.owner.clone();
    }
    pub(crate) fn repo(&self) -> String {
        return self.repo.clone();
    }
    pub(crate) fn number(&self) -> Option<u64> {
        return self.number;
    }
    pub(crate) fn milestone(&self) -> Option<u64> {
        return self.milestone;
    }
}

// out input
#[derive(Eq, PartialEq, Deserialize, Debug, Default)]
#[serde(default)]
pub(crate) struct OutParams {
    // title and body later converted to &str
    title: String,
    body: Option<String>,
    labels: Option<Vec<String>>,
    assignees: Option<Vec<String>>,
    milestone: Option<u64>,
}

impl OutParams {
    /// Readers
    pub(crate) fn title(&self) -> String {
        return self.title.clone();
    }
    pub(crate) fn body(&self) -> Option<String> {
        return self.body.clone();
    }
    pub(crate) fn labels(&self) -> Option<Vec<String>> {
        return self.labels.clone();
    }
    pub(crate) fn assignees(&self) -> Option<Vec<String>> {
        return self.assignees.clone();
    }
    pub(crate) fn milestone(&self) -> Option<u64> {
        return self.milestone.clone();
    }
}

// out output
#[derive(Eq, PartialEq, Serialize, Debug, IntoMetadataKV)]
pub(crate) struct OutMetadata {
    number: u64,
    labels: Vec<octocrab::models::Label>,
    assignees: Vec<octocrab::models::Author>,
    milestone: Option<octocrab::models::Milestone>,
}

impl OutMetadata {
    /// Constructor
    ///
    /// # Examples
    ///
    /// ```
    /// let metadata = OutMetadata::new(10, !vec[String::from("triage")], !vec[String::from("myuser")], 3); // this is inaccurate re: struct models
    /// ```
    pub(crate) fn new(
        number: u64,
        labels: Vec<octocrab::models::Label>,
        assignees: Vec<octocrab::models::Author>,
        milestone: Option<octocrab::models::Milestone>,
    ) -> Self {
        OutMetadata {
            number,
            labels,
            assignees,
            milestone,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_new() {
        assert_eq!(
            Version::new(String::from("Open")),
            Version {
                state: String::from("Open")
            },
            "version could not be constructed with the correct issue state",
        );
    }
    #[test]
    fn test_version_deserialize() {
        let version = serde_json::from_str::<Version>("{\"state\": \"Closed\"}")
            .expect("version could not be deserialized");
        assert_eq!(
            version,
            Version {
                state: String::from("Closed")
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
                number: None,
                milestone: None
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
    "number": 1
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
                milestone: None,
            },
            "source did not contain the expected member values",
        )
    }

    #[test]
    fn test_outparams_title() {
        assert_eq!(
            OutParams {
                title: String::from("mytitle"),
                body: None,
                labels: None,
                assignees: None,
                milestone: None,
            }
            .title,
            String::from("mytitle"),
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
    "milestone": 2
}"#;
        let out_params = serde_json::from_str::<OutParams>(json_input)
            .expect("outparams could not be deserialized");
        assert_eq!(
            out_params,
            OutParams {
                title: String::from("my_issue"),
                body: Some(String::from("approve the concourse step")),
                labels: None,
                assignees: Some(vec![
                    String::from("my_user_one"),
                    String::from("my_user_two")
                ]),
                milestone: Some(2),
            },
            "out params did not contain the expected member values",
        )
    }

    /*#[test]
    fn test_outmetadata_new() {
        assert_eq!(
            OutMetadata::new(
                5,
                vec![octocrab::models::Author {}],
                vec![octocrab::models::Label {}]
            ),
            OutMetadata {
                number: 5,
                labels: vec![octocrab::models::Author {}],
                assignees: vec![octocrab::models::Label {}]
            },
            "outmetadata could not be constructed with the correct values"
        )
    }*/
}
