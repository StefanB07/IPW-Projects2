#include <string.h>
#include <stdio.h>
#define goodPass ”GOODPASS”

int main()
{
    char passIsGood = 0;
    char buf[80];

    printf(“Enter password : \n”);
    gets(buf);

    if (strcmp(buf, goodPass) == 0)
        passIsGood = 1;
    if (passIsGood == 1)
        printf(”You win ! \ n”);
}
