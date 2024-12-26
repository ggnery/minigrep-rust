# minigrep-rust
To run minigrip case sensitive
```bash
cargo run <query> <file absolute path>
```

To run minigrip not case sensitive
```bash
IGNORE_CASE=true cargo run <query> <file absolute path>
```

## Checklist
- [] Make it recursive (dfs)
- [] Read from buffer