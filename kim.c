#include <stdio.h>

void scandisplay2(unsigned char a, unsigned char b, unsigned char c) {
    asm volatile(
       "st%0     $FB\n"
       "st%1     $FA\n"
       "st%2     $F9\n"
       "jmp      $1F1F" // SCANDS kernal fn
        : 
        : "R"(a), "R"(b), "R"(c)
        : "memory"
    );
}

char getkey() {
    char out;
    asm volatile(
        "jsr $1F40\n" // KEYIN
        "jsr $1F6A" // GETKEY
        : "=a" (out)
        :
        :
    );
    return out;
}

void __chrout(char c) {
    asm volatile(
        "jsr $1EA0"
        :
        :"a"(c)
        : 
    );
}

char __chrin2() {
    char out;
    asm volatile(
        "jsr $1E5A\n" // GETCHR
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

