# Antivirus

Antivirus is a SigmaOS component providing a functional domain model for `antivirus2` with validated state, deterministic storage, and typed errors.

## Description

Antivirus antivirus compatibility layer for SigmaOS

## Domain model

- `Security` — domain entity

## API

- `new()`

## Testing

```sh
cargo test
```

Tests are deterministic, isolated, and self-validating.
