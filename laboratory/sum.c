#include <stdio.h>

int sum(int l, int r)
{
    return l + r;
}

int main(void)
{
    int l = 1;
    int r = 2;
    int result = sum(l, r);
    printf("%d", result);
    return 0;
}