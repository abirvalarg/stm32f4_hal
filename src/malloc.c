#include "malloc.h"

static struct AreaInfo *heap;

void init_heap(word *start, word *end)
{
    word size = end - start - 2;
    struct AreaInfo *header = (struct AreaInfo*)start;
    header->taken = 0;
    header->size = size / 4;
    struct AreaInfo *tail = (struct AreaInfo*)end - 1;
    tail->size = 0;
    heap = header;
}

void *malloc(word size)
{
    if(size == 0)
        size = 1;
    word words = size / 4 + (size % 4 != 0);
    struct AreaInfo *area = heap;
    while(area->taken && area->size && area->size < words)
    {
        area = area + area->size + 1;
    }
    if(!area->size)
        return NULL;
    word remSize = area->size - words - 1;
    if(remSize == 1)
        area->size = words + 1;
    else
    {
        struct AreaInfo *next = area + words + 1;
        next->size = remSize;
        next->taken = 0;
        area->size = words;
    }
    return area + 1;
}

void free(void *mem)
{
    struct AreaInfo *area = (struct AreaInfo*)mem - 1;
    area->taken = 0;
    struct AreaInfo *next = area + area->size + 1;
    if(!next->taken)
        area->size += next->size + 1;
}
