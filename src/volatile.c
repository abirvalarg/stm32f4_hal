#include "type.h"

word volatile_read(volatile word *addr)
{
    return *addr;
}

void volatile_write(volatile word *addr, word value)
{
    *addr = value;
}
