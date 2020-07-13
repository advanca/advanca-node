FROM rust:1.44.0 as builder
LABEL maintainer "Advanca Authors"
LABEL description="This is the build stage for advanca-node"

ARG PROFILE=release
WORKDIR /advanca

COPY . /advanca

RUN apt-get update && \
	apt-get install -y --no-install-recommends cmake clang

RUN rustup toolchain install nightly-2020-04-07 && \
    rustup default nightly-2020-04-07 && \
    rustup target add wasm32-unknown-unknown --toolchain nightly-2020-04-07

ENV RUSTC_BOOTSTRAP=1
RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM rust:1.44.0
LABEL maintainer "Advanca Authors"
LABEL description="This is the 2nd stage"

ARG PROFILE=release
COPY --from=builder /advanca/target/$PROFILE/advanca-node /usr/local/bin

RUN	useradd -m -u 1000 -U -s /bin/sh -d /advanca advanca && \
	mkdir -p /advanca/.local/share/advanca && \
	chown -R advanca:advanca /advanca/.local && \
	ln -s /advanca/.local/share/advanca /data

USER advanca
EXPOSE 30333 9933 9944
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/advanca-node"]
