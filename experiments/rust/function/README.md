Launch a local Lambda function instance:

```bash
cargo lambda watch
```

Query the function:

```bash
cargo lambda invoke function --data-ascii '{"name": "Bob"}'
```
