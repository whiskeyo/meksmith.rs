/* Wire-byte fixtures for tests/protocols/choice_message.mek */

#ifndef MEKSMITH_CHOICE_MESSAGE_WIRE_VECTORS_H
#define MEKSMITH_CHOICE_MESSAGE_WIRE_VECTORS_H

#include <stddef.h>
#include <stdint.h>

/* Requires choicedemo.h to be included before this header. */

typedef struct {
    const char *name;
    Message message;
    uint16_t body_size;
    const uint8_t *wire;
    size_t wire_len;
} message_vector_t;

static const uint8_t ALPHA_TAG_ONLY_WIRE[] = {0x00, 0x00, 0x01, 0x42};
static const uint8_t BETA_CODE_ONLY_WIRE[] = {0x01, 0x00, 0x02, 0x12, 0x34};

static const message_vector_t MESSAGE_VECTORS[] = {
    {
        .name = "alpha_tag_only",
        .message = {
            .header = {.kind = Kind_alpha},
            .body = {
                .tag = Kind_alpha,
                .body = {.alpha = {.tag = 0x42, .rest = {.data = NULL, .len = 0}}},
            },
        },
        .body_size = 1,
        .wire = ALPHA_TAG_ONLY_WIRE,
        .wire_len = sizeof(ALPHA_TAG_ONLY_WIRE),
    },
    {
        .name = "beta_code_only",
        .message = {
            .header = {.kind = Kind_beta},
            .body = {
                .tag = Kind_beta,
                .body = {.beta = {.code = 0x1234, .rest = {.data = NULL, .len = 0}}},
            },
        },
        .body_size = 2,
        .wire = BETA_CODE_ONLY_WIRE,
        .wire_len = sizeof(BETA_CODE_ONLY_WIRE),
    },
};

#define MESSAGE_VECTOR_COUNT (sizeof(MESSAGE_VECTORS) / sizeof(MESSAGE_VECTORS[0]))

#endif
