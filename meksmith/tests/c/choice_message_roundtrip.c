// Encode → decode → assert roundtrips for tests/protocols/choice_message.mek
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "choicedemo.h"
#include "wire_vectors.h"

#define WIRE_BUF_SIZE 256

static void assert_messages_equal(const Message *expected, const Message *actual) {
    assert(expected->header.kind == actual->header.kind);
    assert(expected->body.tag == actual->body.tag);
    switch (expected->body.tag) {
    case Kind_alpha:
        assert(expected->body.body.alpha.tag == actual->body.body.alpha.tag);
        assert(expected->body.body.alpha.rest.len == actual->body.body.alpha.rest.len);
        if (expected->body.body.alpha.rest.len > 0) {
            assert(memcmp(
                expected->body.body.alpha.rest.data,
                actual->body.body.alpha.rest.data,
                expected->body.body.alpha.rest.len) == 0);
        }
        break;
    case Kind_beta:
        assert(expected->body.body.beta.code == actual->body.body.beta.code);
        assert(expected->body.body.beta.rest.len == actual->body.body.beta.rest.len);
        if (expected->body.body.beta.rest.len > 0) {
            assert(memcmp(
                expected->body.body.beta.rest.data,
                actual->body.body.beta.rest.data,
                expected->body.body.beta.rest.len) == 0);
        }
        break;
    default:
        assert(0 && "unexpected kind");
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

static void roundtrip_message_vector(const message_vector_t *vector) {
    uint8_t rest_buf[WIRE_BUF_SIZE] = {0};
    Message original = vector->message;

    switch (original.body.tag) {
    case Kind_alpha:
        original.body.body.alpha.rest.data = rest_buf;
        break;
    case Kind_beta:
        original.body.body.beta.rest.data = rest_buf;
        break;
    default:
        break;
    }

    uint8_t wire[WIRE_BUF_SIZE];
    size_t wire_len = 0;
    assert(choicedemo_message_encode(&original, wire, sizeof(wire), &wire_len) == 0);
    assert_wire_equal(vector->name, wire, wire_len, vector->wire, vector->wire_len);

    Message decoded = {0};
    uint8_t out_rest[WIRE_BUF_SIZE] = {0};
    decoded.body.body.alpha.rest.data = out_rest;
    assert(choicedemo_message_decode(wire, wire_len, &decoded) == 0);
    assert_messages_equal(&original, &decoded);
}

int main(void) {
    for (size_t i = 0; i < MESSAGE_VECTOR_COUNT; ++i) {
        roundtrip_message_vector(&MESSAGE_VECTORS[i]);
    }
    return 0;
}
