# Concourse Github Issue Resource

A [concourse-ci](https://concourse-ci.org) resource for interacting with [Github Issues](https://docs.github.com/en/issues/tracking-your-work-with-issues).

This repository and project is based on the work performed for [MITODL](https://github.com/mitodl/concourse-github-issue-resource), and now serves as an upstream for the project hosted within that organization. Accordingly it maintains the BSD-3 license with copyright notice.

## Behavior

### `source`: designates the Github repository, issue number, and personal access token

**parameters**
- `pat`: _required/optional_ The personal access token for authentication and authorization. If anonymous read is permitted, then this is optional for the `check` step. Otherwise it is required for `check` with private repos and `out` with probably all repos.

- `owner`: _required_ The owner of the target repo expressed as either a user or organization.

- `repo`: _required_ The Github repository with the issue tracker in which to read and/or create issues.

- `number`: _optional_ The issue number to read during the `check` step for triggering Concourse pipelines based on the issue state. If this is omitted then the `check` step is skipped.

- `milestone`: _optional_ currently not interfaced between frontend and backend

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

This ignores any inputs and quickly dummies outputs, and therefore is primarily useful for enforcing a useful `check` step with minimal overhead.

### `out`: creates a Github issue

The `out` step creates a Github issue according to the input parameters below. The number of the created Github issue is written to a file at `/opt/resource/issue_number.txt` so that it can be re-used later in the build (especially for a subsequent `check` step to trigger based on the status of the Github issue created during this step).

- `title`: _required_ The title of the Github issue.

- `body`: _optional_ The body of the Github issue.

- `labels`: _optional_ A list of labels for the Github issue.

- `assignees`: _optional_ A list of assignees for the Github issue.

- `milestone`: _optional_ The milestone number to associate with the issue during creation.

## Example

```yaml
resource_types:
- name: github_issue
  type: docker-image
  source:
    repository: mitodl/concourse-github-isse-resource
    tag: latest

resources:
- name: github-issue
  type: github-issue
  source:
    pat: abcdefg12345!
    owner: mitodl
    repo: ol-infrastructure
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
