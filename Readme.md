## Install Maelstorm 
Just download the compressed build from this [link](https://github.com/jepsen-io/maelstrom/releases/tag/v0.2.3).
You'll find the Maelstrom binary inside which can be used to run the distributed systems tests.

## Dependencies on OSX

```sh
brew install openjdk graphviz gnuplot    
```

## Running the [ECHO challenge](https://fly.io/dist-sys/1/)
1. Checkout to the commit 

```
git checkout c3f9b96875a81f282647c5999ecb599bb8daf3ec
```

2. Build 

```sh
cargo build
```

2. Run the malestorm binary from the root of the project 

```
maelstrom test -w echo --bin target/debug/maelstrome --node-count 1 --time-limit 10
```

`target/debug/maelstrome` points to the cargo build wheras `maelstrom test` points to the `maelstrom` binary downloaded from the [link here](https://github.com/jepsen-io/maelstrom/releases/tag/v0.2.3). 