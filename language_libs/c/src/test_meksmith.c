#include "meksmith.h"
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>
#include <inttypes.h>

#define TERM_RED     "\033[31m"
#define TERM_GREEN   "\033[32m"
#define TERM_YELLOW  "\033[33m"
#define TERM_CANCEL  "\033[0m"

static uint32_t tests_count = 0;
static uint32_t tests_passed = 0;
static uint32_t tests_failed = 0;

static inline void meksmith_print_buffer(const char* message, const uint8_t* buffer, size_t length) {
    printf("%s", message);
    for (size_t i = 0; i < length; i++) {
        printf("%02X ", buffer[i]);
    }
    printf("\n");
}

static inline bool are_buffers_equal(const uint8_t* buffer1, const uint8_t* buffer2, size_t length) {
    for (size_t i = 0; i < length; i++) {
        if (buffer1[i] != buffer2[i]) {
            return false;
        }
    }

    return true;
}

void test_meksmith_encode_aligned_bytes(
    const size_t buffer_size,
    const size_t byte_offset,
    const size_t byte_count,
    const uint64_t value,
    const meksmith_endianness_t endianness,
    const uint8_t* expected_buffer
) {
    tests_count++;

    const char* endianness_str = endianness == MEKSMITH_ENCODING_LITTLE_ENDIAN ? "little-endian" : "big-endian";
    printf("%sTest (test_meksmith_encode_aligned_bytes): buffer size: %zu, value: %" PRIu64 " (0x%016" PRIX64 "), byte offset: %zu, byte count: %zu, endianness: %s%s\n",
        TERM_YELLOW, buffer_size, value, value, byte_offset, byte_count, endianness_str, TERM_CANCEL);

    meksmith_print_buffer("\tExpected buffer: ", expected_buffer, buffer_size);

    uint8_t* buffer = (uint8_t*)malloc(buffer_size);
    if (!buffer) {
        printf("%sError: malloc failed%s\n", TERM_RED, TERM_CANCEL);
        tests_failed++;
        return;
    }
    for (size_t i = 0; i < buffer_size; i++) {
        buffer[i] = 0;
    }

    meksmith_encode_aligned_bytes(buffer, buffer_size, byte_offset, value, byte_count, endianness);
    meksmith_print_buffer("\tBuffer: ", buffer, buffer_size);

    bool equal = are_buffers_equal(buffer, expected_buffer, buffer_size);
    if (equal) {
        printf("\t%sSuccess: both buffers are equal%s\n", TERM_GREEN, TERM_CANCEL);
        tests_passed++;
    } else {
        printf("\t%sFailure: buffers are not equal%s\n", TERM_RED, TERM_CANCEL);
        tests_failed++;
    }
    free(buffer);
}

void test_little_endian_encoding() {
    test_meksmith_encode_aligned_bytes(1, 0, 1, 0xAB, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0xAB});
    test_meksmith_encode_aligned_bytes(2, 0, 2, 0x1234uLL, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0x34, 0x12});
    test_meksmith_encode_aligned_bytes(2, 0, 1, 0x12uLL, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0x12, 0x00});
    test_meksmith_encode_aligned_bytes(4, 0, 4, 0x12345678uLL, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0x78, 0x56, 0x34, 0x12});
    test_meksmith_encode_aligned_bytes(4, 0, 4, 0xFFFFFFFFuLL, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0xFF, 0xFF, 0xFF, 0xFF});
    test_meksmith_encode_aligned_bytes(4, 1, 2, 0xABCDuLL, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0x00, 0xCD, 0xAB, 0x00});
    test_meksmith_encode_aligned_bytes(8, 2, 4, 0x11223344uLL, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0x00, 0x00, 0x44, 0x33, 0x22, 0x11, 0x00, 0x00});
}

void test_big_endian_encoding() {
    test_meksmith_encode_aligned_bytes(1, 0, 1, 0xAB, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0xAB});
    test_meksmith_encode_aligned_bytes(2, 0, 2, 0x1234uLL, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x12, 0x34});
    test_meksmith_encode_aligned_bytes(2, 0, 1, 0x12uLL, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x12, 0x00});
    test_meksmith_encode_aligned_bytes(4, 0, 4, 0x12345678uLL, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x12, 0x34, 0x56, 0x78});
    test_meksmith_encode_aligned_bytes(4, 0, 4, 0xFFFFFFFFuLL, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0xFF, 0xFF, 0xFF, 0xFF});
    test_meksmith_encode_aligned_bytes(4, 1, 2, 0xABCDuLL, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x00, 0xAB, 0xCD, 0x00});
    test_meksmith_encode_aligned_bytes(8, 2, 4, 0x11223344uLL, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x00, 0x00});
}

