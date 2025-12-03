#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

long max_joltage(char **ptr, int len, char *digits) {
    char *eol = strchr(*ptr, '\n');
    bool last = false;

    if (eol == NULL) {
        last = true;
        eol = strchr(*ptr, '\0');
    }

    char max = '0';
    char *ptr_max = *ptr;
    char *start = *ptr;
    int space = len;

    while (*ptr < eol && space > 0) {
        if (**ptr > max) {
            max = **ptr;
            ptr_max = *ptr;
        }

        if ((eol - *ptr) <= space || max == '9') {
            // use the current max digit
            digits[len - space] = max;
            *ptr = ptr_max + 1;
            max = '0';
            ptr_max = *ptr;
            space--;
        } else {
            (*ptr)++;
        }
    }

    if (space > 0) {
        printf("line ended but number not fully populated");
    }

    long max_joltage = 0;
    long radix = 1;

    for (int idx = len - 1; idx >= 0; idx--) {
        max_joltage += radix * (digits[idx] - '0');
        radix *= 10;
    }

    *ptr = eol + 1;
    return max_joltage;
}

int main() {
    FILE *f = fopen("day03_large.txt", "r");

    struct stat st;
    fstat(fileno(f), &st);
    size_t size = st.st_size;

    char *buf = malloc(size + 1);
    fread(buf, 1, size, f);
    buf[size] = '\0';
    fclose(f);

    long total1 = 0;
    char digits1[2];

    char *ptr = buf;
    while (*ptr) {
        total1 += max_joltage(&ptr, 2, digits1);
    }

    long total2 = 0;
    char digits2[12];

    ptr = buf;
    while (*ptr) {
        total2 += max_joltage(&ptr, 12, digits2);
    }

    printf("Part 1: %li\n", total1);
    printf("Part 2: %li\n", total2);

    return 0;
}
