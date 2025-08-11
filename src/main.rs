use env_logger;
use log;

use concourse_resource::*;
use octocrab::models::IssueState;

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
                concourse::Version::new(IssueState::Open),
                concourse::Version::new(IssueState::Closed),
            ];
        }

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
            source.state(),
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
        log::info!(
            "the github issue information was successfully retrieved for number {}",
            issue.number
        );

        // return two sized version vector if issue state matches trigger, and one sized if otherwise
        if issue.state == source.trigger() {
            vec![
                concourse::Version::new(IssueState::Open),
                concourse::Version::new(IssueState::Closed),
            ]
        } else {
            vec![concourse::Version::new(IssueState::Open)]
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

        log::info!(
            "reminder: the in step is only to be used for an efficient check step with minimal overhead"
        );
        log::info!(
            "there is no actual functionality for the in step, and the version and metadata are dummied"
        );
        Ok(concourse_resource::InOutput {
            version: concourse::Version::new(IssueState::Open),
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
            params.state(),
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
        log::info!(
            "successful {} for the github issue number {}",
            String::from(action),
            issue.number
        );

        if source.number().is_none() {
            // store created issue number in file for subsequent check step
            let file_path = format!("{input_path}/issue_number.txt");
            match std::fs::write(&file_path, issue.number.to_string()) {
                Ok(_) => log::info!(
                    "the issue number was stored in a file at '{input_path}/issue_number.txt'"
                ),
                Err(error) => {
                    log::warn!(
                        "issue number could not be written to {file_path}, and issue number will therefore not be available for subsequent check step"
                    );
                    log::warn!("error: {error}");
                }
            }
        }

        // return out step output
        concourse_resource::OutOutput {
            version: concourse::Version::new(IssueState::Open),
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
mod tests;
