# DeepTrace CLI

This is the command-line interface (CLI) tool for the DeepTrace project. It provides commands to manage agents, run trace association algorithms, manage the database, and assemble traces.

## Requirements

- Python 3.7+
- All dependencies installed (see your project requirements)

## Usage

Run the CLI at the server dir DeepTrace/server/:

```bash
python -m cli.src.cmd <command> [options]
```

## Commands

### Agent Management

Manage tracing agents.

```bash
python -m cli.src.cmd agent <install|test|stop|run>
```

- `install` — Install agents
- `test` — Test agent connectivity
- `stop` — Stop agents
- `run` — Run agents

### Association Algorithms

Run trace association algorithms.

```bash
python -m cli.src.cmd asso algo <algorithm>
```

- `<algorithm>`: Choose from `fifo`, `deeptrace`, `vpath`, `wap5`, `traceweaver_v1`, `deepflow`

Example:

```bash
python -m cli.src.cmd asso algo fifo
```

### Database Management

Install or uninstall the database.

```bash
python -m cli.src.cmd db <install|uninstall>
```

- `install` — Deploy the database
- `uninstall` — Remove the database

### Assemble Traces

Assemble traces from the database.

```bash
python -m cli.src.cmd assemble
```


## Example

Install agents:

```bash
python -m cli.src.cmd agent install
```

Run the FIFO association algorithm:

```bash
python -m cli.src.cmd asso algo fifo
```

Assemble traces:

```bash
python -m cli.src.cmd assemble
```

## Notes

- Make sure your environment is properly configured and all dependencies are installed.
- Some commands may require a running database or Elasticsearch instance.

---