sed -i 's/tempfile = "3.10"//g' crates/mev-internal/Cargo.toml
cat << 'INNEREOF' >> crates/mev-internal/Cargo.toml
[dependencies]
tempfile = "3.10"
INNEREOF
