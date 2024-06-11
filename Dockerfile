FROM ubuntu:22.04
COPY ./target/release/my-no-sql-ui /target/release/my-no-sql-ui
COPY ./dist /target/release/dist
RUN chmod +x /target/release/my-no-sql-ui
WORKDIR /target/release/
ENTRYPOINT ["./my-no-sql-ui" ]