#include <stdio.h>

/*
    Name: Tyler Adam Martinez
    EUID: tam0301
    Section: 206
*/

int main()
{
    printf("Enter an ASCII character : ");

    unsigned char unsigned_char_userChar;
    scanf("%c", &unsigned_char_userChar);

    printf("The ASCII value of %c is:\n", unsigned_char_userChar);
    printf("dec -- %d\n", unsigned_char_userChar);
    printf("hex -- %x\n", unsigned_char_userChar);
    printf("bin -- ");

    int int_temp;
    for (int i = 7; i >= 0; i--)
    {
        int_temp = unsigned_char_userChar >> i;
        if (int_temp & 1)
        {
            printf("1");
        }
        else
        {
            printf("0");
        }
    }

    printf("\n");
    return 0;
}