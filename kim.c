#include <stdio.h>

void scandisplay2(unsigned char a, unsigned char b, unsigned char c) {
    asm volatile(
       "st%0     0xFB\n"
       "st%1     0xFA\n"
       "st%2     0xF9\n"
       "jmp      0x1F1F" //; SCANDS kernal fn
        : 
        : "R"(a), "R"(b), "R"(c)
        : "memory"
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

