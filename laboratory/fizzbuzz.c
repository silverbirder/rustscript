#include <stdio.h>

void fizzbuzz(int n)
{
     // if (n % 3 == 0 && n % 5 == 0)
     // {
     //      printf("Fizz,Buzz\n");
     // }
     // else if (n % 3 == 0)
     // {
     //      printf("Fizz\n");
     // }
     // else if (n % 5 == 0)
     // {
     //      printf("Buzz\n");
     // }

     switch (n % 15)
     {
     case 3:
     case 6:
     case 9:
     case 12:
          printf("%s", "Fizz");
          break;
     case 0:
          printf("%s", "Fizz");
     case 5:
     case 10:
          printf("%s", "Buzz");
          break;
     default:
          printf("%d", n);
     }
}

int main(void)
{
     printf("Please input the number.\n");

     int i;
     scanf("%d", &i);

     fizzbuzz(i);

     return 0;
}