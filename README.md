# DNS Lookup and Concurrent Connections (with Rust)

This project is a basic exploration of establishing connections in Rust via socket types in its standard libraries, as well as an attempt at a message-passing architecture to handle multiple connection attempts.

## dns-lookup

Obtains IP addresses (v4 and v6) from provided URLs.

## sequential-connections

Obtains IP addresses from provided URLs and attempts to connect to them in the order that DNS provides them.

## concurrent-connections

Obtains IP addresses from provided URLs and attempts concurrent connections, terminating when a connection is succesful. A message-passing architecture is used to pass addresses to connection attempts, and return a connected socket to the main thread.
