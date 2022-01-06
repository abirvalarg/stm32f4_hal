#pragma once
#include "type.h"

struct AreaInfo
{
    word size: 31;
    bool taken: 1;
};

void init_heap(word *start, word *end);
void *malloc(word size);
void free(void *mem);
