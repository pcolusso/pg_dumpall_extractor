# PostgreSQL pg_dumpall Database Extractor

PostgreSQL's pg_dumpall is a great tool for creating logical backups of your database. However, you cannot selectivley choose which database to restore. This utility will allow you to pull out the required SQL for a specific database and save it to a new file.

## Compiling in docker

`docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.23.0 cargo build --release`
