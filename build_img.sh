APP1=hello_app_1
APP2=hello_app_2
APP5=hello_app_v5
APP=apps

cd ${APP1} && ./build_bin.sh && cd ..
cd ${APP2} && ./build_bin.sh && cd ..

size1=$(xxd -p ./${APP1}.bin | tr -d '\n' | wc -c)
echo "Size: $((size1/2))"

dd if=/dev/zero of=./${APP}.bin bs=1M count=32
printf "%02x" $((size1/2)) | xxd -p -r | dd of=${APP}.bin conv=notrunc
dd if=./${APP1}.bin of=./${APP}.bin seek=2 bs=1 conv=notrunc 

# size2=$(xxd -p ./${APP2}.bin | tr -d '\n' | wc -c)
# echo "Size: $((size2/2))"

# printf "%02x" $((size2/2)) | xxd -p -r | dd of=${APP}.bin conv=notrunc seek=$((size1/2 + 1)) bs=1
# dd if=./${APP2}.bin of=./${APP}.bin seek=$((size1/2 + 2)) bs=1 conv=notrunc 

xxd -l $((size1/2)) -p ./${APP}.bin

mkdir -p ../arceos/payload
mv ./apps.bin ../arceos/payload/apps.bin