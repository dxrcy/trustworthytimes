# start file directory must mirror `DIR_BUILD` in `statics.rs`
dev:
  cargo watch -x 'run -- --dev' -i docs -i .devbuild;
