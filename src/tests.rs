use super::*;

#[test]
fn test_resource_check_read() {
    // validate basic check reading from mitodl/ol-infrastructure issue 1
    // concourse pipeline json input
    let source_input = r#"
{
    "owner": "mitodl",
    "repo": "ol-infrastructure",
    "number": 1
}"#;
    let version_input = r#"
{
    "state": "closed"
}"#;
    // deserialize version and source for inputs
    let source =
        serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Source>(source_input)
            .expect("source could not be deserialized");
    let version = serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Version>(
        version_input,
    )
    .expect("version could not be deserialized");
    let version_vec = GithubIssue::resource_check(Some(source), Some(version));
    // the issue is closed so we expect a size two vec
    assert_eq!(
        version_vec,
        vec![
            concourse::Version::new(octocrab::models::IssueState::Open),
            concourse::Version::new(octocrab::models::IssueState::Closed)
        ],
        "the resource_check did not return a two size vector of issue states for a closed issue",
    );
}

#[test]
fn test_resource_check_list() {
    // validate basic check listing from mitodl/ol-infrastructure and filtering to issue 833
    // concourse pipeline json input
    let source_input = r#"
{
    "owner": "mitodl",
    "repo": "ol-infrastructure",
    "trigger": "open",
    "assignee": "pdpinch",
    "creator": "blarghmatey",
    "milestone": 3,
    "state": "closed"
}"#;
    let version_input = r#"
{
    "state": "closed"
}"#;
    // deserialize version and source for inputs
    let source =
        serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Source>(source_input)
            .expect("source could not be deserialized");
    let version = serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Version>(
        version_input,
    )
    .expect("version could not be deserialized");
    let version_vec = GithubIssue::resource_check(Some(source), Some(version));
    // the issue is closed and trigger is open so we expect a size one vec
    assert_eq!(
        version_vec,
        vec![concourse::Version::new(octocrab::models::IssueState::Open)],
        "the resource_check did not return a one size vector of issue states for an issue with differing trigger and state",
    );
}

#[test]
fn test_resource_check_skip() {
    // validate basic check step skip
    // concourse pipeline json input
    let source_input = r#"
{
    "owner": "mitodl",
    "repo": "ol-infrastructure",
    "skip_check": true
}"#;
    let version_input = r#"
{
    "state": "open"
}"#;
    // deserialize version and source for inputs
    let source =
        serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Source>(source_input)
            .expect("source could not be deserialized");
    let version = serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Version>(
        version_input,
    )
    .expect("version could not be deserialized");
    let version_vec = GithubIssue::resource_check(Some(source), Some(version));
    // skip check step requested so we expect size two vec
    assert_eq!(
        version_vec,
        vec![
            concourse::Version::new(octocrab::models::IssueState::Open),
            concourse::Version::new(octocrab::models::IssueState::Closed)
        ],
        "the resource_check did not return a two size vector of issue states for a requested check skip",
    );
}

#[test]
fn test_resource_in() {
    let in_output = GithubIssue::resource_in(
        None,
        concourse::Version::new(octocrab::models::IssueState::Open),
        None,
        "",
    )
    .unwrap();
    assert_eq!(
        in_output.version,
        concourse::Version::new(octocrab::models::IssueState::Open),
        "the resource in did not dummy the expected return version output",
    );
}
