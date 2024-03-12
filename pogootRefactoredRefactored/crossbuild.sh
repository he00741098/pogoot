mv ./Cargo.toml ./CargoTemp.toml
mv ./CargoCross.toml ./Cargo.toml
cross build --release --target aarch64-unknown-linux-gnu
mv ./Cargo.toml ./CargoCross.toml
mv ./CargoTemp.toml ./Cargo.toml
