# ckb-graphql-server

GraphQL server for CKB. For maximum performance, it works by reading from CKB's data directory directly.

# Usage

For now, the GraphQL server works with CKB 0.23.

First, a running CKB instance is needed. Since this project directly reads data from CKB's rocksdb instance, we need to keep track of the running directory of CKB. For example, if CKB is started via `ckb run -C /foo/bar`, the directory we will need here is `/foo/bar`. Please take a look at your CKB running configuration, and keep a note of the running directory in our environment.

Then we can start CKB's GraphQL server:

```
$ git clone https://github.com/nervosnetwork/ckb-graphql-server
$ cd ckb-graphql-server
$ cargo build --release
$ target/release/ckb-graphql-server --db /foo/bar/data/db --listen 0.0.0.0:3000
```

Again, if you are running CKB in a different directory than `/foo/bar`, you need to modify the command accordingly.

Now if you go to `http://localhost:3000`, you should have a GraphQL server to play with.

# Caveat

Due to limitations in `rust-rocksdb`, we can only work with rocksdb's read only mode, not secondary mode for now. This will have a severe performance degradation for now, we will see if we can tackle this later.
