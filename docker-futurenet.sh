docker run --rm -i \
    -p "8000:8000" \
    --name stellar \
    stellar/quickstart:latest \
    --futurenet \
    --enable-soroban-rpc \
    --enable-soroban-diagnostic-events \
    --enable rpc,horizon