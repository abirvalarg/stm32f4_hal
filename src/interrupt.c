#include "malloc.h"
#include "type.h"

extern byte _STACK, _DATA_START, _DATA_END, _DATA_VAL_START, _BSS_START, _BSS_END,
    _HEAP_START, _HEAP_END;

static volatile byte irqMaskCount = 0;

void _reset();
void _nmi();
void _hardfault();
void _tim3();
void _tim4();
void _tim6_dac();
void _tim7();
void _usart1();
void _usart3();
void start();

void mask_irq()
{
    asm("cpsid i");
    irqMaskCount++;
}

void unmask_irq()
{
    if(--irqMaskCount == 0)
        asm("cpsie i");
}

__attribute__((used, section(".vector")))
const word *__vector[] = {
    (word*)&_STACK,
    (word*)_reset,
    (word*)_nmi,
    (word*)_hardfault,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 10
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 20
    0, 0, 0, 0, 0, 0, 0, 0, 0, (word*)_tim3, // 30
    (word*)_tim4, 0, 0, 0, 0, 0, 0, (word*)_usart1, 0, (word*)_usart3, // 40
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 50
    0, 0, 0, 0,
    (word*)_tim6_dac,
    (word*)_tim7
};

__attribute__((noreturn))
void _reset()
{
    for(byte *dest = &_DATA_START, *src = &_DATA_VAL_START; dest != &_DATA_END; dest++, src++)
        *dest = *src;
    
    for(byte *dest = &_BSS_START; dest != &_BSS_END; dest++)
        *dest = 0;
    
    init_heap((word*)&_HEAP_START, (word*)&_HEAP_END);
    
    *(word*)0xE000ED88 = 0xf << 20;
    asm("dsb");
    asm("isb");
    start();
    while(1);
}
