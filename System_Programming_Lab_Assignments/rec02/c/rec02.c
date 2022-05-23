#include <stdio.h>
#include <stdlib.h>

/*
    Name: Tyler Adam Martinez
    EUID: tam0301
    Section: 206
*/

int main(int argc, char const *argv[])
{
    int *int_ptr;
    int_ptr = (int *)malloc(2 * sizeof(int));

    printf("Enter first integer: ");
    scanf("%d", &int_ptr[0]);
    printf("Enter second integer: ");
    scanf("%d", &int_ptr[1]);

    if (!int_ptr)
    {
        printf("Program failed to properly store values\n");
        printf("Try to enter a different integer... \n");
        exit(0);
    }

    printf("Original values: 1st = %d 2nd = %d \n", int_ptr[0], int_ptr[1]);

    int_ptr[0] = int_ptr[0] ^ int_ptr[1];
    int_ptr[1] = int_ptr[1] ^ int_ptr[0];
    int_ptr[0] = int_ptr[0] ^ int_ptr[1];

    printf("Swapped values: 1st = %d 2nd = %d \n", int_ptr[0], int_ptr[1]);

    free(int_ptr);
    return 0;
}
