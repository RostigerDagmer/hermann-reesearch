# Setup

Make sure docker + docker compose is installed and a docker daemon is running.
In a terminal run:
```
docker compose up
```

## Because I was to lazy to write a dockerfile
Switch to your api container.

```
docker exec -it hermann-reesearch-api-1 /bin/bash
```

Install protoc in the container

```
curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v22.3/protoc-22.3-linux-x86_64.zip

unzip protoc-22.3-linux-x86_64.zip -d $HOME/.local

export PATH="$PATH:$HOME/.local/bin"
```

Start the GRPC endpoint:

```
cargo run --release
```

## Getting the paper data

Have an access key for the Semantic Scholar Dataset API.
Download the files containing the links to the "papers" and "abstracts" datasets to the root of the project.
Download the datasets to any location.
Check paths in scripts/download.py beforehand

```
python3 scripts/download.py
```

## Merge the datasets

Check and adjust paths in scripts/merge.py

```
python3 scripts/merge.py
```

## Insert papers into vector database

Make sure you have access to the triton server running specter2 (not published yet).

Check insert.py.

Range queries on the sql database holding the merged data perform incredibly bad.
Even with an index.
TODO.
Inserting the first couple million papers should be relatively quick and good enough for testing.

```
python3 scripts/insert.py
```