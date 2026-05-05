// #include <stdio.h>
// #include <stdlib.h>
// int main(){
//     int x = 10;
//     void *vp = &x;
//     printf("%d\n", *(int *)vp);
//     void *ptr = malloc(128);
//     printf("%p\n",ptr);
//     free(ptr);
//     int a;
//     scanf("%d", &a);
//     int *a = (int*) malloc(10 *sizeof(int));
//     printf("%p %p %p %p\n", a, a+1, a+2, a);
//     free(a);
//     return 0;
// }
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int x = 10;
    void *vp = &x;
    printf("%d\n", *(int *)vp);

    void *ptr = malloc(128);
    printf("%p\n", ptr);
    free(ptr);

    int val;
    scanf("%d", &val);

    int *arr = malloc(10 * sizeof(int));
    if (!arr) return 1;

    printf("%p %p %p %p\n", arr, arr+1, arr+2, arr+3);
    free(arr);

    return 0;
}
