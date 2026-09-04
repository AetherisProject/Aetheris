# GitHub Project Configuration

## Project: Aetheris Roadmap

Since the `gh` token lacks the `project` scope, the project must be created manually at:
https://github.com/users/merlin-tribukait/projects/new

### Project Settings

- **Title:** Aetheris Roadmap
- **Description:** The Secrets Operating System — development roadmap and tracking
- **Template:** Board (Kanban)
- **Visibility:** Public

### Columns (Board View)

| Column | Description |
|--------|-------------|
| 📋 Backlog | Unprioritized issues |
| 📅 Ready | Prioritized and ready for work |
| 🔨 In Progress | Actively being worked on |
| 👀 In Review | Under review / testing |
| ✅ Done | Completed |

### Views

#### Board View (Default)
- Group by: Status
- Columns as above

#### Table View
- Fields: Title, Status, Milestone, Labels, Assignees, Linked Pull Requests

#### Roadmap View
- Group by: Milestone
- Timeline: Quarterly

### Milestones

| Milestone | Description | Due Date |
|-----------|-------------|----------|
| Phase 0: VaultEngine Core | Complete Rust core modules | 2026-12-31 |
| Phase 1: MVP Everywhere | All platforms can use the vault | 2027-03-31 |
| Phase 2: Platform Powers | SSH terminal, autofill, admin | 2027-06-30 |
| Phase 3: Advanced Security | Shamir, deniability, post-quantum | 2027-09-30 |
| Phase 4: Polish + Network | Languages, sharing, marketplace | 2027-12-31 |

### Labels

| Label | Color | Description |
|-------|-------|-------------|
| enhancement | #a2eeef | New feature |
| bug | #d73a4a | Bug fix |
| security | #d73a4a | Security-related |
| crypto | #0075ca | Cryptography |
| vault | #5319e7 | Vault management |
| ssh | #0e8a16 | SSH terminal |
| sync | #fbca04 | Sync engine |
| ui | #c5def5 | User interface |
| docs | #0075ca | Documentation |

### Automations

- **When PR is merged** → move linked issues to "Done"
- **When issue is added to project** → set status to "Backlog"
- **When PR is opened** → move linked issues to "In Review"

### Adding Issues to Project

Issues have been created and linked to milestones. To add them to the project:

1. Open the project at https://github.com/users/merlin-tribukait/projects
2. Click "Add item" → "Issues"
3. Select all issues
4. Set status field values

### Required Token Scopes

To create projects programmatically, the token needs:
- `project` (Full control of projects)

To update the token scope:
1. Generate a new token at https://github.com/settings/tokens
2. Select `project` scope
3. Update `gh` auth with the new token
