# Rusty Orb

A project to embed rust binaries in CircleCI Orbs.

## jobs

### `rusty/hello`

You can call the hello job directly in a workflow.

```yaml
version: 2.1

orbs:
  rusty: bencord0/rusty-orb@v1

workflows:
  say-hi:
    jobs:
      - rusty/hello
```

## commands

### `rusty/hello`

If not using the job, you can call the command in a custom job.


```yaml
version: 2.1

orbs:
  rusty: bencord0/rusty-orb@v1

workflows:
  say-hi:
    jobs:
      - say-hi

jobs:
  say-hi:
    executor: rusty/default
    steps:
      - rusty/hello

```
