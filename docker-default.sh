# -v "./opt/stellar:/opt/stellar" \

docker run --rm -i \
    -p "8000:8000" \
    --name stellar \
    stellar/quickstart:latest \
    --local \
    --limits default \
    --enable-soroban-rpc