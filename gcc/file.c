# include <stdio.h>

int main (void) {
    char arr[] = {'a', 'c', 'z', 'q', 'h'};

    FILE* fp = fopen("../test.txt", "wb");

    fwrite(arr, sizeof(arr[0]), sizeof(arr) / sizeof(arr[0]), fp);
    
    fclose(fp);

    FILE* fpr = fopen("../test.txt", "rb");

    char c;

    while (fread(&c, sizeof(char), 1, fpr) > 0)
    {
        printf("%c", c);
    }
    
    fclose(fp);
}
