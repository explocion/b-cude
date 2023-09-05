# Byte-by-Byte: A Simple Bytestream Protocol

## The Name

Byte-by-Byte is BbB in short, where can be further abberivated into `b^3`, where the name `b-cube` comes from. This is a protocol inspired by a protocol in labs of EECS 473 at University of Michigan.

## The Protocol

The protocol is fairly simple. It transmits and receives in units of packets, where each packet has the following form: SoP + Length (1 byte) + TypeId (1 byte) + Bytes of Length + EoP. For the protocol to work out nicely, SoP and EoP byte should not be common in the communication.
