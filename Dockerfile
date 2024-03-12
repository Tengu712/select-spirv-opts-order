FROM ubuntu:22.04

RUN DEBIAN_FRONTEND=noninteractive apt-get update \
 && apt-get install --no-install-recommends -y ca-certificates git gcc rustc cargo wget \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

RUN DEBIAN_FRONTEND=noninteractive apt-get update \
 && wget -qO- https://packages.lunarg.com/lunarg-signing-key-pub.asc | tee /etc/apt/trusted.gpg.d/lunarg.asc \
 && wget -qO /etc/apt/sources.list.d/lunarg-vulkan-jammy.list http://packages.lunarg.com/vulkan/lunarg-vulkan-jammy.list \
 && DEBIAN_FRONTEND=noninteractive apt-get update \
 && apt-get install --no-install-recommends -y mesa-utils vulkan-sdk libvulkan-dev vulkan-tools \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

COPY . /root/

RUN mkdir /root/external \
 && export GIT_SSL_NO_VERIFY=1 \
 && git clone https://github.com/rust-random/rand.git /root/external/rand \
 && mkdir /root/.cargo \
 && echo "[http]\ncheck-revoke = false\n" > /root/.cargo/config \
 && cargo build --release --manifest-path /root/external/rand/Cargo.toml \
 && mkdir /root/bin

RUN cd bin \
 && gcc -c -DRELEASE_BUILD /root/src/c/util/memory/*.c /root/src/c/util/*.c /root/src/c/*.c \
 && ar r libvulkan-wrapper.a *.o \
 && rm *.o \
 && rustc -o /root/bin/ga \
      --edition=2021 \
      -L . \
      -L /usr/lib/x86_64-linux-gnu \
      -L /root/external/rand/target/release \
      -L /root/external/rand/target/release/deps \
      -lvulkan-wrapper \
      -lvulkan \
      --extern rand=/root/external/rand/target/release/librand.rlib \
      /root/src/rust/main.rs \
 && glslc -o /root/bin/shader.vert.spv /root/src/shader/shader.vert \
 && glslc -o /root/bin/shader.org.frag.spv /root/src/shader/shader.frag

WORKDIR /root/bin

CMD ./ga
