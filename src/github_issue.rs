//! # Github Issue
//!
//! `github_issue` is a minimal utility to create and update issues within Github.
use log;

use octocrab;

// allowed operations for github issue interactions
#[non_exhaustive]
#[derive(Copy, Clone)]
pub(super) enum Action {
    Create,
    List,
    Read,
    Update,
}

impl From<Action> for String {
    #[allow(unreachable_patterns)]
    fn from(action: Action) -> Self {
        match action {
            Action::Create => String::from("Create"),
            Action::List => String::from("List"),
            Action::Read => String::from("Read"),
            Action::Update => String::from("Update"),
            _ => String::from("Unknown"),
        }
    }
}

// convert string to IssueState or params::State without trait implementations because not allowed (no impl Into<octocrab::models::IssueState> for &str)
fn str_to_issue_state(param: &str) -> Result<octocrab::models::IssueState, &str> {
    match param {
        "open" => Ok(octocrab::models::IssueState::Open),
        "closed" => Ok(octocrab::models::IssueState::Closed),
        "all" => {
            log::warn!(
                "all was specified for issue state, and this can only be utilized with issue filtering, and not updating"
            );
            log::warn!("the issue state will be reset to 'open'");
            Ok(octocrab::models::IssueState::Open)
        }
        &_ => Err("the issue state must be either open or closed"),
    }
}

fn str_to_params_state(param: &str) -> Result<octocrab::params::State, &str> {
    match param {
        "open" => Ok(octocrab::params::State::Open),
        "closed" => Ok(octocrab::params::State::Closed),
        "all" => Ok(octocrab::params::State::All),
        &_ => Err("the issue state must be either open, closed, or all"),
    }
}

// struct for general interfacing with module
// the types correspond to octocrab when not advantageous otherwise
#[derive(Eq, PartialEq, Debug)]
pub(super) struct Issue<'issue> {
    // client and issues: OctocrabBuilder and issues::IssueHandler
    pat: Option<&'issue str>,
    owner: &'issue str,
    repo: &'issue str,
    // create and update (octocrab update expects AsRef<str> instead of String and AsRef<[String]> instead of Vec<String>)
    title: Option<&'issue str>,
    body: Option<&'issue str>,
    labels: Option<Vec<String>>,
    assignees: Option<Vec<String>>,
    // read and update
    number: Option<u64>,
    // update octocrab::models::IssueState and list octocrab::params::State
    state: Option<&'issue str>,
    // create, list, and update
    milestone: Option<u64>,
}

impl<'issue> Issue<'issue> {
    /// Constructor for the Config struct. Contains all of the members necessary for instantiating a client and performing an action.
    /// ```
    /// let gh_issue = Issue::new(None, String::from("my_org"), String::from("my_repo"), None, None, None, None, Some(100), None);
    /// ```
    pub(super) fn new(
        pat: Option<&'issue str>,
        owner: &'issue str,
        repo: &'issue str,
        title: Option<&'issue str>,
        body: Option<&'issue str>,
        labels: Option<Vec<String>>,
        assignees: Option<Vec<String>>,
        number: Option<u64>,
        state: Option<&'issue str>,
        milestone: Option<u64>,
    ) -> Self {
        // return instantiated github issue
        Self {
            pat,
            owner,
            repo,
            title,
            body,
            labels,
            assignees,
            number,
            state,
            milestone,
        }
    }

