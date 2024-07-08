cargo doc --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=rustic\">" > target/doc/index.html
cp -r target/doc ./docs
cargo install cargo-tarpaulin
cargo tarpaulin --out Html