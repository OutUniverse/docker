# include <stdio.h>

int main (void) {
    int i1, i2;

    scanf("%d", &i1);

    freopen("test.txt", "r", stdin);
    scanf("%d", &i2);

    printf("%d %d", i1, i2);
}
