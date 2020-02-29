FROM scratch
ADD target/x86_64-unknown-linux-musl/release/board /main
ADD db.config.json /db.config.json
CMD [ "/main" ]

