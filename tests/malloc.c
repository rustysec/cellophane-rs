#include <stdlib.h>
#include <stdio.h>
#include <string.h>

char *malloc_string()  {
    char *data = (char *)malloc(128);
    memset(data, 0, 128);
    sprintf((char*)data, "test 123");
    return (char *)data;
}

struct TestStruct {
    int first;
    int second;
};

void malloc_struct(void **ts) {
    *ts = (struct TestStruct*)malloc(sizeof(struct TestStruct));

    ((struct TestStruct*)(*ts))->first = 1;
    ((struct TestStruct*)(*ts))->second = 2;
}
