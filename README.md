## Utils commands for dev

- For watching the server
```sh
  cargo watch -q -c /src -x run
  cargo watch -c -x "run"
```

- For watching tests
```sh
  cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
