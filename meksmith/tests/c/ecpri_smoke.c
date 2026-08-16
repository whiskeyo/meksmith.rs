// Minimal compile-time smoke test for examples/ecpri.mek generated codecs.
#include <assert.h>
#include <stdint.h>

#include "ecpri.h"

int main(void) {
    uint8_t wire[] = {0x10, 0x00, 0x00, 0x06};
    Header decoded = {0};
    uint16_t payload_size = 0;

    assert(ecpri_header_decode(wire, sizeof(wire), &payload_size, &decoded) == 0);
    assert(payload_size == 6);
    assert(decoded.revision == ProtocolRevision_v1_through_v2);
    assert(decoded.message_type == MessageType_iq_data);
    return 0;
}
