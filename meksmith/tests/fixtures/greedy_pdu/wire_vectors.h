/* Wire-byte fixtures for tests/protocols/greedy_pdu.mek */

#ifndef MEKSMITH_GREEDY_PDU_WIRE_VECTORS_H
#define MEKSMITH_GREEDY_PDU_WIRE_VECTORS_H

#include <stddef.h>
#include <stdint.h>

/* Requires greedydemo.h to be included before this header. */

typedef struct {
    const char *name;
    size_t frame_count;
    const uint8_t *wire;
    size_t wire_len;
} pdu_vector_t;

static const uint8_t FRAME_2_BYTES[] = {0xAA, 0xBB};
static const uint8_t FRAME_1_BYTE[] = {0xCC};
static const uint8_t TWO_FRAMES_WIRE[] = {0x00, 0x02, 0xAA, 0xBB, 0x00, 0x01, 0xCC};

static const pdu_vector_t PDU_VECTORS[] = {
    {
        .name = "two_frames",
        .frame_count = 2,
        .wire = TWO_FRAMES_WIRE,
        .wire_len = sizeof(TWO_FRAMES_WIRE),
    },
};

#define PDU_VECTOR_COUNT (sizeof(PDU_VECTORS) / sizeof(PDU_VECTORS[0]))

#endif
