# -v "./opt/stellar:/opt/stellar" \

docker run --rm -it \
    -p "8000:8000" \
    --name stellar \
    stellar/quickstart:testing \
    --local \
    --enable-soroban-rpc