#!/bin/bash
iface=enp62s0u1u1

ip link set dev $iface up
ip addr add 10.9.8.1/24 dev $iface
