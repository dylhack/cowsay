BIN=./node_modules/.bin
API=./api
PROTO=../proto
JS_PLUGIN=$BIN/grpc_tools_node_protoc_plugin
TS_PLUGIN=$BIN/protoc-gen-ts

protoc \
  --js_out=import_style=commonjs,binary:$API \
  --grpc_out=grpc_js:$API \
  --plugin=protoc-gen-grpc=$JS_PLUGIN \
  -I $PROTO \
  $PROTO/*.proto

protoc \
  --plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts \
  --ts_out=grpc_js:$API \
  -I $PROTO \
  $PROTO/*.proto
