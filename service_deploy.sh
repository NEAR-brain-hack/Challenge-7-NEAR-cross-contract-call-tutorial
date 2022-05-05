# deploy dao

near deploy \
    --wasmFile out/service.wasm \
    --initFunction "new" \
    --initArgs '{}' \
    --accountId service.manhng.testnet