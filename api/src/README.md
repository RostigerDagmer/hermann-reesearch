# Install Protocol buffers

PB_REL="https://github.com/protocolbuffers/protobuf/releases"

curl -LO $PB_REL/download/v22.2/protoc-22.2-linux-aarch_64.zip

unzip protoc-22.2-linux-aarch_64.zip -d $HOME/.local

export PATH="$PATH:$HOME/.local/bin"
