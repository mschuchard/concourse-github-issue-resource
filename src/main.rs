use concourse_resource::*;

mod concourse;
mod github_issue;

struct GithubIssue {}

impl concourse_resource::Resource for GithubIssue {
    // implementtations for inputs and outputs
    type Source = concourse::Source;
    type Version = concourse::Version;
    type InParams = concourse_resource::Empty;
    type InMetadata = concourse_resource::Empty;
    type OutParams = concourse::OutParams;
    type OutMetadata = concourse::OutMetadata;

    // implementations for steps
    /// Performs the check step for the resource. Returns a single sized vector of version of state string if the input issue is Open (no trigger), and a two sized vector of version of state string if the input issue is closed (trigger). For convenience and standardization the former return is "Open", and the latter is "Open" and "Closed".
    #[tokio::main]
    async fn resource_check(
        source: Option<Self::Source>,
        _version: Option<Self::Version>,
    ) -> Vec<Self::Version> {
        // validate and unwrap source
        let source = match source {
            Some(source) => source,
            None => panic!("source is required for the Github Issue resource"),
        };

        // construct an issue...
        let gh_issue = github_issue::Issue::new(
            source.pat(),
            source.owner(),
            source.repo(),
            None,
            None,
            source.labels(),
            source.assignees(),
            source.number(),
            None, //source.state(),
            source.milestone(),
        );
        // ...determine the action...
        let action = match source.number() {
            Some(_) => github_issue::Action::Read,
            None => github_issue::Action::List,
        };
        // ...and read the octocrab github issue
        let issue = match gh_issue.main(action).await {
            Ok(issue) => issue,
            Err(error) => {
                println!("{error}");
                panic!("the check step was unable to read the specified github issue number");
            }
        };

        // return one sized version vector if issue is open and two sized if closed
        match issue.state {
            octocrab::models::IssueState::Open => vec![concourse::Version::new(String::from("Open"))],
            octocrab::models::IssueState::Closed => vec![concourse::Version::new(String::from("Open")), concourse::Version::new(String::from("Closed"))],
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
        Ok(concourse_resource::InOutput {
            version: concourse::Version::new(String::from("Open")),
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
        // validate source and params
        let source = match source {
            Some(source) => source,
            None => panic!("source is required for the Github Issue resource"),
        };
        let params = match params {
            Some(params) => params,
            None => panic!("params is required for the Github Issue resource out/put step"),
        };

        // construct an issue...
        let gh_issue = github_issue::Issue::new(
            source.pat(),
            source.owner(),
            source.repo(),
            Some(params.title()),
            params.body(),
            params.labels(),
            params.assignees(),
            None,
            None,
            params.milestone(),
        );
        // ...and create the octocrab github issue
        let issue = match gh_issue.main(github_issue::Action::Create).await {
            Ok(issue) => issue,
            Err(error) => {
                println!("{error}");
                panic!("the out/put step was unable to create the associated github issue");
            }
        };

        // store issue number in file for subsequent check step
        let file_path = format!("{input_path}/issue_number.txt");
        std::fs::write(file_path, issue.number.to_string())
            .expect("issue number could not be written to {file_path}");

        // return out step output
        concourse_resource::OutOutput {
            version: concourse::Version::new(String::from("Open")),
            metadata: Some(concourse::OutMetadata::new(
                issue.number,
                issue.labels,
                issue.assignees,
                issue.milestone,
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
    fn test_resource_check() {
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
    "state": "Closed"
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
                vec![concourse::Version::new(String::from("Open")), concourse::Version::new(String::from("Closed"))],
                "the resource_check did not return a two size vector of issue states for a closed issue",
            );
    }

    #[test]
    fn test_resource_in() {
        let in_output = GithubIssue::resource_in(
            None,
            concourse::Version::new(String::from("Open")),
            None,
            "",
        )
        .unwrap();
        assert_eq!(
            in_output.version,
            concourse::Version::new(String::from("Open")),
            "the resource in did not dummy the expected return version output",
        );
    }
}
