sudo docker-compose up builder && \
  rm -rf ./dist/* && \
  cp ./src/* ./dist/ && \
  cp -r ./wasm-app/pkg ./dist/pkg