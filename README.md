# forge
TOML-based task definition | Written in Rust

## How to Build

To build the Forge binary, use the Makefile:

1) View the avaiable `make` targets with:
    - `make help`

2) Build and relocate the Forge binary with:
    - `make`


## How to Run

To run Forge, do the following:

1) Ensure Forge is accessible via PATH with:
    - `which forge`

2) View the available Forge options with:
    - `forge -h`

3) Run Forge using a `forge.toml` in your current directory with:
    - `forge`


## Forgefile Setup

Below outlines how to setup your Forgefile:

### Example

```toml
[config]
default_task = "hello_world"
shell = "/usr/bin/ssh"

[task.hello_world]
description = "Basic task to echo 'Hello, World!'"
command = "echo 'Hello, World!'"
ignore_fail = true

[task.pwd]
description = "Test working_dir default"
command = "pwd"
working_dir = "/etc/ssh

[task.whoami]
description = "Another basic task to print who I am"
command = "whoami"

[task.ping]
description = "Ping google"
command = "ping 8.8.8.8"
working_dir = "/var/log"
confirm = true
timeout = 5
ignore_fail = true
```



### Config Options

#### Default Task

This option describes the task to execute by default if no other task is specified.

- Example: `default_task = "hello_world"`
- Type: String

#### Shell

This option specifies the shell to use when executing each task command.

- Example: `shell = "/bin/bash"`
- Type: String

#### Environment File

This option specifies a `.env` environment variable to apply to every task.

- Example: `env_file = "/home/user/.env"`
- Type: String


### Task Options

Below lists the available options you can include in each Forgefile

#### Description

This option describes its corresponding task.

- Example: `description = "Silly description"`
- Type: String

#### Command

This option specifies the command to run inside the task.

- Example: `command = "echo 'Hello, World!'"
- Type: String

#### Depends On

This option specifies the other tasks to complete prior to this one.

- Example: `depends_on = [hello_world, ping]`
- Type: List

*Value should be a comma separated list*

#### Environment

This option specifies shell environment variables to apply to the current task command.

- Example: `env = {"BUILD_DEBUG": 1, "DO_SOMETHING_ELSE": true}`
- Type: Dictionary

#### Working Directory

This option specifies the working directory of this task's command execution. 

- Example: `working_fir = "/etc/ssh/"`
- Type: String

#### Confirm

This option specifies that the user will be prompted prior to task execution.

- Example: `confirm = true`
- Type: Boolean

#### Timeout

This option specifies the timeout (in seconds) to place on the current task.

- Example: `timeout = 300`
- Type: Integer

#### Ignore Fail

This option specifies that the Forgefile will not exit upon task failure.

- Example: `ignore_fail = true`
- Type: Boolean