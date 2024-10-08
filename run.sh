#!/bin/bash
cd trust/
# /usr/local/cargo/bin/cargo clean
/usr/local/cargo/bin/cargo b --release
ext=$?
if [[ $ext -ne 0 ]]; then
	exit $ext
fi
setcap cap_net_admin=eip target/release/trust
./target/release/trust &
pid=$!
ip addr add 192.168.0.1/24 dev tun0
ip link set up dev tun0
ping -w 1 10.0.0.2 -I tun0
trap "kill $pid" INT TERM
wait $pid