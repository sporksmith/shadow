ARG CONTAINER="ubuntu:20.04"

FROM ${CONTAINER}

COPY . /src

WORKDIR /src

ENV RUSTPROFILE minimal

ARG CONTAINER="ubuntu:20.04"
ARG BUILDTYPE=release
ARG CC=gcc

RUN ci/container_scripts/install_deps.sh

RUN ci/container_scripts/install_extra_deps.sh

ENV PATH "/root/.cargo/bin:${PATH}"
RUN ci/container_scripts/build_and_install.sh

ENV PATH="/root/.local/bin:${PATH}"

RUN ci/container_scripts/test.sh

ENTRYPOINT ["shadow"]
