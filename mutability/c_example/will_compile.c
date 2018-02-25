#include <stdio.h>

int main() {
    const int x = 5;
    int *p = &x;
    *p = 6;
    printf("%d", x);
}
