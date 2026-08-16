// Encode → decode → assert roundtrips for tests/protocols/greedy_pdu.mek
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "greedydemo.h"
#include "wire_vectors.h"

#define WIRE_BUF_SIZE 256
#define MAX_FRAMES 8

static void assert_frames_equal(const Frame *expected, const Frame *actual) {
    assert(expected->len == actual->len);
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

static void roundtrip_two_frames_vector(const pdu_vector_t *vector) {
    uint8_t frame0_bytes[] = {0xAA, 0xBB};
    uint8_t frame1_bytes[] = {0xCC};
    Frame frames[2] = {
        {.len = 2, .bytes = {.data = frame0_bytes, .len = 2}},
        {.len = 1, .bytes = {.data = frame1_bytes, .len = 1}},
    };
    Pdu original = {
        .frames = {.data = frames, .len = 2},
    };

    uint8_t wire[WIRE_BUF_SIZE];
    size_t wire_len = 0;
    assert(greedydemo_pdu_encode(&original, wire, sizeof(wire), &wire_len) == 0);
    assert_wire_equal(vector->name, wire, wire_len, vector->wire, vector->wire_len);

    Frame decoded_frames[MAX_FRAMES];
    uint8_t decoded_bytes[MAX_FRAMES][WIRE_BUF_SIZE];
    for (size_t i = 0; i < MAX_FRAMES; ++i) {
        decoded_frames[i].bytes.data = decoded_bytes[i];
    }

    Pdu decoded = {
        .frames = {.data = decoded_frames, .len = MAX_FRAMES},
    };
    assert(greedydemo_pdu_decode(wire, wire_len, &decoded) == 0);
    assert(decoded.frames.len == vector->frame_count);
    for (size_t i = 0; i < vector->frame_count; ++i) {
        assert_frames_equal(&frames[i], &decoded.frames.data[i]);
    }
}

int main(void) {
    for (size_t i = 0; i < PDU_VECTOR_COUNT; ++i) {
        roundtrip_two_frames_vector(&PDU_VECTORS[i]);
    }
    return 0;
}
