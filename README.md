# AoCopt

Just a couple of people optimizing AoC problems a bit too far.

# Usage

## CLI Arguments

| short/long argument | meaning                                                                                                                                                                      | default       |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| -y/year             | The year to run solutions for                                                                                                                                                | 2023          |
| -d/day              | The day to run solutions for                                                                                                                                                 |               |
| -p/part             | The part to run solutions for                                                                                                                                                | Both parts    |
| -a/aoc_token        | The AOC session token to use for downloading the inputs. Only necessary if you want to automatically download inputs. Can be specified in the environment variable AOC_TOKEN |               |
| -b/bench            | The names of the solutions you want to run. When left empty,                                                                                                                 | All solutions |
|                     | all solutions for that year/day/part will be ran.                                                                                                                            |               |
|                     | Can be provided multiple times to run multiple solutions.                                                                                                                    |               |
| -c/clean            | Run the clean hook of the solutions                                                                                                                                          | false         |
| --config            | The path of the config file to use                                                                                                                                           | config.toml   |

# Config

### Solution model

Any given solution is recorded with a specific year, day and part that it's associated with.\
The harness will execute the steps defined by the solution in this order:\
`build -> pre_hook -> execute -> post_hook -> clean(if enabled with the -c CLI flag)`

#### Global values

|               key | type    | default   | meaning                                                                         |
| ----------------: | ------- | --------- | :------------------------------------------------------------------------------ |
| `download_inputs` | `bool`  | `true`    | Whether to download any missing task inputs by using your AOC token             |
|      `input_path` | `path`  | `inputs`  | The folder to save to/read inputs from. The input format is `year/day/part.txt` |
| `request_timeout` | seconds | `an hour` | The timeout for downloading the input for a task.                               |
| `command_timeout` | seconds | `an hour` | The timeout any command in the solutions.                                       |
|   `year.day.part` |         |           | The definition of a given solution.                                             |

#### Per-solution values

##### Required

|      key | type   | default | meaning                                                                                           |
| -------: | ------ | ------- | :------------------------------------------------------------------------------------------------ |
| language | string |         | The programming language the solution is written in.                                              |
|     name | string |         | The name of the solution, can be used to only run selected solutions.                             |
|     exec | string |         | The command to run at the exec step when executing the solution. This is where your solution goes |

##### Optional/have defaults

|             key | type   | default               | meaning                                                                                                                                                                                                                                               |
| --------------: | ------ | --------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|     description | string |                       | The description to print when benchmarking the solution.                                                                                                                                                                                              |
|            path | path   |                       | The _relative_ path to execute all the commands for the solution in.                                                                                                                                                                                  |
|           build | string |                       | The command to run at the build step when executing the solution.                                                                                                                                                                                     |
|        pre_hook | string |                       | The command to run at the pre_hook step when executing the solution.                                                                                                                                                                                  |
|       post_hook | string |                       | The command to run at the post_hook step when executing the solution.                                                                                                                                                                                 |
|           clean | string |                       | The command to run at the clean step when executing the solution. Only enabled with the `-c` CLI flag.                                                                                                                                                |
|           shell | list   | sh/cmd                | The command to execute any given command in the shell. On Windows, this defaults to `["cmd", "/C"]`, and everywhere else to `["sh", "-c"]`. The command you provide to be executed will be given as the next argument after your `shell` list         |
|          output | enum   | stdout                | whether to parse the output out of stdout or stderr.                                                                                                                                                                                                  |
|      time_regex | string | `(?m)^\d+[^\d]+(\d+)` | The regex that will be used to capture the timing out of the output, if time_externally isn't set to true. The time(assumed to be an integer in nanoseconds) will be taken from the first capture group. Uses the syntax of the Rust `regex` library. |
|    result_regex | string | `(?m)^(\d+)[^\d]+\d+` | The regex that will be used to capture the result out of the output. Uses the syntax of the Rust `regex` library.                                                                                                                                     |
| time_externally | bool   | false                 | Whether to rely on the time the exec command takes as the runtime of the solution. Generally not recommended as internal timing is almost certainly far more accurate.                                                                                |

# Defining solutions
