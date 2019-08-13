#
# Author: Simon Bergström
# simon at menuan dot se
#

FROM alpine:3.10.1

COPY target/x86_64-unknown-linux-musl/release/whecko .

ENV RUST_LOG=info

CMD ["./whecko"]
