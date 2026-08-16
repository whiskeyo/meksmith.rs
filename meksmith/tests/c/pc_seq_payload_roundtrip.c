// Encode → decode → assert roundtrips for tests/protocols/pc_seq_payload.mek
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "pcseqdemo.h"
#include "wire_vectors.h"

#define WIRE_BUF_SIZE 256

static void assert_pc_seq_equal(const PcSeqPayload *expected, const PcSeqPayload *actual) {
    assert(expected->lane_id == actual->lane_id);
    assert(expected->part_id == actual->part_id);
    assert(expected->bytes.len == actual->bytes.len);
    if (expected->bytes.len > 0) {
        assert(memcmp(expected->bytes.data, actual->bytes.data, expected->bytes.len) == 0);
    }
}

static void assert_wire_equal(const char *name, const uint8_t *actual, size_t actual_len,
                              const uint8_t *expected, size_t expected_len) {
    if (actual_len != expected_len || memcmp(actual, expected, expected_len) != 0) {
        fprintf(stderr, "wire mismatch in %s\n", name);
        assert(actual_len == expected_len);
        assert(memcmp(actual, expected, expected_len) == 0);
    }
}

static void roundtrip_pc_seq_vector(const pc_seq_payload_vector_t *vector) {
    uint8_t byte_buf[WIRE_BUF_SIZE];
    if (vector->byte_len > 0) {
        memcpy(byte_buf, vector->bytes, vector->byte_len);
    }

    PcSeqPayload original = {
        .lane_id = vector->lane_id,
        .part_id = vector->part_id,
        .bytes = {.data = byte_buf, .len = vector->byte_len},
    };

    uint8_t wire[WIRE_BUF_SIZE];
    size_t wire_len = 0;
    assert(pcseqdemo_pc_seq_payload_encode(&original, wire, sizeof(wire), &wire_len) == 0);
    assert(wire_len == vector->payload_size);
    assert_wire_equal(vector->name, wire, wire_len, vector->wire, vector->wire_len);

    uint8_t out_bytes[WIRE_BUF_SIZE] = {0};
    PcSeqPayload decoded = {0};
    decoded.bytes.data = out_bytes;
    assert(pcseqdemo_pc_seq_payload_decode(wire, wire_len, vector->payload_size, &decoded) == 0);
    assert_pc_seq_equal(&original, &decoded);
}

int main(void) {
    for (size_t i = 0; i < PC_SEQ_VECTOR_COUNT; ++i) {
        roundtrip_pc_seq_vector(&PC_SEQ_VECTORS[i]);
    }
    return 0;
}
