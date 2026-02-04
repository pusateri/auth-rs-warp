# auth-rs-warp

Authentication / Authorization server example with Rust, Warp, Sqlite

## Features

- **Register users with email/password**
- **Get an exsisting user / Check if an email is already taken**
- **Login / Logout with email/password**
- **Access / modify protected resources**: one example included: creating posts that belong to a user

and also:

- **Great performance and minimal footprint** thanks to async Rust
- **Users database** using Sqlite

### Limitations

- As of now, this example is usable for writing API routes in Rust (i.e. starting from this code and extending). Usage with an other stack might be possible but wasn't yet taken into consideration for this project.

### Running the auth server

### Configuration options

Configuration is applied, from highest to lowest priority, through:

- Environment variables
- Config file located at `./.config/api_config(.ext)?` (relative to the binary). The config format (and extension `(.ext)?`) can be `json`,`yaml`,`toml`,`hcl`, `ini` or none (autodetected).
- Hardcoded defaults

These options are:

| Option            | ENV_VAR name        | Config name         | Default              |
| ----------------- | :------------------ | :------------------ | -------------------- |
| Sqlite filename   | `DATABASE_URL`      | `database_url`      | file.db              |
| HTTP port         | `HTTP_PORT`         | `http_port`         | `8080`               |
| Log level         | `RUST_LOG`          | `postgres_db`       | `auth-rs-warp=debug` |
| Enable backtraces | `RUST_BACKTRACE`    | `rust_backtrace`    | `1`                  |

## Testing

### Automated

Local testing is available by running

```shell
make test
```

### Manual

Test requests are included in the makefile (using `curl`)

- **Register a user** using `make users/register`
- **Check if an email is already taken** using `make users/check`
- **Login** using `make users/login`
- **Access a dummy protected route** using `make protected`

## TODO and contributing

Don't hesitate to file a bug, request a feature, or simply comment using issues.

If you want to help here's a few useful tasks I have in mind for this project:

- [ ] Email verification and email invitations
- [ ] CI
- [ ] example deployment instructions
- [ ] Write a tutorial to re-create this repo from scratch
