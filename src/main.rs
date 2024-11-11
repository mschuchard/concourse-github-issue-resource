use concourse_resource::*;
use env_logger;
use log;

mod concourse;
mod github_issue;

struct GithubIssue {}

impl concourse_resource::Resource for GithubIssue {
    // implementations for inputs and outputs
    type Source = concourse::Source;
    type Version = concourse::Version;
    type InParams = concourse_resource::Empty;
    type InMetadata = concourse_resource::Empty;
    type OutParams = concourse::OutParams;
    type OutMetadata = concourse::OutMetadata;

    // implementations for steps
    /// Performs the check step for the resource. Returns a single sized vector of version of state string if the input issue is open (no trigger), and a two sized vector of version of state string if the input issue is closed (trigger). For convenience and standardization the former return is "open", and the latter is "open" and "closed".
    #[tokio::main]
    async fn resource_check(
        source: Option<Self::Source>,
        _version: Option<Self::Version>,
    ) -> Vec<Self::Version> {
        // init logger
        _ = env_logger::try_init();

        // validate and unwrap source
        let source = source.expect("source is required for the Github Issue resource");

        // return immediately with two sized vector if check step skip requested (e.g. source for out/put+create)
        if source.skip_check() {
            log::info!(
                "the check step will be skipped because 'skip_check' was set to true in source"
            );
            return vec![
                concourse::Version::new(String::from("open")),
                concourse::Version::new(String::from("closed")),
            ];
        }

        // create longer lifetime bindings
        let state_binding = source.state();
        // construct an issue...
        let gh_issue = github_issue::Issue::new(
            source.pat(),
            source.owner(),
            source.repo(),
            None,
            None,
            source.labels(),
            source.assignee(),
            source.number(),
            state_binding.as_deref(),
            source.milestone(),
        );
        // ...determine the action...
        let action = match source.number() {
            Some(_) => github_issue::Action::Read,
            None => github_issue::Action::List,
        };
        // ...and return the octocrab github issue
        let issue = match gh_issue.main(action).await {
            Ok(issue) => issue,
            Err(error) => {
                log::error!("{error}");
                panic!("the check step was unable to return a github issue from the source values");
            }
        };
        log::info!("the github issue information was successfully retrieved");

        // return one sized version vector if issue is open and two sized if closed
        match issue.state {
            octocrab::models::IssueState::Open => vec![concourse::Version::new(String::from("open"))],
            octocrab::models::IssueState::Closed => vec![concourse::Version::new(String::from("open")), concourse::Version::new(String::from("closed"))],
            _ => panic!("expected the github issue state to either be open or closed, and somehow it is something else")
        }
    }

    /// Dummies the in step as it performs no functionality.
    #[tokio::main]
    async fn resource_in(
        _source: Option<Self::Source>,
        _version: Self::Version,
        _params: Option<Self::InParams>,
        _output_path: &str,
    ) -> Result<
        concourse_resource::InOutput<Self::Version, Self::InMetadata>,
        Box<dyn std::error::Error>,
    > {
        // init logger
        _ = env_logger::try_init();

        log::info!("reminder: the in step is only to be used for an efficient check step with minimal overhead");
        Ok(concourse_resource::InOutput {
            version: concourse::Version::new(String::from("open")),
            metadata: None,
        })
    }

