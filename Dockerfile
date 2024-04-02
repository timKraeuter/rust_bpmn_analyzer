FROM alpine:3.19.1
ARG RUST_APP=/target/x86_64-unknown-linux-musl/release/rust_bpmn_analyzer
ARG WEB_APP=/public
COPY ${WEB_APP} /public
COPY ${RUST_APP} /rust_bpmn_analyzer
RUN chmod +x /rust_bpmn_analyzer
ENTRYPOINT ["/rust_bpmn_analyzer"]
EXPOSE 8080/tcp