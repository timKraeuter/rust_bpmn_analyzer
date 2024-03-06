FROM alpine:3.19.1
ARG RUST_APP=/target/x86_64-unknown-linux-musl/release/bpmnanalyzer
ARG WEB_APP=/public
COPY ${WEB_APP} /public
COPY ${RUST_APP} /bpmnanalyzer
RUN chmod +x /bpmnanalyzer
ENTRYPOINT ["/bpmnanalyzer"]
EXPOSE 8080/tcp