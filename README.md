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

- `skip_check`: _optional_ A boolean that signifies whether to skip the `check` step or not. This is primarily useful for situations where it is known that a specified issue does not exist, and instead must be created during `out`.

- `number`: _optional_ The issue number to read during the `check` step for triggering Concourse pipelines based on the issue state, or for updating during the `out` step. If this is omitted then instead a list operation with filters (i.e. "search") occurs to determine the issue during the `check` step, and a create operation during the `out` step.

The following parameters are for filtering from a list of issues to one issue (i.e. "search") during the `check` step, and therefore their values are ignored when an input value is specified for the `number` parameter.

- `state`: _optional_ The current state of the searched issue. This can be either `Open`, `Closed`, or `All`.

- `milestone`: _optional_ The numeric ID of the milestone associated with the searched issue.

- `assignee`: _optional_ The user name of the assignee for the searched issue.

- `labels`: _optional_ (value currently ignored) The list of labels for the searched issue.

### `version`: designates the Github issue state

**parameters**
- `version`: _optional_ The state of the issue specified in the `source` expressed as the enum `Closed` or `Open` converted to a string. This is an output only and is ignored as an input parameter.

```yaml
version:
  state: <issue state>
```

### `check`: returns size two list for Closed Github issues and size one list for Open Github issues

The `check` step determines the state of the specified Github issue. If the state is `Closed` then the returned list of versions is size two. If the state is `Open` then the returned list of versions is size one. This is specifically to trigger pipelines based on the issue state (`Closed` triggers and `Open` does not trigger) because it simulates a delta of versions for `Closed` and not `Open`. The actual returns are the following:

Closed:
```json
[{"state":"Open"},{"state":"Closed"}]
```

Open:
```json
[{"state":"Open"}]
```

### `in`: currently unused

This ignores any inputs and quickly dummies outputs, and therefore is primarily useful for executing an efficient `check` step with minimal overhead.

### `out`: creates a Github issue

The `out` step updates or creates a Github issue according to the input parameters below. The number of the created Github issue is written to a file at `/opt/resource/issue_number.txt` so that it can be re-used later in the build (especially for a subsequent `check` step to trigger Concourse steps based on the status of the Github issue created during this step).

Recall that the parameter which determines whether a create or update operation occurs during this step is `source.number`.

The metadata output from this step contains the number, url, title, state, labels, assignees, milestone, created time, and last updated time for the issue.

- `title`: _required_ The title of the written Github issue.

- `body`: _optional_ The body of the written Github issue.

- `labels`: _optional_ (value currently ignored for update) The list of labels for the written Github issue.

- `assignees`: _optional_ (value currently ignored for update) The list of assignees for the written Github issue.

- `milestone`: _optional_ The milestone numeric ID to associate with the written Github issue.

- `state`: _optional_ The desired state of the updated issue. This can be either `Open` or `Closed`.

## Example

```yaml
resource_types:
- name: github_issue
  type: docker-image
  source:
    repository: matthewschuchard/concourse-github-issue-resource:1.0
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
