# Pr0t0n Orchestrator Command Line

Spec for the CLI for Pr0t0n Orchestrator.

This CLI should roughly mirror the requirements of the eventual UI.

Until we have a proper UI, it makes the most sense to simply specify the graph via a simple config language like YAML.
The YAML config would contain all of the details that would be entered via the UI.
Then we can run a command that syncs the database state to the YAML config.

For visualization, we can have commands that output the graph in a readable table format with details for the connection state etc.

## Commands

### `porch update`

Updates a service's data.

```
porch update <address> [--<param> <value>]
```

| Flag/Params   | Meaning                     |
| ------------- | --------------------------- |
| `--name`      | Name of the service.        |
| `--config_id` | Config ID for this service. |

### `porch connect`

Connect a service into another service.

```
porch connect <source address> <target address>
```

### `porch alert`

Configure an alert

```
porch connect <source address> <target address>
```

### `porch list`

Lists all services in the system.

```
porch list [-l|-a] [--type <type>]
```

| Flag/Params | Meaning                                                                        |
| ----------- | ------------------------------------------------------------------------------ |
| `-l`        | List more details per service.                                                 |
| `-a`        | Show all services, included disabled services.                                 |
| `--type`    | Only show services of type `<type>`. Can be `input`, `processor`, or `output`. |

### `porch config`

Create a new config.

```
porch config  --path <path.json>
```
