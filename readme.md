# Project Manager

This project is a CLI tool designed to help users manage their projects.

## Usage

```bash
project_manager [PATH] [COMMAND]
```

Both `path` and `command` are optional. If no path is provided, the current working directory is used.

The `new` command takes a project name and creates a new `project.toml` file in the specified path.

If no command is supplied, the tool will look for a `project.toml` file in the specified path, and if none is found, it will look for projects nested up to one level of depth.