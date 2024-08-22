FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /chess
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /chess/target/x86_64-unknown-linux-musl/release/chess /chess
ENTRYPOINT ["/chess"]
EXPOSE 3000