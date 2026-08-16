/* Wire-byte fixtures for tests/protocols/bitpack_header.mek */

#ifndef MEKSMITH_BITPACK_WIRE_VECTORS_H
#define MEKSMITH_BITPACK_WIRE_VECTORS_H

#include <stddef.h>
#include <stdint.h>

/* Requires bitpackdemo.h to be included before this header. */

typedef struct {
    const char *name;
    Header header;
    uint16_t body_size;
    uint8_t wire[4];
} header_vector_t;

static const header_vector_t HEADER_VECTORS[] = {
    {
        .name = "v1_noflag_alpha_bs6",
        .header = {
            .version = Version_one,
            .reserved = 0,
            .flag = 0,
            .kind = Kind_alpha,
        },
        .body_size = 6,
        .wire = {0x10, 0x00, 0x00, 0x06},
    },
    {
        .name = "v1_flag_beta_bs100",
        .header = {
            .version = Version_one,
            .reserved = 0,
            .flag = 1,
            .kind = Kind_beta,
        },
        .body_size = 100,
        .wire = {0x11, 0x01, 0x00, 0x64},
    },
    {
        .name = "v0_noflag_alpha_bs8",
        .header = {
            .version = Version_zero,
            .reserved = 0,
            .flag = 0,
            .kind = Kind_alpha,
        },
        .body_size = 8,
        .wire = {0x00, 0x00, 0x00, 0x08},
    },
};

#define HEADER_VECTOR_COUNT (sizeof(HEADER_VECTORS) / sizeof(HEADER_VECTORS[0]))

#endif
