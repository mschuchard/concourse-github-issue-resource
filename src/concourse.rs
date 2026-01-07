//! # Concourse
//!
//! `concourse` contains the structs for serialization to concourse outputs and deserialization from concourse inputs. Ordinarily more functionality is required here, but this crate leverages the concourse rust bindings to automatically provide functionality through trait implementations.
use serde::{Deserialize, Serialize};

use concourse_resource::IntoMetadataKV;
use octocrab;
use octocrab::models::IssueState;

// standard concourse structs
// check input and (vec seralized to list) output, out output
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
pub(super) struct Version {
    state: IssueState,
}

impl Version {
    /// Constructor
    /// ```
    /// let version = Version::new(IssueState::Closed);
    /// ```
    pub(super) fn new(state: IssueState) -> Self {
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
    creator: Option<String>,
    labels: Option<Vec<String>>,
    // for skipping check during e.g. put+create
    skip_check: Option<bool>,
    // trigger on issue state open or closed
    trigger: Option<IssueState>,
}

impl Source {
    /// Readers
    pub(super) fn pat(&self) -> Option<&str> {
        self.pat.as_deref()
    }
    pub(super) fn owner(&self) -> &str {
        &self.owner
    }
    pub(super) fn repo(&self) -> &str {
        &self.repo
    }
    pub(super) fn number(&self) -> Option<u64> {
        self.number
    }
    pub(super) fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }
    pub(super) fn milestone(&self) -> Option<u64> {
        self.milestone
    }
    pub(super) fn assignee(&self) -> Option<Vec<String>> {
        // convert assignee to single element string vector for compatibility with github issue constructor
        match self.assignee.clone() {
            Some(assignee) => Some(vec![assignee]),
            None => None,
        }
    }
    pub(super) fn creator(&self) -> Option<&str> {
        self.creator.as_deref()
    }
    pub(super) fn labels(&self) -> Option<Vec<String>> {
        self.labels.clone()
    }
    // return unwrapped value with default false for ease of use
    pub(super) fn skip_check(&self) -> bool {
        self.skip_check.unwrap_or(false)
    }
    // return unwrapped value with default closed for ease of use
    pub(super) fn trigger(&self) -> IssueState {
        self.trigger.clone().unwrap_or(IssueState::Closed)
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
    lock: Option<bool>,
    state: Option<String>,
}

impl OutParams {
    /// Readers
    pub(super) fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    pub(super) fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
    pub(super) fn labels(&self) -> Option<Vec<String>> {
        self.labels.clone()
    }
    pub(super) fn assignees(&self) -> Option<Vec<String>> {
        self.assignees.clone()
    }
    pub(super) fn milestone(&self) -> Option<u64> {
        self.milestone
    }
    pub(super) fn lock(&self) -> Option<bool> {
        self.lock
    }
    pub(super) fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }
}

// out output
#[derive(Eq, PartialEq, Serialize, Debug, IntoMetadataKV)]
pub(super) struct OutMetadata {
    number: u64,
    url: String,
    title: String,
    state: IssueState,
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
    ///     IssueState::Open,
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
        state: IssueState,
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
mod tests;
