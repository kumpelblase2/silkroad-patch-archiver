# Silkroad Patch Archiver

A tool to automatically archive patches from the official Silkroad Online. By default it will start by patching up from the current installer version to the live version and then continue to download patches incrementally. This is not by choice - it's simply only possible to download the latest version of a file from the patch server.
This tool does the minimum required to download the patches, and as such does not even decompress the files. It also does not store any additional information provided by the patch server (e.g. if a file is marked as "inside pk2"), which should be easy to figure out manually.

This tool uses [skrillax-network](https://git.eternalwings.de/tim/skrillax-network) to handle connecting to the patch server and collecting the patch list. Downloading patch files is a simple http request to the patch host.

Right now, this tool "does the job" but obviously is not perfect. Some reasons:
- No proper error handling at all
- Metadata is thrown away, but might be good to have

## Running
Simply build it and then execute it:
```shell
$ cargo build --release
$ ./target/release/silkroad-patch-archiver
```
Or you can build the docker container:
```shell
$ docker build -t silkroad-patch-archiver .
$ docker run --rm silkroad-patch-archiver
```

The patches will be collected into a new `patches` subdir of the current working directory.

## Configuration

By creating a `config.toml` in the current directory, you can control two things, the `current_version` and the `base_version`.
Example config:
```toml
base_version = 635
current_version = 657
```
The `base_version` is only really relevant when the tool is first started. It's the starting point for patching.
The `current_version` is the last version we've downloaded a patch for. This will be automatically updated by the tool once a new patch release has been downloaded, but can be manually adjusted to re-download or skip a download.