#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

void scandisplay_slow(unsigned char a, unsigned char b, unsigned char c) {
    asm volatile(
       "ST%0     0xFB\n"
       "ST%1     0xFA\n"
       "ST%2     0xF9\n"
        "ldy #$10\n"
        "delay_loop1: ldx #$ff\n"
        "delay_loop2: jsr      0x1F1F\n" //; SCANDS kernal fn
        "dex\n"
        "bne delay_loop2\n"
        "dey\n"
        "bne delay_loop1\n"

        : 
        : "R"(a), "R"(b), "R"(c)
        : "memory"
    );
}

void scandisplay(unsigned char a, unsigned char b, unsigned char c) {
    asm volatile(
       "ST%0     0xFB\n"
       "ST%1     0xFA\n"
       "ST%2     0xF9\n"
       "jsr      0x1F1F" //; SCANDS kernal fn
        : 
        : "R"(a), "R"(b), "R"(c)
        : "memory"
    );
}



void cleardisplay()
{
    *((uint8_t*)0x1742) = 0;
}

void nop() {
    asm volatile(
        "nop\n"
        :
        :
    );
}

void brk() {
    asm volatile(
        "brk\n"
        :
        :
    );

}

void delay() {
    asm volatile(
        "ldy #$ff\n"
        "loop1: ldx #$ff\n"
        "loop2: nop\n"
        "dex\n"
        "bne loop2\n"
        "dey\n"
        "bne loop1\n"
        :
        :
        :"x", "y"
    );
}

void start() {
    asm volatile(
        "jmp 0x1c4f\n" // START
        :::
    );
}

char getkey() {
    char out;
    asm volatile(
        "JSR 0x1F40\n" //; KEYIN
        "JSR 0x1F6A\n" //; GETKEY
        : "=a" (out)
        :
        :
    );
    return out;
}

bool ak() {
    char out;
    asm volatile(
        "JSR 0x1EFE\n"
        : "=a" (out)
        :
        :
    );
    return (out != 0);
}

void __chrout(char c) {
    asm volatile(
        "JSR 0x1EA0"
        :
        :"a"(c)
        : 
    );
}

char __chrin2() {
    char out;
    asm volatile(
        "JSR 0x1E5A" // GETCHR
        :"=a" (out)
        :
        : 
    );
    return out;
}

void __putchar(char c) {
  if (__builtin_expect(c == '\n', 0))
    __chrout('\r');
  __chrout(c);
}

char getchar2(void) {
  char c = __chrin2();
  return c;
}

int __chrin(void) {
    int c;
    c = (int)__chrin2();
    return c;
}

int getchar(void) {
    int c;
    c = __chrin();
    return c;
}
