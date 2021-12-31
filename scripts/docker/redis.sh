docker run \
        --name axum_rs_redis \
        --restart=always \
        -v /var/docker/axum_rs_redis:/data \
        -p 127.0.0.1:16379:6379 \
        -d redis:alpine
