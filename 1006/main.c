#include <stdio.h>

#include <stdlib.h>
#include <time.h>
#include <stdbool.h>
#include <math.h>

#define LEN 3
#define STREAM_LEN (8 * (LEN))

int main() {
    bool stream[STREAM_LEN];

    int list_true[LEN] = {-1, -1, -1};
    int list[LEN] = {-1, -1, -1};

    int i = 0;

    srand( time(NULL) );

    // rand stream
    for(i = 0; i < STREAM_LEN; i++ ) {
        stream[i] = rand() % 2;
    }

    // print stream
    printf("BIN: \n");
    for(i = 0; i < STREAM_LEN; i++ ) {
        printf("%d", stream[i]);
        if(!((i + 1) % 8)) printf(" ");
    }
    printf("\n");

    // stream to int
    for(i = 0; i < LEN; i++) {
        int k = 0;
        int num = 0;
        for(k = 8 * i; k < 8 * (i + 1); k++) {
            num += stream[k] << (7 - (k - (8 * i)));
            // num += stream[k] * (int)pow(2, (7 - (k - (8 * i))));
        }

        list_true[i] = num;
        list[i] = (num > 228)? 100 : (num < 28)? -100 : num - 128;
    }

    // print true num
    printf("True DEC: \n");
    for(i = 0; i < LEN; i++ ) {
        printf("%d ", list_true[i]);
    }
    printf("\n");

    // print num
    printf("DEC: \n");
    for(i = 0; i < LEN; i++ ) {
        printf("%d ", list[i]);
    }
    printf("\n");

    return 0;
}
