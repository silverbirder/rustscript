#include <stdio.h>
#include <stdlib.h>

int main(void)
{
     int i = rand() % 5;
     for (int count = 1; count <= i; count = count + 1)
          printf("hello\n");

     return 0;
}