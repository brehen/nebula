ARG IMAGE=debian:bullseye-slim

FROM rust:1.70 as build
WORKDIR /usr/src/nebula

COPY shared /usr/src/shared

ARG PROGRAM_NAME
COPY $PROGRAM_NAME .

RUN cargo build --release --features docker 

FROM $IMAGE

RUN if ["$IMAGE" != "archlinux:latest"]; then \
      apt-get update && \
      apt-get install -y libc6 && \
      rm -rf /var/lib/apt/lists/* ;\
    fi

COPY --from=build /usr/src/nebula/target/release/$PROGRAM_NAME /usr/local/bin/
COPY --from=build /usr/src/nebula/run.sh /usr/local/bin/

CMD ["run.sh"]
