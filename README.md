# ckb-graphql-server

GraphQL server for CKB. For maximum performance, it works by reading from CKB's data directory directly.

# Caveat

Due to limitations in `rust-rocksdb`, we can only work with rocksdb's read only mode, not secondary mode for now. This will have a severe performance degradation for now, we will see if we can tackle this later.
