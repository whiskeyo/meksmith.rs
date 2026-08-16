/* Wire-byte fixtures for tests/protocols/pc_seq_payload.mek */

#ifndef MEKSMITH_PC_SEQ_WIRE_VECTORS_H
#define MEKSMITH_PC_SEQ_WIRE_VECTORS_H

#include <stddef.h>
#include <stdint.h>

/* Requires pcseqdemo.h to be included before this header. */

typedef struct {
    const char *name;
    uint16_t lane_id;
    uint16_t part_id;
    const uint8_t *bytes;
    size_t byte_len;
    uint16_t payload_size;
    const uint8_t *wire;
    size_t wire_len;
} pc_seq_payload_vector_t;

static const uint8_t BYTES_EMPTY[] = {};
static const uint8_t BYTES_4[] = {0xde, 0xad, 0xbe, 0xef};
static const uint8_t BYTES_1[] = {0x55};

static const uint8_t EMPTY_WIRE[] = {0xff, 0xff, 0x00, 0x01};
static const uint8_t FOUR_BYTES_WIRE[] = {0x12, 0x34, 0x56, 0x78, 0xde, 0xad, 0xbe, 0xef};
static const uint8_t ONE_BYTE_WIRE[] = {0x00, 0x01, 0x00, 0x02, 0x55};

static const pc_seq_payload_vector_t PC_SEQ_VECTORS[] = {
    {
        .name = "empty_trailer",
        .lane_id = 0xffff,
        .part_id = 0x0001,
        .bytes = BYTES_EMPTY,
        .byte_len = 0,
        .payload_size = 4,
        .wire = EMPTY_WIRE,
        .wire_len = sizeof(EMPTY_WIRE),
    },
    {
        .name = "four_bytes",
        .lane_id = 0x1234,
        .part_id = 0x5678,
        .bytes = BYTES_4,
        .byte_len = sizeof(BYTES_4),
        .payload_size = 8,
        .wire = FOUR_BYTES_WIRE,
        .wire_len = sizeof(FOUR_BYTES_WIRE),
    },
    {
        .name = "one_byte",
        .lane_id = 0x0001,
        .part_id = 0x0002,
        .bytes = BYTES_1,
        .byte_len = sizeof(BYTES_1),
        .payload_size = 5,
        .wire = ONE_BYTE_WIRE,
        .wire_len = sizeof(ONE_BYTE_WIRE),
    },
};

#define PC_SEQ_VECTOR_COUNT (sizeof(PC_SEQ_VECTORS) / sizeof(PC_SEQ_VECTORS[0]))

#endif
