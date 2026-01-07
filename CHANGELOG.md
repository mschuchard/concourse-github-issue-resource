### 1.3.0
- Add issue lock/unlock functionality.
- Add `creator` parameter to issue search filter.

### 1.2.2
- Warn instead of error when issue number file write fails.
- Optimize issue operation customization.
- Improve `source` validation.

### 1.2.1
- Enable Github issue state `check` step trigger customization.
- Convert Concourse model struct types to Octocrab models.
- Enable `labels` in issue update and list.
- Enable `assignees` in issue update.

### 1.2.0
- Update issue state inputs to be lowercase for Octocrab consistency.
- Backup to default client if PAT fails.

### 1.1.1
- Update version outputs to be consistent with Octocrab serialization of state enums.
- Update title parameter to be optional input in `out` step.

### 1.1.0
- Enable issue updating during out/put step.

### 1.0.1
- Add more Metadata to out/put step.
- Enable state parameter in check and out steps.

### 1.0.0
- Initial release.
