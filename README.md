# watch

![Cargo tests and lints status](https://github.com/jacobcallear/watch/actions/workflows/tests.yml/badge.svg)

Linux `watch` command for all OS's and shells

Command line tool that runs a given command repeatedly and prints the command
output.

## Usage

- Run `echo Hello world!` command repeatedly (every 2 seconds by default):

  ```shell
  watch echo Hello world!
  ```

- Run `echo Hello world!` command every 10 seconds:

  ```shell
  watch -i 10 echo Hello world!
  ```

- Run `git status` every 5 seconds, and clear the screen each time:

  ```shell
  watch -c -i 5 git status
  ```
