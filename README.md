# rtcp

rtcp means redirect tcp, writing in rust, a command line pocket capturer

## Usage
> rtcp ip1:port1 ip2:port2
## Example
rtcp localhost:1234 localhost:5678 | grep token
## How to compile
1. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. git clone git@github.com:yanfenglee/rtcp.git
3. cd rtcp
4. cargo build --release

