# include <stdio.h>
# include <stdlib.h>

int main (void) {
    int* restrict p = malloc(sizeof(int));

    int* q = p;

    *q = 1;

    printf("%d", *q);

    return 0;
}
