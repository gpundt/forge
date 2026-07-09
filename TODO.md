# Things To Do

1) Confirm Functionality
    - If no confirm, default is false

2) Working Directory Default
    - If there is no 'working_dir' specified, default to pwd

3) Timeout Functionality
    - If 'timeout = XXX' in Forgefile, exit task after timeout has been reached.
    - If no timeout, skip it

4) Dependency Functionality
    - If 'depends_on' is specified, do some tree generation
    - HARDER