# resolvething

resolve syncthing conflicts and oddities quickly

# Why?

Syncthing is amazing. It's the only piece of software I have ever set up, not
touched for years and it still works. It does leave behind a lot of gunk files if
you switch between devices often and

# Features

- Detect conflicts, use merge tool to resolve them.
- Detect duplicates, offer

# Installation

install dependencies
- fclones
- fzf
- bat
- trash

```sh
cargo install resolvething
```

# Roadmap

- rewrite in rust
    - config
      - verify config
      - allow tilde in path name
    - better error handling
- shell completion
- maybe replace fd with walkdir
- CI
    - tests
    - release binaries

# Disclaimer

this sorta works now, but no promises.
