## Problem description

During assertions for a token contracts symbol and name if there is a difference between
left and right the result is shown as:
```Rust
assertion `left == right` failed
  left: String()
 right: String()
```

## How to reproduce

To reproduce the issue, go into `playground` directory and run `make test`.

