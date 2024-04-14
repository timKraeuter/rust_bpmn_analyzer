FROM gcr.io/distroless/static-debian12
ARG RUST_APP=/target/x86_64-unknown-linux-musl/release/rust_bpmn_analyzer_webserver
ARG WEB_APP=/public
COPY ${WEB_APP} /public
COPY ${RUST_APP} /rust_bpmn_analyzer_webserver
CMD ["/rust_bpmn_analyzer_webserver"]
EXPOSE 8080/tcp