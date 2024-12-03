cargo build --target riscv64gc-unknown-none-elf --release

rust-objcopy --binary-architecture=riscv64 --strip-all -O binary target/riscv64gc-unknown-none-elf/release/hello_app ./hello_app.bin

size=$(xxd -p ./hello_app.bin | tr -d '\n' | wc -c)
echo "Size: $((size/2))"

dd if=/dev/zero of=./apps.bin bs=1M count=32
printf "%02x" $((size/2)) | xxd -p -r | dd of=apps.bin conv=notrunc
dd if=./hello_app.bin of=./apps.bin seek=1 bs=1 conv=notrunc 

xxd -l 10 -p ./apps.bin

mkdir -p ../arceos/payload
mv ./apps.bin ../arceos/payload/apps.bin