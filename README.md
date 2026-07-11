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





### Config Options

#### Default Task



#### Shell



#### Environment File



#### Stop on Failure



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