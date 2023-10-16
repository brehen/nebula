FROM rust:1.70 as build
WORKDIR /usr/src/nebula

COPY shared /usr/src/shared

ARG PROGRAM_NAME
COPY $PROGRAM_NAME .

RUN cargo build --release --features docker 

# Runtime Stage
FROM debian:bullseye-slim
RUN apt-get update && apt-get install libc6 && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/src/nebula/target/release/$PROGRAM_NAME /usr/local/bin/
COPY --from=build /usr/src/nebula/run.sh /usr/local/bin/

CMD ["run.sh"]