void test_meksmith_decode_aligned_bytes(
    const uint8_t* buffer,
    const size_t buffer_size,
    const size_t byte_offset,
    const size_t byte_count,
    const meksmith_endianness_t endianness,
    const uint64_t expected_value
) {
    tests_count++;

    const char* endianness_str = endianness == MEKSMITH_ENCODING_LITTLE_ENDIAN ? "little-endian" : "big-endian";
    printf("%sTest (test_meksmith_decode_aligned_bytes): buffer size: %zu, byte offset: %zu, byte count: %zu, endianness: %s%s\n",
        TERM_YELLOW, buffer_size, byte_offset, byte_count, endianness_str, TERM_CANCEL);

    meksmith_print_buffer("\tBuffer: ", buffer, buffer_size);
    printf("\tExpected value: %" PRIu64 " (0x%016" PRIX64 ")\n", expected_value, expected_value);

    uint64_t value = meksmith_decode_aligned_bytes(buffer, buffer_size, byte_offset, byte_count, endianness);
    printf("\tDecoded value:  %" PRIu64 " (0x%016" PRIX64 ")\n", value, value);

    if (value == expected_value) {
        printf("\t%sSuccess: both values are equal%s\n", TERM_GREEN, TERM_CANCEL);
        tests_passed++;
    } else {
        printf("\t%sFailure: values are not equal%s\n", TERM_RED, TERM_CANCEL);
        tests_failed++;
    }
}

void test_little_endian_decoding() {
    test_meksmith_decode_aligned_bytes((uint8_t[]){0xAB}, 1, 0, 1, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0xAB);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x34, 0x12}, 2, 0, 2, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0x1234uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x12, 0x00}, 2, 0, 1, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0x12uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x78, 0x56, 0x34, 0x12}, 4, 0, 4, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0x12345678uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0xFF, 0xFF, 0xFF, 0xFF}, 4, 0, 4, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0xFFFFFFFFuLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0xCD, 0xAB, 0x00}, 4, 1, 2, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0xABCDuLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0x00, 0x44, 0x33, 0x22, 0x11, 0x00, 0x00}, 8, 2, 4, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0x11223344uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0x00, 0xAB, 0x00, 0x00}, 5, 2, 1, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0xABuLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0x34, 0x12, 0x00, 0x00}, 5, 1, 2, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0x1234uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0x00, 0x78, 0x56, 0x34, 0x12, 0x00, 0x00}, 8, 2, 4, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0x12345678uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22, 0x33, 0x44}, 10, 4, 4, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0x2211FFEEuLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0x00, 0x00, 0xCD, 0xAB, 0x00, 0x00}, 7, 3, 2, MEKSMITH_ENCODING_LITTLE_ENDIAN, 0xABCDuLL);
}

void test_big_endian_decoding() {
    test_meksmith_decode_aligned_bytes((uint8_t[]){0xAB}, 1, 0, 1, MEKSMITH_ENCODING_BIG_ENDIAN, 0xAB);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x12, 0x34}, 2, 0, 2, MEKSMITH_ENCODING_BIG_ENDIAN, 0x1234uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x12, 0x00}, 2, 0, 1, MEKSMITH_ENCODING_BIG_ENDIAN, 0x12uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x12, 0x34, 0x56, 0x78}, 4, 0, 4, MEKSMITH_ENCODING_BIG_ENDIAN, 0x12345678uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0xFF, 0xFF, 0xFF, 0xFF}, 4, 0, 4, MEKSMITH_ENCODING_BIG_ENDIAN, 0xFFFFFFFFuLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0xAB, 0xCD, 0x00}, 4, 1, 2, MEKSMITH_ENCODING_BIG_ENDIAN, 0xABCDuLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0x00, 0x11, 0x22, 0x33, 0x44, 0x00, 0x00}, 8, 2, 4, MEKSMITH_ENCODING_BIG_ENDIAN, 0x11223344uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0x00, 0xAB, 0x00, 0x00}, 5, 2, 1, MEKSMITH_ENCODING_BIG_ENDIAN, 0xABuLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0x12, 0x34, 0x00, 0x00}, 5, 1, 2, MEKSMITH_ENCODING_BIG_ENDIAN, 0x1234uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0x00, 0x12, 0x34, 0x56, 0x78, 0x00, 0x00}, 8, 2, 4, MEKSMITH_ENCODING_BIG_ENDIAN, 0x12345678uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22, 0x33, 0x44}, 10, 4, 4, MEKSMITH_ENCODING_BIG_ENDIAN, 0xEEFF1122uLL);
    test_meksmith_decode_aligned_bytes((uint8_t[]){0x00, 0x00, 0x00, 0xAB, 0xCD, 0x00, 0x00}, 7, 3, 2, MEKSMITH_ENCODING_BIG_ENDIAN, 0xABCDuLL);
}

