#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

typedef enum _meksmith_endianness_t {
    MEKSMITH_ENCODING_LITTLE_ENDIAN,
    MEKSMITH_ENCODING_BIG_ENDIAN
} meksmith_endianness_t;

static inline void meksmith_encode_aligned_bytes(
    uint8_t* buffer,
    const size_t buffer_size,
    const size_t byte_offset,
    const uint64_t value,
    const size_t byte_count,
    const meksmith_endianness_t endianness
) {
    if (byte_offset + byte_count > buffer_size || byte_count > 8) {
        return;
    }

    for (size_t i = 0; i < byte_count; i++) {
        size_t value_index = (endianness == MEKSMITH_ENCODING_LITTLE_ENDIAN) ? i : (byte_count - 1 - i);
        size_t buffer_index = byte_offset + i;
        buffer[buffer_index] = (value >> (value_index * 8)) & 0xFF;
    }
}

uint64_t meksmith_decode_aligned_bytes(
    const uint8_t* buffer,
    const size_t buffer_size,
    const size_t byte_offset,
    const size_t byte_count,
    const meksmith_endianness_t endianness
) {
    if (byte_offset + byte_count > buffer_size || byte_count > 8) {
        return 0;
    }

    uint64_t value = 0;

    for (size_t i = 0; i < byte_count; i++) {
        size_t value_index = (endianness == MEKSMITH_ENCODING_LITTLE_ENDIAN) ? i : (byte_count - 1 - i);
        size_t buffer_index = byte_offset + i;

        value |= ((uint64_t)buffer[buffer_index]) << (value_index * 8);
    }

    return value;
}

static inline void meksmith_encode_bits(
    uint8_t *buffer,
    const size_t buffer_length,
    const size_t byte_offset,
    const uint8_t bit_offset,
    const uint64_t value,
    const uint8_t bit_count,
    const meksmith_endianness_t endianness
) {
    size_t start_bit = byte_offset * 8u + bit_offset;

    for (uint8_t i = 0; i < bit_count; i++) {
        uint8_t bit_index_in_value = endianness == MEKSMITH_ENCODING_LITTLE_ENDIAN ? i : (bit_count - 1u - i);
        uint8_t cur_bit = (uint8_t)((value >> bit_index_in_value) & 1u);

        size_t bit_pos = start_bit + i;
        size_t byte_index = bit_pos / 8u;
        uint8_t bit_index = bit_pos % 8u;

        if (byte_index < buffer_length) {
            if (cur_bit)
                buffer[byte_index] |= (uint8_t)(1u << bit_index);
            else
                buffer[byte_index] &= (uint8_t)~(1u << bit_index);
        }
    }
}

static inline uint64_t meksmith_decode_bits(
    const uint8_t *buffer,
    const size_t buffer_length,
    const size_t byte_offset,
    const uint8_t bit_offset,
    const uint8_t bit_count,
    const meksmith_endianness_t endianness
) {
    size_t start_bit = byte_offset * 8u + bit_offset;
    uint64_t result = 0;

    for (uint8_t i = 0; i < bit_count; i++) {
        size_t bit_pos = start_bit + i;
        size_t byte_index = bit_pos / 8u;
        uint8_t bit_index = bit_pos % 8u;

        uint8_t cur_bit = 0;
        if (byte_index < buffer_length)
            cur_bit = (buffer[byte_index] >> bit_index) & 1u;

        if (endianness == MEKSMITH_ENCODING_LITTLE_ENDIAN) {
            result |= (uint64_t)cur_bit << i;
        } else {
            result |= (uint64_t)cur_bit << (bit_count - 1u - i);
        }
    }

    return result;
}
