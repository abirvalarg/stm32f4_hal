#pragma once

typedef signed char int8_t;
typedef unsigned char uint8_t;
typedef signed short int16_t;
typedef unsigned short uint16_t;
typedef signed int int32_t;
typedef unsigned int uint32_t;

typedef uint8_t byte;
typedef uint16_t halfword;
typedef uint32_t word;

#ifndef __cplusplus
typedef byte bool;
#endif

#define CHECK(type, size) _Static_assert(sizeof(type) == size, "Wrong size of " #type)

CHECK(int8_t, 1);
CHECK(uint8_t, 1);
CHECK(int16_t, 2);
CHECK(uint16_t, 2);
CHECK(int32_t, 4);
CHECK(uint32_t, 4);
CHECK(byte, 1);
CHECK(halfword, 2);
CHECK(word, 4);

#undef CHECK

#define false 0
#define true 1
#define NULL 0
#define BIT(n) (1 << n)
