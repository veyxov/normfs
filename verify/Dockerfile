# Pinned proof toolchain for the NormFS C layer.
#
# The proof numbers quoted in the repository are only meaningful if the tool
# versions that produced them are fixed. Frama-C, Alt-Ergo, Z3 and the OCaml
# compiler are all pinned here; changing any of them changes what "proved"
# means and must be a deliberate commit.
FROM debian:trixie-slim

ARG OCAML_VERSION=4.14.2
ARG FRAMA_C_VERSION=32.1
ARG ALT_ERGO_VERSION=2.6.3
# One asset name, so the version and the glibc it is linked against cannot be
# changed apart. That glibc is also the floor for the base image above.
ARG Z3_ASSET=z3-4.16.0-x64-glibc-2.39
ARG WHY3_VERSION=1.8.2

ENV DEBIAN_FRONTEND=noninteractive
ENV OPAMROOT=/opt/opam
ENV OPAMYES=1

# The depexts are not optional even though nothing here opens a window:
# frama-c pulls conf-graphviz and lablgtk3-sourceview3, and without the system
# packages opam reports "No solution found" rather than naming what is missing.
# Installing the GUI dependencies is what makes the image reproduce the
# toolchain the proof numbers were measured on.
RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential ca-certificates cmake curl git m4 unzip \
        pkg-config libgmp-dev libmpfr-dev zlib1g-dev opam \
        graphviz jq \
        libgtk-3-dev libgtksourceview-3.0-dev libexpat1-dev \
    && rm -rf /var/lib/apt/lists/*

# Z3 ships prebuilt; building it from source would add ~20 minutes for no
# benefit, and the release archive is what everyone else runs.
RUN curl -fsSL -o /tmp/z3.zip \
        "https://github.com/Z3Prover/z3/releases/download/${Z3_ASSET%%-x64*}/${Z3_ASSET}.zip" \
    && unzip -q /tmp/z3.zip -d /tmp \
    && install -m 0755 "/tmp/${Z3_ASSET}/bin/z3" /usr/local/bin/z3 \
    && rm -rf /tmp/z3.zip "/tmp/${Z3_ASSET}"

RUN opam init --bare --disable-sandboxing \
    && opam switch create framac "ocaml-base-compiler.${OCAML_VERSION}" \
    && opam install \
        "frama-c.${FRAMA_C_VERSION}" \
        "alt-ergo.${ALT_ERGO_VERSION}" \
        "why3.${WHY3_VERSION}" \
    && opam clean -a -c -s --logs

ENV PATH=/opt/opam/framac/bin:$PATH

# Why3 caches prover detection; doing it at build time keeps every proof run
# from re-probing and keeps the detected set identical across runs.
RUN why3 config detect

WORKDIR /work

# Fail loudly at container start if a prover is missing rather than silently
# proving fewer goals.
RUN printf '%s\n' \
        '#!/bin/sh' \
        'set -e' \
        'frama-c -version' \
        'alt-ergo --version' \
        'z3 --version' \
        'exec "$@"' \
        > /usr/local/bin/entrypoint && chmod 0755 /usr/local/bin/entrypoint

ENTRYPOINT ["/usr/local/bin/entrypoint"]
CMD ["make", "verify"]