    /// Performs the out step for the resource. Creates a new Github issue based on the parameters.
    #[tokio::main]
    async fn resource_out(
        source: Option<Self::Source>,
        params: Option<Self::OutParams>,
        input_path: &str,
    ) -> concourse_resource::OutOutput<Self::Version, Self::OutMetadata> {
        // init logger
        _ = env_logger::try_init();

        // validate source and params
        let source = source.expect("source is required for the Github Issue resource");
        let params = params.expect("params is required for the Github Issue resource out/put step");

        // create longer lifetime bindings
        let state_binding = params.state();
        // construct an issue...
        let gh_issue = github_issue::Issue::new(
            source.pat(),
            source.owner(),
            source.repo(),
            params.title(),
            params.body(),
            params.labels(),
            params.assignees(),
            source.number(),
            state_binding.as_deref(),
            params.milestone(),
        );
        // ...determine the action...
        let action = match source.number() {
            Some(_) => github_issue::Action::Update,
            None => github_issue::Action::Create,
        };
        // ...and create the octocrab github issue
        let issue = match gh_issue.main(action).await {
            Ok(issue) => issue,
            Err(error) => {
                log::error!("{error}");
                panic!(
                    "the out/put step was unable to {} the associated github issue",
                    String::from(action)
                );
            }
        };
        log::info!("successful {} for the github issue", String::from(action));

        if source.number().is_none() {
            // store created issue number in file for subsequent check step
            let file_path = format!("{input_path}/issue_number.txt");
            std::fs::write(file_path, issue.number.to_string())
                .expect("issue number could not be written to {file_path}");
            log::info!("the issue number was stored in a file at '{input_path}/issue_number.txt'");
        }

        // return out step output
        concourse_resource::OutOutput {
            version: concourse::Version::new(String::from("open")),
            metadata: Some(concourse::OutMetadata::new(
                issue.number,
                issue.url,
                issue.title,
                issue.state,
                issue.labels,
                issue.assignees,
                issue.milestone,
                issue.created_at.to_string(),
                issue.updated_at.to_string(),
            )),
        }
    }
}

// helper functions if we need them
impl GithubIssue {}
// macro to populate the concourse functions
concourse_resource::create_resource!(GithubIssue);

#[cfg(test)]
mod tests {
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
        let source = serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Source>(
            source_input,
        )
        .expect("source could not be deserialized");
        let version =
            serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Version>(
                version_input,
            )
            .expect("version could not be deserialized");
        let version_vec = GithubIssue::resource_check(Some(source), Some(version));
        // the issue is closed so we expect a size two vec
        assert_eq!(
                version_vec,
                vec![concourse::Version::new(String::from("open")), concourse::Version::new(String::from("closed"))],
                "the resource_check did not return a two size vector of issue states for a closed issue",
            );
    }

    #[test]
    fn test_resource_check_list() {
        // validate basic check listing from mitodl/ol-infrastructure and filtering to issue 497
        // concourse pipeline json input
        let source_input = r#"
{
    "owner": "mitodl",
    "repo": "ol-infrastructure",
    "assignee": "pdpinch"
}"#;
        let version_input = r#"
{
    "state": "open"
}"#;
        // deserialize version and source for inputs
        let source = serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Source>(
            source_input,
        )
        .expect("source could not be deserialized");
        let version =
            serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Version>(
                version_input,
            )
            .expect("version could not be deserialized");
        let version_vec = GithubIssue::resource_check(Some(source), Some(version));
        // the issue is open so we expect a size one vec
        assert_eq!(
            version_vec,
            vec![concourse::Version::new(String::from("open"))],
            "the resource_check did not return a one size vector of issue states for an open issue",
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
    "state": "closed"
}"#;
        // deserialize version and source for inputs
        let source = serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Source>(
            source_input,
        )
        .expect("source could not be deserialized");
        let version =
            serde_json::from_str::<<GithubIssue as concourse_resource::Resource>::Version>(
                version_input,
            )
            .expect("version could not be deserialized");
        let version_vec = GithubIssue::resource_check(Some(source), Some(version));
        // skip check step requested so we expect size two vec
        assert_eq!(
                version_vec,
                vec![concourse::Version::new(String::from("open")), concourse::Version::new(String::from("closed"))],
                "the resource_check did not return a two size vector of issue states for a requested check skip",
            );
    }

    #[test]
    fn test_resource_in() {
        let in_output = GithubIssue::resource_in(
            None,
            concourse::Version::new(String::from("open")),
            None,
            "",
        )
        .unwrap();
        assert_eq!(
            in_output.version,
            concourse::Version::new(String::from("open")),
            "the resource in did not dummy the expected return version output",
        );
    }
}
