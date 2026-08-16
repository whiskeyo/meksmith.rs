// Encode → decode → assert roundtrips for tests/protocols/bitpack_header.mek
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "bitpackdemo.h"
#include "wire_vectors.h"

#define WIRE_BUF_SIZE 64

static void assert_headers_equal(const Header *expected, const Header *actual) {
    assert(expected->version == actual->version);
    assert(expected->reserved == actual->reserved);
    assert(expected->flag == actual->flag);
    assert(expected->kind == actual->kind);
}

static void assert_wire_equal(const char *name, const uint8_t *actual, size_t actual_len,
                              const uint8_t *expected, size_t expected_len) {
    if (actual_len != expected_len || memcmp(actual, expected, expected_len) != 0) {
        fprintf(stderr, "wire mismatch in %s\n", name);
        assert(actual_len == expected_len);
        assert(memcmp(actual, expected, expected_len) == 0);
    }
}

static void roundtrip_header_vector(const header_vector_t *vector) {
    uint8_t wire[WIRE_BUF_SIZE];
    size_t wire_len = 0;

    assert(bitpackdemo_header_encode(&vector->header, vector->body_size, wire, sizeof(wire), &wire_len) == 0);
    assert(wire_len == 4);
    assert_wire_equal(vector->name, wire, wire_len, vector->wire, 4);

    Header decoded = {0};
    uint16_t body_size = 0;
    assert(bitpackdemo_header_decode(wire, wire_len, &body_size, &decoded) == 0);
    assert(body_size == vector->body_size);
    assert_headers_equal(&vector->header, &decoded);
}

static void decode_fixed_header_vectors(void) {
    for (size_t i = 0; i < HEADER_VECTOR_COUNT; ++i) {
        const header_vector_t *vector = &HEADER_VECTORS[i];
        Header decoded = {0};
        uint16_t body_size = 0;
        assert(bitpackdemo_header_decode(vector->wire, 4, &body_size, &decoded) == 0);
        assert(body_size == vector->body_size);
        assert_headers_equal(&vector->header, &decoded);
    }
}

int main(void) {
    for (size_t i = 0; i < HEADER_VECTOR_COUNT; ++i) {
        roundtrip_header_vector(&HEADER_VECTORS[i]);
    }
    decode_fixed_header_vectors();
    return 0;
}
