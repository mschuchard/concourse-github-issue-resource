# Concourse Github Issue Resource

A [concourse-ci](https://concourse-ci.org) resource for interacting with [Github Issues](https://docs.github.com/en/issues/tracking-your-work-with-issues).

This resource's container image is currently hosted at [matthewschuchard/concourse-github-issue-resource](https://hub.docker.com/repository/docker/matthewschuchard/concourse-github-issue-resource) for usage within Concourse.

This repository and project is based on the work performed for [MITODL](https://github.com/mitodl/concourse-github-issue-resource), and now serves as an upstream for the project hosted within that organization. Accordingly it maintains the BSD-3 license with copyright notice.

## Behavior

### `source`: designates the Github repository, issue number, and personal access token

**parameters**
- `pat`: _required/optional_ The personal access token for authentication and authorization. If anonymous read and write for Issues is permitted, then this is optional for the `check` and `out` steps. Otherwise it is required for private repos, or any other situation where anonymous read and write for Issues is not authorized.

- `owner`: _required_ The owner of the target repository expressed as either a user or organization.

- `repo`: _required_ The Github repository with the issue tracker in which to read and/or write issues.

- `skip_check`: _optional_ A boolean that signifies whether to skip the `check` step or not. This is primarily useful for situations where it is known that a specified issue does not exist, and instead must be created during `out`. The default value is `false`.

- `trigger`: _optional_ The issue state that causes a trigger during the `check` step. This can be either `open` or `closed`. The default value is `closed`.

- `number`: _optional/required_ The issue number to read during the `check` step for triggering Concourse pipelines based on the issue state, or for updating during the `out` step. If this is omitted then instead a list operation with filters (i.e. "search") occurs to determine the issue during the `check` step, and a create operation during the `out` step. Therefore this is implicitly required if an issue update is desired as a new issue creation attempt will occur during `out` otherwise.

The following parameters are for filtering from a list of issues to one issue (i.e. "search") during the `check` step, and therefore their values are ignored when an input value is specified for the `number` parameter.

- `state`: _optional_ The current state of the searched issue. This can be either `open`, `closed`, or `all`.

- `milestone`: _optional_ The numeric ID of the milestone associated with the searched issue.

- `assignee`: _optional_ The user name of the assignee for the searched issue.

- `labels`: _optional_ The list of labels for the searched issue.

### `version`: designates the Github issue state

**parameters**
- `version`: _optional_ The state of the issue specified in the `source` expressed as the enum `closed` or `open` (note the states' serialization is implemented by Octocrab to be lowercase strings). This is an output only and is ignored as an input parameter.

```yaml
version:
  state: <issue state>
```

### `check`: returns size two list for closed Github issues and size one list for open Github issues

The `check` step determines the state of the specified Github issue. If the state is `closed` (default behavior; otherwise `trigger` source parameter value) then the returned list of versions is size two. If the state is `open` (default behavior; otherwise NOT `trigger` source parameter value) then the returned list of versions is size one. This is specifically to trigger pipelines based on the issue state (`closed` triggers and `open` does not trigger by default; otherwise customized by `trigger` source parameter value) because it simulates a delta of versions for `closed` and not `open` (default). The actual returns are the following (note the states' serialization is implemented by Octocrab to be lowercase strings):

trigger:
```json
[{"state":"open"},{"state":"closed"}]
```

no trigger:
```json
[{"state":"open"}]
```

### `in`: currently unused

This ignores any inputs and quickly dummies outputs, and therefore is primarily useful for executing an efficient `check` step with minimal overhead.

### `out`: creates or updates a Github issue

The `out` step updates or creates a Github issue according to the input parameters below. The number of the created Github issue is written to a file at `/opt/resource/issue_number.txt` so that it can be re-used later in the build (especially for a subsequent `check` step to trigger Concourse steps based on the status of the Github issue created during this step).

Recall that the parameter which determines whether a create or update operation occurs during this step is `source.number` (update when a specific existing issue number is specified; otherwise create).

The metadata output from this step contains the number, url, title, state, labels, assignees, milestone, created time, and last updated time for the issue.

- `title`: _optional/required_ The title of the written Github issue (required for new issue).

- `body`: _optional_ The body of the written Github issue.

- `labels`: _optional_ The list of labels for the written Github issue.

- `assignees`: _optional_ The list of assignees for the written Github issue.

- `milestone`: _optional_ The milestone numeric ID to associate with the written Github issue.

- `state`: _optional_ The desired state of the updated issue. This can be either `open` or `closed`.

### Metadata

Below is the general structure of the generated Concourse metadata. Note that the `labels` and `assignees`  keys will not exist if their value is empty, and the `milestone` key will not exist if its value is `null`.

```json
{
  "number": "issue number",
  "url": "issue url",
  "title": "issue title",
  "state": "open|closed",
  "labels": ["issue labels (see below doc link)"],
  "assignees": ["issue assignees (see below doc link)"],
  "milestone": "issue milestone (see below doc link)",
  "created": "issue creation time",
  "updated": "issue updated time"
}
```

Octocrab doc links for model serialization:  
[Label](https://docs.rs/octocrab/latest/octocrab/models/struct.Label.html)  
[Assignee](https://docs.rs/octocrab/latest/octocrab/models/struct.Author.html)  
[Milestone](https://docs.rs/octocrab/latest/octocrab/models/struct.Milestone.html)

## Example

```yaml
resource_types:
- name: github_issue
  type: docker-image
  source:
    repository: matthewschuchard/concourse-github-issue-resource:1.2
    tag: latest

resources:
- name: github-issue
  type: github-issue
  source:
    pat: abcdefg12345!
    owner: mitodl
    repo: ol-infrastructure
    skip_check: true
- name: github-issue-check
  type: github-issue
  source:
    owner: mitodl
    repo: ol-infrastructure
    number: 1

jobs:
- name: do something
  plan:
  - get: my-code
  - task: something cool
    file: foo.yml
    on_failure:
      put: github-issue
      params:
        title: concourse failed
        body: go fix it
        assignees:
        - my_user
        - your_user
  - get: githhub-issue-check
```

## Contributing
Code should pass all unit and acceptance tests. New features should involve new unit tests.

Please consult the GitHub Project for the current development roadmap.
