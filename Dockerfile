# Use the official Rust image
FROM rust:latest

# Create a new directory for your application
WORKDIR /usr/src/rusttcp

# setcap capbabilities
RUn apt-get update && apt-get install -y \
    libcap2-bin \
    iproute2 \
    iputils-ping \
    net-tools \
    libcap2-bin \
    iptables \
    tmux \
    sudo


# RUN setcap cap_net_admin=eip /usr/src/rusttcp/trust/target/release/trust


# Set the working directory to the newly created Rust project
WORKDIR /usr/src/rusttcp

# Set up the default command to run in interactive mode
CMD ["bash"]


############################################################
# Build and run in interactive mode
# 
# docker build -t rusttcp .
# docker run -it --privileged -v $(pwd):/usr/src/rusttcp rusttcp
############################################################