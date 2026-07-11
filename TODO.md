# Things To Do

1) Refactor execution.rs
    - Helper functions
    - Error handling
    - Easier to read

2) Timeout Functionality
    - If 'timeout = XXX' in Forgefile, exit task after timeout has been reached.
    - If no timeout, skip it

3) Dependency Functionality
    - If 'depends_on' is specified, do some tree generation
    - HARDER