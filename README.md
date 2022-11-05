# Rusty Orb

A project to embed rust binaries in CircleCI Orbs.

## jobs

### `rusty/hello`

You can call the hello job directly in a workflow.

```yaml
version: 2.1

orbs:
  rusty: bencord0/rusty-orb@1.0.0

workflows:
  say-hi:
    jobs:
      - rusty/hello
```

### `rusty/continue`

You can generate a circleci config dynamically and use this as a replacement
for the continuation-orb.

```yaml
# .circleci/config.yml
version: '2.1'
setup: true

orbs:
  rusty: bencord0/rusty-orb@1.1.0

jobs:
  setup:
    ...
    steps:
      - checkout
      - run:
          name: Generate Config
          command:
            ./generate_config.py > generated-config/config.yml
            ./generate_params.py > generated-config/parameters.yml
      - persist_to_workspace:
          root: .
          paths:
            - generated-config

workflows:
  setup:
    jobs:
    - setup
    - rusty/continue
        configuration: ./generated-config/config.yml
        parameters: ./generated-config/parameters.yml
        requires:
          - setup

```

```yaml
# ./generated-config/config.yml
version: '2.1'

parameters:
  my_parameter:
    type: string
    default: "FOO!"

jobs:
  my_job:
    ...

workflows:
  my_workflow:
    jobs:
      - my_job:
          my_parameter: << pipeline.parameters.my_parameter >>
```

```yaml
# ./generated-config/parameters.yml
my_parameter: foo
```

## commands

### `rusty/hello`

If not using the job, you can call the command in a custom job.


```yaml
version: 2.1

orbs:
  rusty: bencord0/rusty-orb@1.0.0

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
