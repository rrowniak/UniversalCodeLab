#include <stdio.h>

void hello() {
    printf("Hello from C!\n");
#ifdef WELCOME
    printf("Welcome!\n");
#endif
    printf("%s - version %s\n", APP_NAME, VERSION);
}

void greet(const char* name) {
    hello();
    printf("Hello, %s!\n", name);
}
