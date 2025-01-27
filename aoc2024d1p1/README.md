# Build instructions

cargo +mos build --release

or 

podman run -t -v $(pwd):/work:z mrkits/rust-mos:latest bash -c "cd /work && /home/mos/.cargo/bin/cargo +mos build --release"

objcopy -I elf32-little target/mos-unknown-none/release/rusty-kim -O binary target/mos-unknown-none/release/rusty-kim.bin

# Instructions for running the Adevent of Code 2024 Day 1 program

Load the binary at 0x200 and press GO

The program will display 3 addresses. It will wait for any keypress between each address

Ex:
```
0020 01
0204 02
0028 03
```

- Address 01 is an 8 byte input buffer. Store 2 32-bit Big-Endian integers from the puzzle input here.
- Address 02 is a function that computes the absolute difference of the two values and adds them to the total
- Address 03 is the answer buffer, a 64-bit Little-Endian integer.

Store your two input values in the input buffer beginning at Address 01 (0020 in example) (BIG ENDIAN for your convenience!)
Then execute the function at Address 02 (0204 in the example) by entering it in address mode and pressing the go button.

Repeat until all 1000 input pairs are entered.

The answer will be stored as 8 bytes (LITTLE ENDIAN for your inconvenience!) beginning at Address 03.
