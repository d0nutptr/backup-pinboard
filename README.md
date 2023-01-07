# backup-pinboard

This is a tool for creating a local backup of your Pinboard bookmarks.

## Installation

```console
$ cargo install backup-pinboard
```

[rust]: https://www.rust-lang.org/en-US/
[rustup]: https://rustup.rs

## Usage

Download your bookmarks metadata:

```console
$ backup-pinboard metadata --username=USERNAME --password=PASSWORD
```

This downloads your metadata to `pinboard.json`.
You can specify an alternative path with `--output`, for example:

```console
$ backup-pinboard metadata --username=USERNAME --password=PASSWORD --output=~/backups/pinboard.json
```

If you have an archival account, you can also download copies of your saved pages:

```console
$ backup-pinboard archive --username=USERNAME --password=PASSWORD --output_directory=~/backups/pinboard-archive
```

## Credit
This was [a project](https://github.com/alexwlchan/backup-pinboard) originally written by [alexwlchan](https://github.com/alexwlchan)
