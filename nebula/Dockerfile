# Build stage
FROM rust:1.70 as build
WORKDIR /usr/src
USER root

RUN curl -fsSL https://bun.sh/install | bash && \
  ln -s $HOME/.bun/bin/bun /usr/local/bin/bun

RUN bun install -g tailwindcss



# Copy the Cargo manifests and lock files for both `nebula_lib` and `nebula_server`
COPY nebula_lib/Cargo.toml nebula_lib/Cargo.lock ./nebula_lib/
COPY nebula_server/Cargo.toml nebula_server/Cargo.lock ./nebula_server/

# Dummy build to cache dependencies (this helps in faster subsequent builds)
# # Dummy build for nebula_lib
# RUN mkdir -p nebula_lib/src && echo "fn main() {}" > nebula_lib/src/lib.rs && cargo build --manifest-path nebula_lib/Cargo.toml
# # # Dummy build for nebula_server
# RUN mkdir -p nebula_server/src && echo "fn main() {}" > nebula_server/src/main.rs && cargo build --manifest-path nebula_server/Cargo.toml


# Now, copy the actual source code and build the projects
COPY nebula_lib/src ./nebula_lib/src
COPY nebula_server/src ./nebula_server/src
COPY nebula_server/styles ./nebula_server/styles
COPY nebula_server/tailwind.config.cjs ./nebula_server/tailwind.config.cjs
COPY nebula_server/templates ./nebula_server/templates

RUN cd nebula_server && bun x tailwindcss -i ./styles/tailwind.css -o ./assets/main.css -c ./tailwind.config.cjs && cd ..

RUN cat nebula_server/assets/main.css


COPY nebula_server/assets ./nebula_server/assets

RUN cargo build --release --manifest-path nebula_server/Cargo.toml


# Runtime Stage
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y docker.io
COPY --from=build /usr/src/nebula_server/target/release/nebula_server /usr/local/bin/
COPY --from=build /usr/src/nebula_server/assets /assets

COPY nebula_server/entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["entrypoint.sh"]

EXPOSE 8000
CMD ["nebula_server"]
