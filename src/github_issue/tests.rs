use super::*;

#[test]
fn test_action_to_string() {
    // validates ToString trait impl for action enum
    assert_eq!(String::from(Action::Create), String::from("Create"));
    assert_eq!(String::from(Action::Read), String::from("Read"));
}

#[test]
fn test_str_to_issue_state() {
    // validates issue open and closed conversions
    assert_eq!(
        str_to_issue_state("open"),
        Ok(octocrab::models::IssueState::Open),
        "failed to convert open str to open enum"
    );

    assert_eq!(
        str_to_issue_state("closed"),
        Ok(octocrab::models::IssueState::Closed),
        "failed to convert closed str to closed enum"
    );
}
#[test]
fn test_str_to_params_state() {
    // octocrab::params::State does not implement Eq
    str_to_params_state("open")
        .expect("could not convert \"open\" to octocrab::params::State::open");
    str_to_params_state("closed")
        .expect("could not convert \"closed\" to octocrab::params::State::Closed");
    str_to_params_state("all").expect("could not convert \"all\" to octocrab::params::State::all");
    assert_eq!(
        str_to_params_state("foo").unwrap_err(),
        "the issue state must be either open, closed, or all",
    )
}

#[test]
fn test_issue_new() {
    // validates basic read constructor
    assert_eq!(
        Issue::new(
            None,
            "my_org",
            "my_repo",
            None,
            None,
            None,
            None,
            Some(100),
            None,
            None
        ),
        Issue {
            pat: None,
            owner: "my_org",
            repo: "my_repo",
            title: None,
            body: None,
            labels: None,
            assignees: None,
            number: Some(100),
            state: None,
            //params_state: None,
            milestone: None
        },
        "failed to construct Issue for read"
    );

    // validate basic create constructor
    assert_eq!(
        Issue::new(
            None,
            "my_org",
            "my_repo",
            Some("my issue"),
            Some("my body"),
            Some(vec![String::from("label")]),
            Some(vec![String::from("assignee")]),
            None,
            None,
            None
        ),
        Issue {
            pat: None,
            owner: "my_org",
            repo: "my_repo",
            title: Some("my issue"),
            body: Some("my body"),
            labels: Some(vec![String::from("label")]),
            assignees: Some(vec![String::from("assignee")]),
            number: None,
            state: None,
            //params_state: None,
            milestone: None
        },
        "failed to construct Issue for create"
    );
}

#[test]
fn test_issue_main_read() {
    // validate issue returned when read from main
    let test = async {
        let gh_issue = Issue::new(
            None,
            "mitodl",
            "ol-infrastructure",
            None,
            None,
            None,
            None,
            Some(100),
            None,
            None,
        );
        let issue = gh_issue.main(Action::Read).await;
        assert_eq!(
            issue.unwrap().state,
            octocrab::models::IssueState::Closed,
            "hundredth issue from mitodl/ol-infrastructure not read and returned correctly",
        );
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_issue_main_list() {
    // validate one issue of multiple listed returned from main
    let test = async {
        let gh_issue = Issue::new(
            None,
            "mitodl",
            "ol-infrastructure",
            None,
            None,
            None,
            Some(vec![String::from("pdpinch")]),
            None,
            Some("closed"),
            Some(3),
        );
        let issue = gh_issue.main(Action::List).await;
        assert_eq!(
            issue.unwrap().number,
            833,
            "single issue #833 from multiple listed from mitodl/ol-infrastructure not returned correctly",
        );
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}

#[test]
fn test_errors() {
    // validate errors
    let test = async {
        let gh_issue = Issue::new(
            None,
            "mitodl",
            "ol-infrastructure",
            None,
            None,
            None,
            Some(vec![String::from("foo"), String::from("bar")]),
            None,
            None,
            None,
        );
        let gh_issue_four = Issue::new(
            None,
            "mitodl",
            "ol-infrastructure",
            None,
            None,
            None,
            Some(vec![String::from("blarghmatey")]),
            None,
            None,
            None,
        );
        // validate title required for create error
        let issue = gh_issue.main(Action::Create).await;
        assert_eq!(
            issue,
            Err("title unspecified"),
            "attempted create without specified title did not error expectedly",
        );
        // validate issue number required for read
        let issue_two = gh_issue.main(Action::Read).await;
        assert_eq!(
            issue_two,
            Err("issue number unspecified"),
            "attempted read without specified number did not error expectedly",
        );
        // validate only one assignee for list
        let issue_three = gh_issue.main(Action::List).await;
        assert_eq!(
            issue_three,
            Err("multiple assignees and list action"),
            "attempted list with multiple assignees did not error expectedly",
        );
        // validate only one issue returned for list
        let issue_four = gh_issue_four.main(Action::List).await;
        assert_eq!(
            issue_four,
            Err("unexpected number of issues"),
            "attempted list with multiple issues returned did not error expectedly",
        );
        // validate issue number required for update
        let issue_five = gh_issue.main(Action::Update).await;
        assert_eq!(
            issue_five,
            Err("issue number unspecified"),
            "attempted update without specified number did not error expectedly",
        );
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test);
}
