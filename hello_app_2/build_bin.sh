APP=hello_app_2

cargo build --target riscv64gc-unknown-none-elf --release

rust-objcopy --binary-architecture=riscv64 --strip-all -O binary target/riscv64gc-unknown-none-elf/release/${APP} ../${APP}.bin