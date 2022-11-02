set shell := ["pwsh.exe", "-c"]

# start file directory must mirror `DIR_BUILD` in `statics.rs`
dev:
  start ./docs/index.html;
  cargo watch -x 'run -- --dev' -i docs;
