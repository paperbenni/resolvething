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
- fdupes
- fzf
- bat
- trash
- fd

TODO: crates.io


# Roadmap

- rewrite in rust
    - add clap documentation
    - config
      - verify config
      - allow tilde in path name
    - better error handling
- maybe replace fd with walkdir
- CI
    - tests
    - release binaries

# Thoughts

I do not want to reinvent the wheel. This is supposed to be small and rely on
existing, mature and well supported tools as much as possible. fzf, fd, fdupes
etc are good. I could not do a better job with the amount of ime I'm willing to
spend on this.

# Disclaimer

this sorta works now, but no promises.
