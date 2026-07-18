# jellyname

`jellyname` is a small command-line tool that organizes movie and TV episode files using Jellyfin-compatible names and folders.

It moves files rather than copying them, so review your paths before running it.

## Install

Install from the repository with Rust and Cargo:

```console
git clone https://github.com/aixoio/jellyname.git
cd jellyname
cargo install --path .
```

## Examples

Rename a movie after creating and editing `jellyname.toml`:

```console
jellyname init movie
jellyname rename movie /path/to/movie.mkv
```

Prepare, review, and apply a TV series rename:

```console
jellyname init series
jellyname rename series /path/to/series
jellyname apply series
```

Run `jellyname --help` or `jellyname <command> --help` for the full command documentation.

## License

[GPL-3.0](LICENSE)
