FROM rust as build

RUN rustup update nightly;
RUN rustup default nightly;

WORKDIR /app

COPY ./ ./

CMD cargo build --release

FROM ubuntu

RUN apt-get update && apt-get -y install ca-certificates libssl-dev
RUN apt install libsqlite3-dev -y && apt install libmecab-dev -y

COPY --from=build /app/target/release/ginkou /ginkou
COPY --from=build /app/.ginkoudb /.ginkoudb

CMD /ginkou