    /// Instantiate a reusable Octocrab issues object with input authentication, and an input owner and repo.
    /// ```
    /// let issue = gh_issue.main(Action::Read).await?;
    /// ```
    pub(super) async fn main<'octo>(
        &self,
        action: Action,
    ) -> Result<octocrab::models::issues::Issue, &str> {
        // instantiate client and issues
        let client = match &self.pat {
            Some(pat) => octocrab::Octocrab::builder()
                .personal_token(pat.to_string())
                .build()
                .unwrap_or({
                    log::warn!("could not authenticate client with Personal Access Token");
                    log::warn!("will continue with unauthenticated client");
                    octocrab::Octocrab::default()
                }),
            None => octocrab::Octocrab::default(),
        };
        log::debug!("built octocrab client");
        let issues = client.issues(self.owner, self.repo);
        log::debug!("built octocrab issues");
        // execute action and assign returned issue
        let issue = match action {
            // create an issue
            Action::Create => self.create(issues).await?,
            // list issues and filter to one issue
            Action::List => self.list(issues).await?,
            // read an issue state
            Action::Read => self.read(issues).await?,
            // update an issue
            Action::Update => self.update(issues).await?,
        };
        log::debug!("issue interfacing completed");

        Ok(issue)
    }

    // create a github issue according to configuration
    async fn create<'octo>(
        &self,
        issues: octocrab::issues::IssueHandler<'octo>,
    ) -> Result<octocrab::models::issues::Issue, &str> {
        // validate a title was specified
        match self.title {
            // title specified
            Some(title) => {
                // build the issue
                // createissuebuilder milestone type is impl Into<Option<u64>> so we can build it immediately
                let mut issue = issues.create(title).milestone(self.milestone);
                // ... with optional parameters
                if self.body.is_some() {
                    issue = issue.body(self.body.unwrap());
                }
                if self.labels.is_some() {
                    issue = issue.labels(self.labels.clone());
                }
                if self.assignees.is_some() {
                    issue = issue.assignees(self.assignees.clone());
                }
                if self.milestone.is_some() {
                    issue = issue.milestone(self.milestone);
                }

                log::debug!("creating issue");
                // send and await the issue
                match issue.send().await {
                    // return created issue
                    Ok(issue) => return Ok(issue),
                    // issue could not be created
                    Err(error) => {
                        log::error!("the issue could not be created");
                        log::error!("{error}");
                        return Err("issue not created");
                    }
                }
            }
            // title unspecified
            None => {
                log::error!("a title was not specified, and so an issue could not be created");
                return Err("title unspecified");
            }
        }
    }

    // read a github issue according to configuration
    async fn read<'octo>(
        &self,
        issues: octocrab::issues::IssueHandler<'octo>,
    ) -> Result<octocrab::models::issues::Issue, &str> {
        // validate an issue number was specified
        match self.number {
            // issue number specified
            Some(number) => {
                log::debug!("reading issue");
                // retrieve the issue with the handler
                match issues.get(number).await {
                    Ok(issue) => return Ok(issue),
                    // issue number probably does not exist, or some other error
                    Err(error) => {
                        log::error!(
                            "the issue number {number} could not be retrieved from the repository"
                        );
                        log::error!("{error}");
                        return Err("unknown issue");
                    }
                };
            }
            // issue number unspecified
            None => {
                log::error!(
                    "an issue number was not specified, and so its state cannot be retrieved"
                );
                return Err("issue number unspecified");
            }
        }
    }

    // list github issues according to configuration
    async fn list<'octo>(
        &self,
        issues: octocrab::issues::IssueHandler<'octo>,
    ) -> Result<octocrab::models::issues::Issue, &str> {
        // declare labels and assignees at higher scope so values are not dropped before borrow is used later in function
        let labels: Vec<String>;

        // build the issue pages
        let mut issue_page = issues.list();
        // ... with optional parameters
        if self.state.is_some() {
            // convert str state to params state
            let params_state = str_to_params_state(self.state.unwrap())?;
            issue_page = issue_page.state(params_state);
        }
        if self.milestone.is_some() {
            issue_page = issue_page.milestone(self.milestone.unwrap());
        }
        if self.assignees.is_some() {
            // assert only one assignee in assignees
            let num_assignees = self.assignees.as_ref().unwrap().len();
            if num_assignees != 1 {
                log::error!("list action attempted with other than one assignee: {num_assignees}");
                log::error!(
                    "this is an error with custom resource frontend and backend interfacing, and should be reported"
                );
                return Err("multiple assignees and list action");
            }
            // assign value of only assignee and use for assignee filter
            let assignee = &self.assignees.as_ref().unwrap()[0][..];
            issue_page = issue_page.assignee(assignee);
        }
        if self.labels.is_some() {
            labels = self.labels.clone().unwrap();
            issue_page = issue_page.labels(&labels[..]);
        }

        log::debug!("listing issues");
        // send and await the issue page
        let page = match issue_page.send().await {
            // return issue pages
            Ok(page) => page,
            // issues probably do not exist with given filters, or some other error
            Err(error) => {
                log::error!(
                    "the issues with the given filters could not be retrieved from the repository"
                );
                log::error!("{error}");
                return Err("unknown issues");
            }
        };
        // items member is Page<T> into Vec<T> so we can iter
        let vec_issues = page.items;
        // ensure only one issue exists in octocrab::Page<octocrab::models::issues::Issue>
        match vec_issues.len() {
            1 => return Ok(vec_issues[0].clone()),
            _ => {
                let num = vec_issues.len();
                log::error!("expected only one issue to be returned from filtered list");
                log::error!("actual number of issues returned was {num}");
                return Err("unexpected number of issues");
            }
        }
    }

    // update a github issue according to configuration
    async fn update<'octo>(
        &self,
        issues: octocrab::issues::IssueHandler<'octo>,
    ) -> Result<octocrab::models::issues::Issue, &str> {
        // validate an issue number was specified
        match self.number {
            // issue number specified
            Some(number) => {
                // declare labels and assignees at higher scope so values are not dropped before borrow is used later in function
                let (labels, assignees): (Vec<String>, Vec<String>);

                // build the issue
                let mut issue = issues.update(number);
                // ... with optional parameters
                if self.title.is_some() {
                    issue = issue.title(self.title.as_ref().unwrap());
                }
                if self.body.is_some() {
                    issue = issue.body(self.body.as_ref().unwrap());
                }
                if self.labels.is_some() {
                    labels = self.labels.clone().unwrap();
                    issue = issue.labels(&labels[..]);
                }
                if self.assignees.is_some() {
                    assignees = self.assignees.clone().unwrap();
                    issue = issue.assignees(&assignees[..]);
                }
                if self.state.is_some() {
                    // convert str state to issue_state
                    let issue_state = str_to_issue_state(self.state.unwrap())?;
                    issue = issue.state(issue_state);
                }
                if self.milestone.is_some() {
                    issue = issue.milestone(self.milestone.unwrap());
                }

                log::debug!("updating issue");
                // send and await the issue
                match issue.send().await {
                    // return updated issue
                    Ok(issue) => return Ok(issue),
                    // issue number probably does not exist, or some other error
                    Err(error) => {
                        log::error!("the issue number {number} could not be updated");
                        log::error!("{error}");
                        return Err("issue not updated");
                    }
                }
            }
            // issue number unspecified
            None => {
                log::error!(
                    "an issue number was not specified, and so an issue could not be updated"
                );
                return Err("issue number unspecified");
            }
        }
    }
}

#[cfg(test)]
mod tests;