void test_encoding_decoding_aligned_bytes(
    const size_t buffer_size,
    const uint64_t value,
    const size_t byte_offset,
    const size_t byte_count,
    const meksmith_endianness_t endianness,
    const uint8_t *expected_buffer
) {
    tests_count++;

    const char* endianness_str = endianness == MEKSMITH_ENCODING_LITTLE_ENDIAN ? "little-endian" : "big-endian";
    printf("%sTest (test_encoding_decoding_aligned_bytes): buffer size: %zu, value: %" PRIu64 " (0x%016" PRIX64 "), byte offset: %zu, byte count: %zu, endianness: %s%s\n",
        TERM_YELLOW, buffer_size, value, value, byte_offset, byte_count, endianness_str, TERM_CANCEL);
    meksmith_print_buffer("\tExpected buffer: ", expected_buffer, buffer_size);

    uint8_t* buffer = (uint8_t*)malloc(buffer_size);
    if (!buffer) {
        printf("%sError: malloc failed%s\n", TERM_RED, TERM_CANCEL);
        tests_failed++;
        return;
    }
    for (size_t i = 0; i < buffer_size; i++) {
        buffer[i] = 0;
    }

    meksmith_encode_aligned_bytes(buffer, buffer_size, byte_offset, value, byte_count, endianness);
    meksmith_print_buffer("\tBuffer: ", buffer, buffer_size);
    uint64_t extracted_value = meksmith_decode_aligned_bytes(buffer, buffer_size, byte_offset, byte_count, endianness);
    printf("\tComparing original value: %" PRIu64 " (0x%016" PRIX64 ") with extracted value: %" PRIu64 " (0x%016" PRIX64 ")\n",
        value, value, extracted_value, extracted_value);
    if (extracted_value == value) {
        printf("\t%sSuccess: extracted value: %" PRIu64 " (0x%016" PRIX64 ") matches original value: %" PRIu64 " (0x%016" PRIX64 ")%s\n",
            TERM_GREEN, extracted_value, extracted_value, value, value, TERM_CANCEL);
        tests_passed++;
    } else {
        printf("\t%sFailure: extracted value: %" PRIu64 " (0x%016" PRIX64 ") does not match original value: %" PRIu64 " (0x%016" PRIX64 ")%s\n",
            TERM_RED, extracted_value, extracted_value, value, value, TERM_CANCEL);
        tests_failed++;
    }
    free(buffer);
}

void test_little_endian_encoding_decoding() {
    test_encoding_decoding_aligned_bytes(1, 0xAA, 0, 1, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0xAA});
    test_encoding_decoding_aligned_bytes(1, 0x55, 0, 1, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0x55});
    test_encoding_decoding_aligned_bytes(2, 0xAAAA, 0, 2, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0xAA, 0xAA});
    test_encoding_decoding_aligned_bytes(2, 0x5555, 0, 2, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0x55, 0x55});
    test_encoding_decoding_aligned_bytes(4, 0x12345678, 0, 4, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0x78, 0x56, 0x34, 0x12});
    test_encoding_decoding_aligned_bytes(4, 0x9ABCDEF0, 0, 4, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0xF0, 0xDE, 0xBC, 0x9A});
    test_encoding_decoding_aligned_bytes(8, 0x123456789ABCDEF0, 0, 8, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12});
    test_encoding_decoding_aligned_bytes(8, 0x0FEDCBA987654321, 0, 8, MEKSMITH_ENCODING_LITTLE_ENDIAN, (uint8_t[]){0x21, 0x43, 0x65, 0x87, 0x9A, 0xBC, 0xDE, 0xF0});
}

void test_big_endian_encoding_decoding() {
    test_encoding_decoding_aligned_bytes(1, 0xAA, 0, 1, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0xAA});
    test_encoding_decoding_aligned_bytes(1, 0x55, 0, 1, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x55});
    test_encoding_decoding_aligned_bytes(2, 0xAAAA, 0, 2, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0xAA, 0xAA});
    test_encoding_decoding_aligned_bytes(2, 0x5555, 0, 2, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x55, 0x55});
    test_encoding_decoding_aligned_bytes(4, 0x12345678, 0, 4, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x12, 0x34, 0x56, 0x78});
    test_encoding_decoding_aligned_bytes(4, 0x9ABCDEF0, 0, 4, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x9A, 0xBC, 0xDE, 0xF0});
    test_encoding_decoding_aligned_bytes(8, 0x123456789ABCDEF0, 0, 8, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0});
    test_encoding_decoding_aligned_bytes(8, 0x0FEDCBA987654321, 0, 8, MEKSMITH_ENCODING_BIG_ENDIAN, (uint8_t[]){0x0F, 0xED, 0xCB, 0xA9, 0x87, 0x65, 0x43, 0x21});
}

int main() {
    test_little_endian_encoding();
    test_big_endian_encoding();
    test_little_endian_decoding();
    test_big_endian_decoding();
    test_little_endian_encoding_decoding();
    test_big_endian_encoding_decoding();

    printf("%sTests completed. Total: %u, Passed: %u, Failed: %u%s\n", TERM_YELLOW, tests_count, tests_passed, tests_failed, TERM_CANCEL);

    if (tests_failed > 0) {
        return 1;
    }
    return 0;
}
