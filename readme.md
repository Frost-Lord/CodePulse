# CodePulse

CodePulse is a dynamic, Rust-based tool designed to streamline the process of keeping your local code repositories synchronized with their counterparts on GitHub. Utilizing efficient, asynchronous operations, CodePulse offers real-time monitoring and updates, ensuring your projects are always up-to-date with the latest changes and contributions.

## Features

- **Real-Time Updates**: Automatically fetches the latest updates from GitHub, ensuring your codebase remains current without manual oversight.
- **Project Management**: Easily manage multiple projects, allowing for seamless updates across your entire development landscape.
- **Customizable Notifications**: Get notified about changes in your repositories with customizable alerts, tailored to fit your workflow.
- **Efficiency at Scale**: Designed to handle numerous repositories efficiently, making it suitable for individual developers and large teams alike.
- **Secure**: Implements best practices for authentication and secure communication with GitHub's API, safeguarding your code and credentials.

# Run the code:
```bash
cargo build --release && cargo run --release
```

```json
{
  "projects": [
    {
      "github_url": "Frost-Lord/HaskMate",
      "name": "HaskMate",
      "path": "./production/HaskMate",
      "updated_at": "2023-12-08T15:13:52Z" // The script will auto update it || e.g don't need to set updated_at as the program will manage it
    }
  ],
  "settings": {
    "intivial": "15" // optional
  }
}
```