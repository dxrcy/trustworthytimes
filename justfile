set shell := ["pwsh.exe", "-c"]

dev:
  cargo watch -x 'run -- --dev' -i docs
