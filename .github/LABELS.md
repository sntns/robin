# GitHub Labels Setup

This directory contains a `labels.yml` file that defines a comprehensive set of labels for organizing issues and pull requests in the robin repository.

## Using the Labels

### Manual Setup

You can manually create these labels in your GitHub repository by going to:
`https://github.com/sntns/robin/labels`

Then create each label with the specified name, color, and description from the `labels.yml` file.

### Automated Setup

For automated label synchronization, you can use tools like:

1. **[github-label-sync](https://github.com/Financial-Times/github-label-sync)**
   ```bash
   npm install -g github-label-sync
   github-label-sync --access-token YOUR_TOKEN sntns/robin .github/labels.yml
   ```

2. **[GitHub Labeler Action](https://github.com/marketplace/actions/github-labeler)**
   Add to your workflow to automatically sync labels on push

## Label Categories

The labels are organized into several categories:

- **Type labels**: Categorize by issue/PR type (bug, enhancement, documentation, question)
- **Priority labels**: Indicate urgency (high, medium, low)
- **Status labels**: Workflow status (good first issue, help wanted, wontfix, duplicate)
- **Component labels**: Specific to robin components (cli, library, netlink, api)
- **Technology labels**: For indexing and discovery (rust, networking, mesh-networking, batman-adv, linux)
- **Development labels**: Development workflow (dependencies, security, testing, ci/cd, performance)

## Benefits for Repository Indexing

These labels help the repository be properly indexed and discovered by:
- Making it easier for users to find relevant issues and understand the project scope
- Enabling GitHub's topic and search features to categorize the repository
- Helping contributors identify areas where they can contribute
- Providing clear organization for project management
