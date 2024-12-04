cargo +mos build --release

or 

podman run -t -v $(pwd):/work:z mrkits/rust-mos:latest bash -c "cd /work && /home/mos/.cargo/bin/cargo +mos build --release"

objcopy -I elf32-little target/mos-unknown-none/release/rusty-kim -O binary target/mos-unknown-none/release/rusty-kim.bin
