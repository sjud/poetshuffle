FROM rustlang/rust:nightly-buster
COPY . .
ENV JWT_SECRET="Secret"
ENV DATABASE_URL="postgresql://user:password@0.0.0.0:5432/test_data"
RUN cargo +nightly build --release --bin server
ENTRYPOINT ["./target/release/server"]

