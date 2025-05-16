# Dotstore

Create dot directories (e.g. `/home/user/.path`) in common system directories.

## Install

```bash
cargo add dotstore
```

## Usage

```rust
use dotstore;

fn main() -> std::io::Result<()> {
    // Create a new directory called `/home/user/.barracuda`
    // The `.` is automatically appended
    let project_dir = dotstore::home_store("barracuda")?;

    // Create a new directory called `/home/user/.config/.editor`
    let editor_dir = dotstore::config_store("editor")?;

    // Create a new directory called `/home/user/workspace/middle-earth/.eregion`
    let custom_dir = dotstore::custom_store("/home/user/workspace/middle-earth", "eregion")?;

    Ok(())
}
```
