use super::*;

#[test]
fn test_version_new() {
    assert_eq!(
        Version::new(octocrab::models::IssueState::Open),
        Version {
            state: octocrab::models::IssueState::Open
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
            state: octocrab::models::IssueState::Closed
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
            trigger: None,
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
    "skip_check": false,
    "trigger": "open"
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
            trigger: Some(octocrab::models::IssueState::Open)
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
            lock: None,
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
    let out_params =
        serde_json::from_str::<OutParams>(json_input).expect("outparams could not be deserialized");
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
            lock: None,
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
