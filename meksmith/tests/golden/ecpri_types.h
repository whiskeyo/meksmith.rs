#include <stdint.h>
#include <stddef.h>
#include <string.h>

typedef uint16_t SEQ_ID;

typedef enum {
    RemoteResetCodeOp_reserved = 0,
    RemoteResetCodeOp_request = 1,
    RemoteResetCodeOp_response = 2,
    RemoteResetCodeOp_reserved_future_3 = 3,
    RemoteResetCodeOp_reserved_future_4 = 4,
    RemoteResetCodeOp_reserved_future_5 = 5,
    RemoteResetCodeOp_reserved_future_6 = 6,
    RemoteResetCodeOp_reserved_future_7 = 7,
    RemoteResetCodeOp_reserved_future_8 = 8,
    RemoteResetCodeOp_reserved_future_9 = 9,
    RemoteResetCodeOp_reserved_future_10 = 10,
    RemoteResetCodeOp_reserved_future_11 = 11,
    RemoteResetCodeOp_reserved_future_12 = 12,
    RemoteResetCodeOp_reserved_future_13 = 13,
    RemoteResetCodeOp_reserved_future_14 = 14,
    RemoteResetCodeOp_reserved_future_15 = 15,
    RemoteResetCodeOp_reserved_future_16 = 16,
    RemoteResetCodeOp_reserved_future_17 = 17,
    RemoteResetCodeOp_reserved_future_18 = 18,
    RemoteResetCodeOp_reserved_future_19 = 19,
    RemoteResetCodeOp_reserved_future_20 = 20,
    RemoteResetCodeOp_reserved_future_21 = 21,
    RemoteResetCodeOp_reserved_future_22 = 22,
    RemoteResetCodeOp_reserved_future_23 = 23,
    RemoteResetCodeOp_reserved_future_24 = 24,
    RemoteResetCodeOp_reserved_future_25 = 25,
    RemoteResetCodeOp_reserved_future_26 = 26,
    RemoteResetCodeOp_reserved_future_27 = 27,
    RemoteResetCodeOp_reserved_future_28 = 28,
    RemoteResetCodeOp_reserved_future_29 = 29,
    RemoteResetCodeOp_reserved_future_30 = 30,
    RemoteResetCodeOp_reserved_future_31 = 31,
    RemoteResetCodeOp_reserved_future_32 = 32,
    RemoteResetCodeOp_reserved_future_33 = 33,
    RemoteResetCodeOp_reserved_future_34 = 34,
    RemoteResetCodeOp_reserved_future_35 = 35,
    RemoteResetCodeOp_reserved_future_36 = 36,
    RemoteResetCodeOp_reserved_future_37 = 37,
    RemoteResetCodeOp_reserved_future_38 = 38,
    RemoteResetCodeOp_reserved_future_39 = 39,
    RemoteResetCodeOp_reserved_future_40 = 40,
    RemoteResetCodeOp_reserved_future_41 = 41,
    RemoteResetCodeOp_reserved_future_42 = 42,
    RemoteResetCodeOp_reserved_future_43 = 43,
    RemoteResetCodeOp_reserved_future_44 = 44,
    RemoteResetCodeOp_reserved_future_45 = 45,
    RemoteResetCodeOp_reserved_future_46 = 46,
    RemoteResetCodeOp_reserved_future_47 = 47,
    RemoteResetCodeOp_reserved_future_48 = 48,
    RemoteResetCodeOp_reserved_future_49 = 49,
    RemoteResetCodeOp_reserved_future_50 = 50,
    RemoteResetCodeOp_reserved_future_51 = 51,
    RemoteResetCodeOp_reserved_future_52 = 52,
    RemoteResetCodeOp_reserved_future_53 = 53,
    RemoteResetCodeOp_reserved_future_54 = 54,
    RemoteResetCodeOp_reserved_future_55 = 55,
    RemoteResetCodeOp_reserved_future_56 = 56,
    RemoteResetCodeOp_reserved_future_57 = 57,
    RemoteResetCodeOp_reserved_future_58 = 58,
    RemoteResetCodeOp_reserved_future_59 = 59,
    RemoteResetCodeOp_reserved_future_60 = 60,
    RemoteResetCodeOp_reserved_future_61 = 61,
    RemoteResetCodeOp_reserved_future_62 = 62,
    RemoteResetCodeOp_reserved_future_63 = 63,
    RemoteResetCodeOp_reserved_future_64 = 64,
    RemoteResetCodeOp_reserved_future_65 = 65,
    RemoteResetCodeOp_reserved_future_66 = 66,
    RemoteResetCodeOp_reserved_future_67 = 67,
    RemoteResetCodeOp_reserved_future_68 = 68,
    RemoteResetCodeOp_reserved_future_69 = 69,
    RemoteResetCodeOp_reserved_future_70 = 70,
    RemoteResetCodeOp_reserved_future_71 = 71,
    RemoteResetCodeOp_reserved_future_72 = 72,
    RemoteResetCodeOp_reserved_future_73 = 73,
    RemoteResetCodeOp_reserved_future_74 = 74,
    RemoteResetCodeOp_reserved_future_75 = 75,
    RemoteResetCodeOp_reserved_future_76 = 76,
    RemoteResetCodeOp_reserved_future_77 = 77,
    RemoteResetCodeOp_reserved_future_78 = 78,
    RemoteResetCodeOp_reserved_future_79 = 79,
    RemoteResetCodeOp_reserved_future_80 = 80,
    RemoteResetCodeOp_reserved_future_81 = 81,
    RemoteResetCodeOp_reserved_future_82 = 82,
    RemoteResetCodeOp_reserved_future_83 = 83,
    RemoteResetCodeOp_reserved_future_84 = 84,
    RemoteResetCodeOp_reserved_future_85 = 85,
    RemoteResetCodeOp_reserved_future_86 = 86,
    RemoteResetCodeOp_reserved_future_87 = 87,
    RemoteResetCodeOp_reserved_future_88 = 88,
    RemoteResetCodeOp_reserved_future_89 = 89,
    RemoteResetCodeOp_reserved_future_90 = 90,
    RemoteResetCodeOp_reserved_future_91 = 91,
    RemoteResetCodeOp_reserved_future_92 = 92,
    RemoteResetCodeOp_reserved_future_93 = 93,
    RemoteResetCodeOp_reserved_future_94 = 94,
    RemoteResetCodeOp_reserved_future_95 = 95,
    RemoteResetCodeOp_reserved_future_96 = 96,
    RemoteResetCodeOp_reserved_future_97 = 97,
    RemoteResetCodeOp_reserved_future_98 = 98,
    RemoteResetCodeOp_reserved_future_99 = 99,
    RemoteResetCodeOp_reserved_future_100 = 100,
    RemoteResetCodeOp_reserved_future_101 = 101,
    RemoteResetCodeOp_reserved_future_102 = 102,
    RemoteResetCodeOp_reserved_future_103 = 103,
    RemoteResetCodeOp_reserved_future_104 = 104,
    RemoteResetCodeOp_reserved_future_105 = 105,
    RemoteResetCodeOp_reserved_future_106 = 106,
    RemoteResetCodeOp_reserved_future_107 = 107,
    RemoteResetCodeOp_reserved_future_108 = 108,
    RemoteResetCodeOp_reserved_future_109 = 109,
    RemoteResetCodeOp_reserved_future_110 = 110,
    RemoteResetCodeOp_reserved_future_111 = 111,
    RemoteResetCodeOp_reserved_future_112 = 112,
    RemoteResetCodeOp_reserved_future_113 = 113,
    RemoteResetCodeOp_reserved_future_114 = 114,
    RemoteResetCodeOp_reserved_future_115 = 115,
    RemoteResetCodeOp_reserved_future_116 = 116,
    RemoteResetCodeOp_reserved_future_117 = 117,
    RemoteResetCodeOp_reserved_future_118 = 118,
    RemoteResetCodeOp_reserved_future_119 = 119,
    RemoteResetCodeOp_reserved_future_120 = 120,
    RemoteResetCodeOp_reserved_future_121 = 121,
    RemoteResetCodeOp_reserved_future_122 = 122,
    RemoteResetCodeOp_reserved_future_123 = 123,
    RemoteResetCodeOp_reserved_future_124 = 124,
    RemoteResetCodeOp_reserved_future_125 = 125,
    RemoteResetCodeOp_reserved_future_126 = 126,
    RemoteResetCodeOp_reserved_future_127 = 127,
    RemoteResetCodeOp_reserved_future_128 = 128,
    RemoteResetCodeOp_reserved_future_129 = 129,
    RemoteResetCodeOp_reserved_future_130 = 130,
    RemoteResetCodeOp_reserved_future_131 = 131,
    RemoteResetCodeOp_reserved_future_132 = 132,
    RemoteResetCodeOp_reserved_future_133 = 133,
    RemoteResetCodeOp_reserved_future_134 = 134,
    RemoteResetCodeOp_reserved_future_135 = 135,
    RemoteResetCodeOp_reserved_future_136 = 136,
    RemoteResetCodeOp_reserved_future_137 = 137,
    RemoteResetCodeOp_reserved_future_138 = 138,
    RemoteResetCodeOp_reserved_future_139 = 139,
    RemoteResetCodeOp_reserved_future_140 = 140,
    RemoteResetCodeOp_reserved_future_141 = 141,
    RemoteResetCodeOp_reserved_future_142 = 142,
    RemoteResetCodeOp_reserved_future_143 = 143,
    RemoteResetCodeOp_reserved_future_144 = 144,
    RemoteResetCodeOp_reserved_future_145 = 145,
    RemoteResetCodeOp_reserved_future_146 = 146,
    RemoteResetCodeOp_reserved_future_147 = 147,
    RemoteResetCodeOp_reserved_future_148 = 148,
    RemoteResetCodeOp_reserved_future_149 = 149,
    RemoteResetCodeOp_reserved_future_150 = 150,
    RemoteResetCodeOp_reserved_future_151 = 151,
    RemoteResetCodeOp_reserved_future_152 = 152,
    RemoteResetCodeOp_reserved_future_153 = 153,
    RemoteResetCodeOp_reserved_future_154 = 154,
    RemoteResetCodeOp_reserved_future_155 = 155,
    RemoteResetCodeOp_reserved_future_156 = 156,
    RemoteResetCodeOp_reserved_future_157 = 157,
    RemoteResetCodeOp_reserved_future_158 = 158,
    RemoteResetCodeOp_reserved_future_159 = 159,
    RemoteResetCodeOp_reserved_future_160 = 160,
    RemoteResetCodeOp_reserved_future_161 = 161,
    RemoteResetCodeOp_reserved_future_162 = 162,
    RemoteResetCodeOp_reserved_future_163 = 163,
    RemoteResetCodeOp_reserved_future_164 = 164,
    RemoteResetCodeOp_reserved_future_165 = 165,
    RemoteResetCodeOp_reserved_future_166 = 166,
    RemoteResetCodeOp_reserved_future_167 = 167,
    RemoteResetCodeOp_reserved_future_168 = 168,
    RemoteResetCodeOp_reserved_future_169 = 169,
    RemoteResetCodeOp_reserved_future_170 = 170,
    RemoteResetCodeOp_reserved_future_171 = 171,
    RemoteResetCodeOp_reserved_future_172 = 172,
    RemoteResetCodeOp_reserved_future_173 = 173,
    RemoteResetCodeOp_reserved_future_174 = 174,
    RemoteResetCodeOp_reserved_future_175 = 175,
    RemoteResetCodeOp_reserved_future_176 = 176,
    RemoteResetCodeOp_reserved_future_177 = 177,
    RemoteResetCodeOp_reserved_future_178 = 178,
    RemoteResetCodeOp_reserved_future_179 = 179,
    RemoteResetCodeOp_reserved_future_180 = 180,
    RemoteResetCodeOp_reserved_future_181 = 181,
    RemoteResetCodeOp_reserved_future_182 = 182,
    RemoteResetCodeOp_reserved_future_183 = 183,
    RemoteResetCodeOp_reserved_future_184 = 184,
    RemoteResetCodeOp_reserved_future_185 = 185,
    RemoteResetCodeOp_reserved_future_186 = 186,
    RemoteResetCodeOp_reserved_future_187 = 187,
    RemoteResetCodeOp_reserved_future_188 = 188,
    RemoteResetCodeOp_reserved_future_189 = 189,
    RemoteResetCodeOp_reserved_future_190 = 190,
    RemoteResetCodeOp_reserved_future_191 = 191,
    RemoteResetCodeOp_reserved_future_192 = 192,
    RemoteResetCodeOp_reserved_future_193 = 193,
    RemoteResetCodeOp_reserved_future_194 = 194,
    RemoteResetCodeOp_reserved_future_195 = 195,
    RemoteResetCodeOp_reserved_future_196 = 196,
    RemoteResetCodeOp_reserved_future_197 = 197,
    RemoteResetCodeOp_reserved_future_198 = 198,
    RemoteResetCodeOp_reserved_future_199 = 199,
    RemoteResetCodeOp_reserved_future_200 = 200,
    RemoteResetCodeOp_reserved_future_201 = 201,
    RemoteResetCodeOp_reserved_future_202 = 202,
    RemoteResetCodeOp_reserved_future_203 = 203,
    RemoteResetCodeOp_reserved_future_204 = 204,
    RemoteResetCodeOp_reserved_future_205 = 205,
    RemoteResetCodeOp_reserved_future_206 = 206,
    RemoteResetCodeOp_reserved_future_207 = 207,
    RemoteResetCodeOp_reserved_future_208 = 208,
    RemoteResetCodeOp_reserved_future_209 = 209,
    RemoteResetCodeOp_reserved_future_210 = 210,
    RemoteResetCodeOp_reserved_future_211 = 211,
    RemoteResetCodeOp_reserved_future_212 = 212,
    RemoteResetCodeOp_reserved_future_213 = 213,
    RemoteResetCodeOp_reserved_future_214 = 214,
    RemoteResetCodeOp_reserved_future_215 = 215,
    RemoteResetCodeOp_reserved_future_216 = 216,
    RemoteResetCodeOp_reserved_future_217 = 217,
    RemoteResetCodeOp_reserved_future_218 = 218,
    RemoteResetCodeOp_reserved_future_219 = 219,
    RemoteResetCodeOp_reserved_future_220 = 220,
    RemoteResetCodeOp_reserved_future_221 = 221,
    RemoteResetCodeOp_reserved_future_222 = 222,
    RemoteResetCodeOp_reserved_future_223 = 223,
    RemoteResetCodeOp_reserved_future_224 = 224,
    RemoteResetCodeOp_reserved_future_225 = 225,
    RemoteResetCodeOp_reserved_future_226 = 226,
    RemoteResetCodeOp_reserved_future_227 = 227,
    RemoteResetCodeOp_reserved_future_228 = 228,
    RemoteResetCodeOp_reserved_future_229 = 229,
    RemoteResetCodeOp_reserved_future_230 = 230,
    RemoteResetCodeOp_reserved_future_231 = 231,
    RemoteResetCodeOp_reserved_future_232 = 232,
    RemoteResetCodeOp_reserved_future_233 = 233,
    RemoteResetCodeOp_reserved_future_234 = 234,
    RemoteResetCodeOp_reserved_future_235 = 235,
    RemoteResetCodeOp_reserved_future_236 = 236,
    RemoteResetCodeOp_reserved_future_237 = 237,
    RemoteResetCodeOp_reserved_future_238 = 238,
    RemoteResetCodeOp_reserved_future_239 = 239,
    RemoteResetCodeOp_reserved_future_240 = 240,
    RemoteResetCodeOp_reserved_future_241 = 241,
    RemoteResetCodeOp_reserved_future_242 = 242,
    RemoteResetCodeOp_reserved_future_243 = 243,
    RemoteResetCodeOp_reserved_future_244 = 244,
    RemoteResetCodeOp_reserved_future_245 = 245,
    RemoteResetCodeOp_reserved_future_246 = 246,
    RemoteResetCodeOp_reserved_future_247 = 247,
    RemoteResetCodeOp_reserved_future_248 = 248,
    RemoteResetCodeOp_reserved_future_249 = 249,
    RemoteResetCodeOp_reserved_future_250 = 250,
    RemoteResetCodeOp_reserved_future_251 = 251,
    RemoteResetCodeOp_reserved_future_252 = 252,
    RemoteResetCodeOp_reserved_future_253 = 253,
    RemoteResetCodeOp_reserved_future_254 = 254,
    RemoteResetCodeOp_reserved_future_255 = 255,
} RemoteResetCodeOp;

typedef struct {
    uint16_t reset_id;
    RemoteResetCodeOp reset_code_op;
    struct { uint8_t *data; size_t len; } vendor_specific_payload;
} RemoteReset;

typedef enum {
    RaiseCease_raise = 0,
    RaiseCease_cease = 1,
    RaiseCease_reserved_2 = 2,
    RaiseCease_reserved_3 = 3,
    RaiseCease_reserved_4 = 4,
    RaiseCease_reserved_5 = 5,
    RaiseCease_reserved_6 = 6,
    RaiseCease_reserved_7 = 7,
    RaiseCease_reserved_8 = 8,
    RaiseCease_reserved_9 = 9,
    RaiseCease_reserved_10 = 10,
    RaiseCease_reserved_11 = 11,
    RaiseCease_reserved_12 = 12,
    RaiseCease_reserved_13 = 13,
    RaiseCease_reserved_14 = 14,
    RaiseCease_reserved_15 = 15,
} RaiseCease;

typedef uint16_t RTC_ID;

typedef struct {
    RTC_ID rtc_id;
    SEQ_ID seq_id;
    struct { uint8_t *data; size_t len; } rt_control_data;
} RealTimeControlData;

typedef enum {
    RMAReqResp_request = 0,
    RMAReqResp_response = 1,
    RMAReqResp_failure = 2,
    RMAReqResp_reserved_3 = 3,
    RMAReqResp_reserved_4 = 4,
    RMAReqResp_reserved_5 = 5,
    RMAReqResp_reserved_6 = 6,
    RMAReqResp_reserved_7 = 7,
    RMAReqResp_reserved_8 = 8,
    RMAReqResp_reserved_9 = 9,
    RMAReqResp_reserved_10 = 10,
    RMAReqResp_reserved_11 = 11,
    RMAReqResp_reserved_12 = 12,
    RMAReqResp_reserved_13 = 13,
    RMAReqResp_reserved_14 = 14,
    RMAReqResp_reserved_15 = 15,
} RMAReqResp;

typedef enum {
    RMAReadWrite_read = 0,
    RMAReadWrite_write = 1,
    RMAReadWrite_write_no_resp = 2,
    RMAReadWrite_reserved_3 = 3,
    RMAReadWrite_reserved_4 = 4,
    RMAReadWrite_reserved_5 = 5,
    RMAReadWrite_reserved_6 = 6,
    RMAReadWrite_reserved_7 = 7,
    RMAReadWrite_reserved_8 = 8,
    RMAReadWrite_reserved_9 = 9,
    RMAReadWrite_reserved_10 = 10,
    RMAReadWrite_reserved_11 = 11,
    RMAReadWrite_reserved_12 = 12,
    RMAReadWrite_reserved_13 = 13,
    RMAReadWrite_reserved_14 = 14,
    RMAReadWrite_reserved_15 = 15,
} RMAReadWrite;

typedef struct {
    uint8_t remote_memory_access_id;
    RMAReadWrite read_write;
    RMAReqResp req_resp;
    uint16_t element_id;
    uint64_t address;
    uint16_t length;
    struct { uint8_t *data; size_t len; } data;
} RemoteMemoryAccess;

typedef struct {
    uint64_t seconds;
    uint32_t nanoseconds;
} PtpTimestamp;

typedef enum {
    ProtocolRevision_reserved = 0,
    ProtocolRevision_v1_through_v2 = 1,
    ProtocolRevision_reserved_future_2 = 2,
    ProtocolRevision_reserved_future_3 = 3,
    ProtocolRevision_reserved_future_4 = 4,
    ProtocolRevision_reserved_future_5 = 5,
    ProtocolRevision_reserved_future_6 = 6,
    ProtocolRevision_reserved_future_7 = 7,
    ProtocolRevision_reserved_future_8 = 8,
    ProtocolRevision_reserved_future_9 = 9,
    ProtocolRevision_reserved_future_10 = 10,
    ProtocolRevision_reserved_future_11 = 11,
    ProtocolRevision_reserved_future_12 = 12,
    ProtocolRevision_reserved_future_13 = 13,
    ProtocolRevision_reserved_future_14 = 14,
    ProtocolRevision_reserved_future_15 = 15,
} ProtocolRevision;

typedef uint16_t PC_ID;

typedef struct {
    struct { uint8_t *data; size_t len; } data;
} OpaquePayload;

typedef enum {
    OneWayDelayActionType_request = 0,
    OneWayDelayActionType_request_with_follow_up = 1,
    OneWayDelayActionType_response = 2,
    OneWayDelayActionType_remote_request = 3,
    OneWayDelayActionType_remote_request_with_follow_up = 4,
    OneWayDelayActionType_follow_up = 5,
    OneWayDelayActionType_reserved_6 = 6,
    OneWayDelayActionType_reserved_7 = 7,
    OneWayDelayActionType_reserved_8 = 8,
    OneWayDelayActionType_reserved_9 = 9,
    OneWayDelayActionType_reserved_10 = 10,
    OneWayDelayActionType_reserved_11 = 11,
    OneWayDelayActionType_reserved_12 = 12,
    OneWayDelayActionType_reserved_13 = 13,
    OneWayDelayActionType_reserved_14 = 14,
    OneWayDelayActionType_reserved_15 = 15,
    OneWayDelayActionType_reserved_16 = 16,
    OneWayDelayActionType_reserved_17 = 17,
    OneWayDelayActionType_reserved_18 = 18,
    OneWayDelayActionType_reserved_19 = 19,
    OneWayDelayActionType_reserved_20 = 20,
    OneWayDelayActionType_reserved_21 = 21,
    OneWayDelayActionType_reserved_22 = 22,
    OneWayDelayActionType_reserved_23 = 23,
    OneWayDelayActionType_reserved_24 = 24,
    OneWayDelayActionType_reserved_25 = 25,
    OneWayDelayActionType_reserved_26 = 26,
    OneWayDelayActionType_reserved_27 = 27,
    OneWayDelayActionType_reserved_28 = 28,
    OneWayDelayActionType_reserved_29 = 29,
    OneWayDelayActionType_reserved_30 = 30,
    OneWayDelayActionType_reserved_31 = 31,
    OneWayDelayActionType_reserved_32 = 32,
    OneWayDelayActionType_reserved_33 = 33,
    OneWayDelayActionType_reserved_34 = 34,
    OneWayDelayActionType_reserved_35 = 35,
    OneWayDelayActionType_reserved_36 = 36,
    OneWayDelayActionType_reserved_37 = 37,
    OneWayDelayActionType_reserved_38 = 38,
    OneWayDelayActionType_reserved_39 = 39,
    OneWayDelayActionType_reserved_40 = 40,
    OneWayDelayActionType_reserved_41 = 41,
    OneWayDelayActionType_reserved_42 = 42,
    OneWayDelayActionType_reserved_43 = 43,
    OneWayDelayActionType_reserved_44 = 44,
    OneWayDelayActionType_reserved_45 = 45,
    OneWayDelayActionType_reserved_46 = 46,
    OneWayDelayActionType_reserved_47 = 47,
    OneWayDelayActionType_reserved_48 = 48,
    OneWayDelayActionType_reserved_49 = 49,
    OneWayDelayActionType_reserved_50 = 50,
    OneWayDelayActionType_reserved_51 = 51,
    OneWayDelayActionType_reserved_52 = 52,
    OneWayDelayActionType_reserved_53 = 53,
    OneWayDelayActionType_reserved_54 = 54,
    OneWayDelayActionType_reserved_55 = 55,
    OneWayDelayActionType_reserved_56 = 56,
    OneWayDelayActionType_reserved_57 = 57,
    OneWayDelayActionType_reserved_58 = 58,
    OneWayDelayActionType_reserved_59 = 59,
    OneWayDelayActionType_reserved_60 = 60,
    OneWayDelayActionType_reserved_61 = 61,
    OneWayDelayActionType_reserved_62 = 62,
    OneWayDelayActionType_reserved_63 = 63,
    OneWayDelayActionType_reserved_64 = 64,
    OneWayDelayActionType_reserved_65 = 65,
    OneWayDelayActionType_reserved_66 = 66,
    OneWayDelayActionType_reserved_67 = 67,
    OneWayDelayActionType_reserved_68 = 68,
    OneWayDelayActionType_reserved_69 = 69,
    OneWayDelayActionType_reserved_70 = 70,
    OneWayDelayActionType_reserved_71 = 71,
    OneWayDelayActionType_reserved_72 = 72,
    OneWayDelayActionType_reserved_73 = 73,
    OneWayDelayActionType_reserved_74 = 74,
    OneWayDelayActionType_reserved_75 = 75,
    OneWayDelayActionType_reserved_76 = 76,
    OneWayDelayActionType_reserved_77 = 77,
    OneWayDelayActionType_reserved_78 = 78,
    OneWayDelayActionType_reserved_79 = 79,
    OneWayDelayActionType_reserved_80 = 80,
    OneWayDelayActionType_reserved_81 = 81,
    OneWayDelayActionType_reserved_82 = 82,
    OneWayDelayActionType_reserved_83 = 83,
    OneWayDelayActionType_reserved_84 = 84,
    OneWayDelayActionType_reserved_85 = 85,
    OneWayDelayActionType_reserved_86 = 86,
    OneWayDelayActionType_reserved_87 = 87,
    OneWayDelayActionType_reserved_88 = 88,
    OneWayDelayActionType_reserved_89 = 89,
    OneWayDelayActionType_reserved_90 = 90,
    OneWayDelayActionType_reserved_91 = 91,
    OneWayDelayActionType_reserved_92 = 92,
    OneWayDelayActionType_reserved_93 = 93,
    OneWayDelayActionType_reserved_94 = 94,
    OneWayDelayActionType_reserved_95 = 95,
    OneWayDelayActionType_reserved_96 = 96,
    OneWayDelayActionType_reserved_97 = 97,
    OneWayDelayActionType_reserved_98 = 98,
    OneWayDelayActionType_reserved_99 = 99,
    OneWayDelayActionType_reserved_100 = 100,
    OneWayDelayActionType_reserved_101 = 101,
    OneWayDelayActionType_reserved_102 = 102,
    OneWayDelayActionType_reserved_103 = 103,
    OneWayDelayActionType_reserved_104 = 104,
    OneWayDelayActionType_reserved_105 = 105,
    OneWayDelayActionType_reserved_106 = 106,
    OneWayDelayActionType_reserved_107 = 107,
    OneWayDelayActionType_reserved_108 = 108,
    OneWayDelayActionType_reserved_109 = 109,
    OneWayDelayActionType_reserved_110 = 110,
    OneWayDelayActionType_reserved_111 = 111,
    OneWayDelayActionType_reserved_112 = 112,
    OneWayDelayActionType_reserved_113 = 113,
    OneWayDelayActionType_reserved_114 = 114,
    OneWayDelayActionType_reserved_115 = 115,
    OneWayDelayActionType_reserved_116 = 116,
    OneWayDelayActionType_reserved_117 = 117,
    OneWayDelayActionType_reserved_118 = 118,
    OneWayDelayActionType_reserved_119 = 119,
    OneWayDelayActionType_reserved_120 = 120,
    OneWayDelayActionType_reserved_121 = 121,
    OneWayDelayActionType_reserved_122 = 122,
    OneWayDelayActionType_reserved_123 = 123,
    OneWayDelayActionType_reserved_124 = 124,
    OneWayDelayActionType_reserved_125 = 125,
    OneWayDelayActionType_reserved_126 = 126,
    OneWayDelayActionType_reserved_127 = 127,
    OneWayDelayActionType_reserved_128 = 128,
    OneWayDelayActionType_reserved_129 = 129,
    OneWayDelayActionType_reserved_130 = 130,
    OneWayDelayActionType_reserved_131 = 131,
    OneWayDelayActionType_reserved_132 = 132,
    OneWayDelayActionType_reserved_133 = 133,
    OneWayDelayActionType_reserved_134 = 134,
    OneWayDelayActionType_reserved_135 = 135,
    OneWayDelayActionType_reserved_136 = 136,
    OneWayDelayActionType_reserved_137 = 137,
    OneWayDelayActionType_reserved_138 = 138,
    OneWayDelayActionType_reserved_139 = 139,
    OneWayDelayActionType_reserved_140 = 140,
    OneWayDelayActionType_reserved_141 = 141,
    OneWayDelayActionType_reserved_142 = 142,
    OneWayDelayActionType_reserved_143 = 143,
    OneWayDelayActionType_reserved_144 = 144,
    OneWayDelayActionType_reserved_145 = 145,
    OneWayDelayActionType_reserved_146 = 146,
    OneWayDelayActionType_reserved_147 = 147,
    OneWayDelayActionType_reserved_148 = 148,
    OneWayDelayActionType_reserved_149 = 149,
    OneWayDelayActionType_reserved_150 = 150,
    OneWayDelayActionType_reserved_151 = 151,
    OneWayDelayActionType_reserved_152 = 152,
    OneWayDelayActionType_reserved_153 = 153,
    OneWayDelayActionType_reserved_154 = 154,
    OneWayDelayActionType_reserved_155 = 155,
    OneWayDelayActionType_reserved_156 = 156,
    OneWayDelayActionType_reserved_157 = 157,
    OneWayDelayActionType_reserved_158 = 158,
    OneWayDelayActionType_reserved_159 = 159,
    OneWayDelayActionType_reserved_160 = 160,
    OneWayDelayActionType_reserved_161 = 161,
    OneWayDelayActionType_reserved_162 = 162,
    OneWayDelayActionType_reserved_163 = 163,
    OneWayDelayActionType_reserved_164 = 164,
    OneWayDelayActionType_reserved_165 = 165,
    OneWayDelayActionType_reserved_166 = 166,
    OneWayDelayActionType_reserved_167 = 167,
    OneWayDelayActionType_reserved_168 = 168,
    OneWayDelayActionType_reserved_169 = 169,
    OneWayDelayActionType_reserved_170 = 170,
    OneWayDelayActionType_reserved_171 = 171,
    OneWayDelayActionType_reserved_172 = 172,
    OneWayDelayActionType_reserved_173 = 173,
    OneWayDelayActionType_reserved_174 = 174,
    OneWayDelayActionType_reserved_175 = 175,
    OneWayDelayActionType_reserved_176 = 176,
    OneWayDelayActionType_reserved_177 = 177,
    OneWayDelayActionType_reserved_178 = 178,
    OneWayDelayActionType_reserved_179 = 179,
    OneWayDelayActionType_reserved_180 = 180,
    OneWayDelayActionType_reserved_181 = 181,
    OneWayDelayActionType_reserved_182 = 182,
    OneWayDelayActionType_reserved_183 = 183,
    OneWayDelayActionType_reserved_184 = 184,
    OneWayDelayActionType_reserved_185 = 185,
    OneWayDelayActionType_reserved_186 = 186,
    OneWayDelayActionType_reserved_187 = 187,
    OneWayDelayActionType_reserved_188 = 188,
    OneWayDelayActionType_reserved_189 = 189,
    OneWayDelayActionType_reserved_190 = 190,
    OneWayDelayActionType_reserved_191 = 191,
    OneWayDelayActionType_reserved_192 = 192,
    OneWayDelayActionType_reserved_193 = 193,
    OneWayDelayActionType_reserved_194 = 194,
    OneWayDelayActionType_reserved_195 = 195,
    OneWayDelayActionType_reserved_196 = 196,
    OneWayDelayActionType_reserved_197 = 197,
    OneWayDelayActionType_reserved_198 = 198,
    OneWayDelayActionType_reserved_199 = 199,
    OneWayDelayActionType_reserved_200 = 200,
    OneWayDelayActionType_reserved_201 = 201,
    OneWayDelayActionType_reserved_202 = 202,
    OneWayDelayActionType_reserved_203 = 203,
    OneWayDelayActionType_reserved_204 = 204,
    OneWayDelayActionType_reserved_205 = 205,
    OneWayDelayActionType_reserved_206 = 206,
    OneWayDelayActionType_reserved_207 = 207,
    OneWayDelayActionType_reserved_208 = 208,
    OneWayDelayActionType_reserved_209 = 209,
    OneWayDelayActionType_reserved_210 = 210,
    OneWayDelayActionType_reserved_211 = 211,
    OneWayDelayActionType_reserved_212 = 212,
    OneWayDelayActionType_reserved_213 = 213,
    OneWayDelayActionType_reserved_214 = 214,
    OneWayDelayActionType_reserved_215 = 215,
    OneWayDelayActionType_reserved_216 = 216,
    OneWayDelayActionType_reserved_217 = 217,
    OneWayDelayActionType_reserved_218 = 218,
    OneWayDelayActionType_reserved_219 = 219,
    OneWayDelayActionType_reserved_220 = 220,
    OneWayDelayActionType_reserved_221 = 221,
    OneWayDelayActionType_reserved_222 = 222,
    OneWayDelayActionType_reserved_223 = 223,
    OneWayDelayActionType_reserved_224 = 224,
    OneWayDelayActionType_reserved_225 = 225,
    OneWayDelayActionType_reserved_226 = 226,
    OneWayDelayActionType_reserved_227 = 227,
    OneWayDelayActionType_reserved_228 = 228,
    OneWayDelayActionType_reserved_229 = 229,
    OneWayDelayActionType_reserved_230 = 230,
    OneWayDelayActionType_reserved_231 = 231,
    OneWayDelayActionType_reserved_232 = 232,
    OneWayDelayActionType_reserved_233 = 233,
    OneWayDelayActionType_reserved_234 = 234,
    OneWayDelayActionType_reserved_235 = 235,
    OneWayDelayActionType_reserved_236 = 236,
    OneWayDelayActionType_reserved_237 = 237,
    OneWayDelayActionType_reserved_238 = 238,
    OneWayDelayActionType_reserved_239 = 239,
    OneWayDelayActionType_reserved_240 = 240,
    OneWayDelayActionType_reserved_241 = 241,
    OneWayDelayActionType_reserved_242 = 242,
    OneWayDelayActionType_reserved_243 = 243,
    OneWayDelayActionType_reserved_244 = 244,
    OneWayDelayActionType_reserved_245 = 245,
    OneWayDelayActionType_reserved_246 = 246,
    OneWayDelayActionType_reserved_247 = 247,
    OneWayDelayActionType_reserved_248 = 248,
    OneWayDelayActionType_reserved_249 = 249,
    OneWayDelayActionType_reserved_250 = 250,
    OneWayDelayActionType_reserved_251 = 251,
    OneWayDelayActionType_reserved_252 = 252,
    OneWayDelayActionType_reserved_253 = 253,
    OneWayDelayActionType_reserved_254 = 254,
    OneWayDelayActionType_reserved_255 = 255,
} OneWayDelayActionType;

typedef struct {
    uint8_t measurement_id;
    OneWayDelayActionType action_type;
    PtpTimestamp timestamp;
    uint64_t compensation_value;
    struct { uint8_t *data; size_t len; } dummy_bytes;
} OneWayDelayMeasurement;

typedef enum {
    MessageType_iq_data = 0,
    MessageType_bit_sequence = 1,
    MessageType_real_time_control_data = 2,
    MessageType_generic_data_transfer = 3,
    MessageType_remote_memory_access = 4,
    MessageType_one_way_delay_measurement = 5,
    MessageType_remote_reset = 6,
    MessageType_event_indication = 7,
    MessageType_iwf_start_up = 8,
    MessageType_iwf_operation = 9,
    MessageType_iwf_mapping = 10,
    MessageType_iwf_delay_control = 11,
    MessageType_reserved_12 = 12,
    MessageType_reserved_13 = 13,
    MessageType_reserved_14 = 14,
    MessageType_reserved_15 = 15,
    MessageType_reserved_16 = 16,
    MessageType_reserved_17 = 17,
    MessageType_reserved_18 = 18,
    MessageType_reserved_19 = 19,
    MessageType_reserved_20 = 20,
    MessageType_reserved_21 = 21,
    MessageType_reserved_22 = 22,
    MessageType_reserved_23 = 23,
    MessageType_reserved_24 = 24,
    MessageType_reserved_25 = 25,
    MessageType_reserved_26 = 26,
    MessageType_reserved_27 = 27,
    MessageType_reserved_28 = 28,
    MessageType_reserved_29 = 29,
    MessageType_reserved_30 = 30,
    MessageType_reserved_31 = 31,
    MessageType_reserved_32 = 32,
    MessageType_reserved_33 = 33,
    MessageType_reserved_34 = 34,
    MessageType_reserved_35 = 35,
    MessageType_reserved_36 = 36,
    MessageType_reserved_37 = 37,
    MessageType_reserved_38 = 38,
    MessageType_reserved_39 = 39,
    MessageType_reserved_40 = 40,
    MessageType_reserved_41 = 41,
    MessageType_reserved_42 = 42,
    MessageType_reserved_43 = 43,
    MessageType_reserved_44 = 44,
    MessageType_reserved_45 = 45,
    MessageType_reserved_46 = 46,
    MessageType_reserved_47 = 47,
    MessageType_reserved_48 = 48,
    MessageType_reserved_49 = 49,
    MessageType_reserved_50 = 50,
    MessageType_reserved_51 = 51,
    MessageType_reserved_52 = 52,
    MessageType_reserved_53 = 53,
    MessageType_reserved_54 = 54,
    MessageType_reserved_55 = 55,
    MessageType_reserved_56 = 56,
    MessageType_reserved_57 = 57,
    MessageType_reserved_58 = 58,
    MessageType_reserved_59 = 59,
    MessageType_reserved_60 = 60,
    MessageType_reserved_61 = 61,
    MessageType_reserved_62 = 62,
    MessageType_reserved_63 = 63,
    MessageType_vendor_specific_64 = 64,
    MessageType_vendor_specific_65 = 65,
    MessageType_vendor_specific_66 = 66,
    MessageType_vendor_specific_67 = 67,
    MessageType_vendor_specific_68 = 68,
    MessageType_vendor_specific_69 = 69,
    MessageType_vendor_specific_70 = 70,
    MessageType_vendor_specific_71 = 71,
    MessageType_vendor_specific_72 = 72,
    MessageType_vendor_specific_73 = 73,
    MessageType_vendor_specific_74 = 74,
    MessageType_vendor_specific_75 = 75,
    MessageType_vendor_specific_76 = 76,
    MessageType_vendor_specific_77 = 77,
    MessageType_vendor_specific_78 = 78,
    MessageType_vendor_specific_79 = 79,
    MessageType_vendor_specific_80 = 80,
    MessageType_vendor_specific_81 = 81,
    MessageType_vendor_specific_82 = 82,
    MessageType_vendor_specific_83 = 83,
    MessageType_vendor_specific_84 = 84,
    MessageType_vendor_specific_85 = 85,
    MessageType_vendor_specific_86 = 86,
    MessageType_vendor_specific_87 = 87,
    MessageType_vendor_specific_88 = 88,
    MessageType_vendor_specific_89 = 89,
    MessageType_vendor_specific_90 = 90,
    MessageType_vendor_specific_91 = 91,
    MessageType_vendor_specific_92 = 92,
    MessageType_vendor_specific_93 = 93,
    MessageType_vendor_specific_94 = 94,
    MessageType_vendor_specific_95 = 95,
    MessageType_vendor_specific_96 = 96,
    MessageType_vendor_specific_97 = 97,
    MessageType_vendor_specific_98 = 98,
    MessageType_vendor_specific_99 = 99,
    MessageType_vendor_specific_100 = 100,
    MessageType_vendor_specific_101 = 101,
    MessageType_vendor_specific_102 = 102,
    MessageType_vendor_specific_103 = 103,
    MessageType_vendor_specific_104 = 104,
    MessageType_vendor_specific_105 = 105,
    MessageType_vendor_specific_106 = 106,
    MessageType_vendor_specific_107 = 107,
    MessageType_vendor_specific_108 = 108,
    MessageType_vendor_specific_109 = 109,
    MessageType_vendor_specific_110 = 110,
    MessageType_vendor_specific_111 = 111,
    MessageType_vendor_specific_112 = 112,
    MessageType_vendor_specific_113 = 113,
    MessageType_vendor_specific_114 = 114,
    MessageType_vendor_specific_115 = 115,
    MessageType_vendor_specific_116 = 116,
    MessageType_vendor_specific_117 = 117,
    MessageType_vendor_specific_118 = 118,
    MessageType_vendor_specific_119 = 119,
    MessageType_vendor_specific_120 = 120,
    MessageType_vendor_specific_121 = 121,
    MessageType_vendor_specific_122 = 122,
    MessageType_vendor_specific_123 = 123,
    MessageType_vendor_specific_124 = 124,
    MessageType_vendor_specific_125 = 125,
    MessageType_vendor_specific_126 = 126,
    MessageType_vendor_specific_127 = 127,
    MessageType_vendor_specific_128 = 128,
    MessageType_vendor_specific_129 = 129,
    MessageType_vendor_specific_130 = 130,
    MessageType_vendor_specific_131 = 131,
    MessageType_vendor_specific_132 = 132,
    MessageType_vendor_specific_133 = 133,
    MessageType_vendor_specific_134 = 134,
    MessageType_vendor_specific_135 = 135,
    MessageType_vendor_specific_136 = 136,
    MessageType_vendor_specific_137 = 137,
    MessageType_vendor_specific_138 = 138,
    MessageType_vendor_specific_139 = 139,
    MessageType_vendor_specific_140 = 140,
    MessageType_vendor_specific_141 = 141,
    MessageType_vendor_specific_142 = 142,
    MessageType_vendor_specific_143 = 143,
    MessageType_vendor_specific_144 = 144,
    MessageType_vendor_specific_145 = 145,
    MessageType_vendor_specific_146 = 146,
    MessageType_vendor_specific_147 = 147,
    MessageType_vendor_specific_148 = 148,
    MessageType_vendor_specific_149 = 149,
    MessageType_vendor_specific_150 = 150,
    MessageType_vendor_specific_151 = 151,
    MessageType_vendor_specific_152 = 152,
    MessageType_vendor_specific_153 = 153,
    MessageType_vendor_specific_154 = 154,
    MessageType_vendor_specific_155 = 155,
    MessageType_vendor_specific_156 = 156,
    MessageType_vendor_specific_157 = 157,
    MessageType_vendor_specific_158 = 158,
    MessageType_vendor_specific_159 = 159,
    MessageType_vendor_specific_160 = 160,
    MessageType_vendor_specific_161 = 161,
    MessageType_vendor_specific_162 = 162,
    MessageType_vendor_specific_163 = 163,
    MessageType_vendor_specific_164 = 164,
    MessageType_vendor_specific_165 = 165,
    MessageType_vendor_specific_166 = 166,
    MessageType_vendor_specific_167 = 167,
    MessageType_vendor_specific_168 = 168,
    MessageType_vendor_specific_169 = 169,
    MessageType_vendor_specific_170 = 170,
    MessageType_vendor_specific_171 = 171,
    MessageType_vendor_specific_172 = 172,
    MessageType_vendor_specific_173 = 173,
    MessageType_vendor_specific_174 = 174,
    MessageType_vendor_specific_175 = 175,
    MessageType_vendor_specific_176 = 176,
    MessageType_vendor_specific_177 = 177,
    MessageType_vendor_specific_178 = 178,
    MessageType_vendor_specific_179 = 179,
    MessageType_vendor_specific_180 = 180,
    MessageType_vendor_specific_181 = 181,
    MessageType_vendor_specific_182 = 182,
    MessageType_vendor_specific_183 = 183,
    MessageType_vendor_specific_184 = 184,
    MessageType_vendor_specific_185 = 185,
    MessageType_vendor_specific_186 = 186,
    MessageType_vendor_specific_187 = 187,
    MessageType_vendor_specific_188 = 188,
    MessageType_vendor_specific_189 = 189,
    MessageType_vendor_specific_190 = 190,
    MessageType_vendor_specific_191 = 191,
    MessageType_vendor_specific_192 = 192,
    MessageType_vendor_specific_193 = 193,
    MessageType_vendor_specific_194 = 194,
    MessageType_vendor_specific_195 = 195,
    MessageType_vendor_specific_196 = 196,
    MessageType_vendor_specific_197 = 197,
    MessageType_vendor_specific_198 = 198,
    MessageType_vendor_specific_199 = 199,
    MessageType_vendor_specific_200 = 200,
    MessageType_vendor_specific_201 = 201,
    MessageType_vendor_specific_202 = 202,
    MessageType_vendor_specific_203 = 203,
    MessageType_vendor_specific_204 = 204,
    MessageType_vendor_specific_205 = 205,
    MessageType_vendor_specific_206 = 206,
    MessageType_vendor_specific_207 = 207,
    MessageType_vendor_specific_208 = 208,
    MessageType_vendor_specific_209 = 209,
    MessageType_vendor_specific_210 = 210,
    MessageType_vendor_specific_211 = 211,
    MessageType_vendor_specific_212 = 212,
    MessageType_vendor_specific_213 = 213,
    MessageType_vendor_specific_214 = 214,
    MessageType_vendor_specific_215 = 215,
    MessageType_vendor_specific_216 = 216,
    MessageType_vendor_specific_217 = 217,
    MessageType_vendor_specific_218 = 218,
    MessageType_vendor_specific_219 = 219,
    MessageType_vendor_specific_220 = 220,
    MessageType_vendor_specific_221 = 221,
    MessageType_vendor_specific_222 = 222,
    MessageType_vendor_specific_223 = 223,
    MessageType_vendor_specific_224 = 224,
    MessageType_vendor_specific_225 = 225,
    MessageType_vendor_specific_226 = 226,
    MessageType_vendor_specific_227 = 227,
    MessageType_vendor_specific_228 = 228,
    MessageType_vendor_specific_229 = 229,
    MessageType_vendor_specific_230 = 230,
    MessageType_vendor_specific_231 = 231,
    MessageType_vendor_specific_232 = 232,
    MessageType_vendor_specific_233 = 233,
    MessageType_vendor_specific_234 = 234,
    MessageType_vendor_specific_235 = 235,
    MessageType_vendor_specific_236 = 236,
    MessageType_vendor_specific_237 = 237,
    MessageType_vendor_specific_238 = 238,
    MessageType_vendor_specific_239 = 239,
    MessageType_vendor_specific_240 = 240,
    MessageType_vendor_specific_241 = 241,
    MessageType_vendor_specific_242 = 242,
    MessageType_vendor_specific_243 = 243,
    MessageType_vendor_specific_244 = 244,
    MessageType_vendor_specific_245 = 245,
    MessageType_vendor_specific_246 = 246,
    MessageType_vendor_specific_247 = 247,
    MessageType_vendor_specific_248 = 248,
    MessageType_vendor_specific_249 = 249,
    MessageType_vendor_specific_250 = 250,
    MessageType_vendor_specific_251 = 251,
    MessageType_vendor_specific_252 = 252,
    MessageType_vendor_specific_253 = 253,
    MessageType_vendor_specific_254 = 254,
    MessageType_vendor_specific_255 = 255,
} MessageType;

typedef uint32_t LONG_SEQ_ID;

typedef uint32_t LONG_PC_ID;

typedef struct {
    PC_ID pc_id;
    SEQ_ID seq_id;
    struct { uint8_t *data; size_t len; } iq_samples;
} IQData;

typedef struct {
    ProtocolRevision revision;
    uint8_t reserved;
    uint8_t concatenation;
    MessageType message_type;
} Header;

typedef struct {
    LONG_PC_ID pc_id;
    LONG_SEQ_ID seq_id;
    struct { uint8_t *data; size_t len; } data_transferred;
} GenericDataTransfer;

typedef struct {
    uint16_t element_id;
    RaiseCease raise_cease;
    uint16_t fault_notification_number;
    uint32_t additional_information;
} FaultNotificationRecord;

typedef enum {
    EventType_fault_indication = 0,
    EventType_fault_indication_ack = 1,
    EventType_notification_indication = 2,
    EventType_synchronization_request = 3,
    EventType_synchronization_ack = 4,
    EventType_synchronization_end = 5,
    EventType_reserved_6 = 6,
    EventType_reserved_7 = 7,
    EventType_reserved_8 = 8,
    EventType_reserved_9 = 9,
    EventType_reserved_10 = 10,
    EventType_reserved_11 = 11,
    EventType_reserved_12 = 12,
    EventType_reserved_13 = 13,
    EventType_reserved_14 = 14,
    EventType_reserved_15 = 15,
    EventType_reserved_16 = 16,
    EventType_reserved_17 = 17,
    EventType_reserved_18 = 18,
    EventType_reserved_19 = 19,
    EventType_reserved_20 = 20,
    EventType_reserved_21 = 21,
    EventType_reserved_22 = 22,
    EventType_reserved_23 = 23,
    EventType_reserved_24 = 24,
    EventType_reserved_25 = 25,
    EventType_reserved_26 = 26,
    EventType_reserved_27 = 27,
    EventType_reserved_28 = 28,
    EventType_reserved_29 = 29,
    EventType_reserved_30 = 30,
    EventType_reserved_31 = 31,
    EventType_reserved_32 = 32,
    EventType_reserved_33 = 33,
    EventType_reserved_34 = 34,
    EventType_reserved_35 = 35,
    EventType_reserved_36 = 36,
    EventType_reserved_37 = 37,
    EventType_reserved_38 = 38,
    EventType_reserved_39 = 39,
    EventType_reserved_40 = 40,
    EventType_reserved_41 = 41,
    EventType_reserved_42 = 42,
    EventType_reserved_43 = 43,
    EventType_reserved_44 = 44,
    EventType_reserved_45 = 45,
    EventType_reserved_46 = 46,
    EventType_reserved_47 = 47,
    EventType_reserved_48 = 48,
    EventType_reserved_49 = 49,
    EventType_reserved_50 = 50,
    EventType_reserved_51 = 51,
    EventType_reserved_52 = 52,
    EventType_reserved_53 = 53,
    EventType_reserved_54 = 54,
    EventType_reserved_55 = 55,
    EventType_reserved_56 = 56,
    EventType_reserved_57 = 57,
    EventType_reserved_58 = 58,
    EventType_reserved_59 = 59,
    EventType_reserved_60 = 60,
    EventType_reserved_61 = 61,
    EventType_reserved_62 = 62,
    EventType_reserved_63 = 63,
    EventType_reserved_64 = 64,
    EventType_reserved_65 = 65,
    EventType_reserved_66 = 66,
    EventType_reserved_67 = 67,
    EventType_reserved_68 = 68,
    EventType_reserved_69 = 69,
    EventType_reserved_70 = 70,
    EventType_reserved_71 = 71,
    EventType_reserved_72 = 72,
    EventType_reserved_73 = 73,
    EventType_reserved_74 = 74,
    EventType_reserved_75 = 75,
    EventType_reserved_76 = 76,
    EventType_reserved_77 = 77,
    EventType_reserved_78 = 78,
    EventType_reserved_79 = 79,
    EventType_reserved_80 = 80,
    EventType_reserved_81 = 81,
    EventType_reserved_82 = 82,
    EventType_reserved_83 = 83,
    EventType_reserved_84 = 84,
    EventType_reserved_85 = 85,
    EventType_reserved_86 = 86,
    EventType_reserved_87 = 87,
    EventType_reserved_88 = 88,
    EventType_reserved_89 = 89,
    EventType_reserved_90 = 90,
    EventType_reserved_91 = 91,
    EventType_reserved_92 = 92,
    EventType_reserved_93 = 93,
    EventType_reserved_94 = 94,
    EventType_reserved_95 = 95,
    EventType_reserved_96 = 96,
    EventType_reserved_97 = 97,
    EventType_reserved_98 = 98,
    EventType_reserved_99 = 99,
    EventType_reserved_100 = 100,
    EventType_reserved_101 = 101,
    EventType_reserved_102 = 102,
    EventType_reserved_103 = 103,
    EventType_reserved_104 = 104,
    EventType_reserved_105 = 105,
    EventType_reserved_106 = 106,
    EventType_reserved_107 = 107,
    EventType_reserved_108 = 108,
    EventType_reserved_109 = 109,
    EventType_reserved_110 = 110,
    EventType_reserved_111 = 111,
    EventType_reserved_112 = 112,
    EventType_reserved_113 = 113,
    EventType_reserved_114 = 114,
    EventType_reserved_115 = 115,
    EventType_reserved_116 = 116,
    EventType_reserved_117 = 117,
    EventType_reserved_118 = 118,
    EventType_reserved_119 = 119,
    EventType_reserved_120 = 120,
    EventType_reserved_121 = 121,
    EventType_reserved_122 = 122,
    EventType_reserved_123 = 123,
    EventType_reserved_124 = 124,
    EventType_reserved_125 = 125,
    EventType_reserved_126 = 126,
    EventType_reserved_127 = 127,
    EventType_reserved_128 = 128,
    EventType_reserved_129 = 129,
    EventType_reserved_130 = 130,
    EventType_reserved_131 = 131,
    EventType_reserved_132 = 132,
    EventType_reserved_133 = 133,
    EventType_reserved_134 = 134,
    EventType_reserved_135 = 135,
    EventType_reserved_136 = 136,
    EventType_reserved_137 = 137,
    EventType_reserved_138 = 138,
    EventType_reserved_139 = 139,
    EventType_reserved_140 = 140,
    EventType_reserved_141 = 141,
    EventType_reserved_142 = 142,
    EventType_reserved_143 = 143,
    EventType_reserved_144 = 144,
    EventType_reserved_145 = 145,
    EventType_reserved_146 = 146,
    EventType_reserved_147 = 147,
    EventType_reserved_148 = 148,
    EventType_reserved_149 = 149,
    EventType_reserved_150 = 150,
    EventType_reserved_151 = 151,
    EventType_reserved_152 = 152,
    EventType_reserved_153 = 153,
    EventType_reserved_154 = 154,
    EventType_reserved_155 = 155,
    EventType_reserved_156 = 156,
    EventType_reserved_157 = 157,
    EventType_reserved_158 = 158,
    EventType_reserved_159 = 159,
    EventType_reserved_160 = 160,
    EventType_reserved_161 = 161,
    EventType_reserved_162 = 162,
    EventType_reserved_163 = 163,
    EventType_reserved_164 = 164,
    EventType_reserved_165 = 165,
    EventType_reserved_166 = 166,
    EventType_reserved_167 = 167,
    EventType_reserved_168 = 168,
    EventType_reserved_169 = 169,
    EventType_reserved_170 = 170,
    EventType_reserved_171 = 171,
    EventType_reserved_172 = 172,
    EventType_reserved_173 = 173,
    EventType_reserved_174 = 174,
    EventType_reserved_175 = 175,
    EventType_reserved_176 = 176,
    EventType_reserved_177 = 177,
    EventType_reserved_178 = 178,
    EventType_reserved_179 = 179,
    EventType_reserved_180 = 180,
    EventType_reserved_181 = 181,
    EventType_reserved_182 = 182,
    EventType_reserved_183 = 183,
    EventType_reserved_184 = 184,
    EventType_reserved_185 = 185,
    EventType_reserved_186 = 186,
    EventType_reserved_187 = 187,
    EventType_reserved_188 = 188,
    EventType_reserved_189 = 189,
    EventType_reserved_190 = 190,
    EventType_reserved_191 = 191,
    EventType_reserved_192 = 192,
    EventType_reserved_193 = 193,
    EventType_reserved_194 = 194,
    EventType_reserved_195 = 195,
    EventType_reserved_196 = 196,
    EventType_reserved_197 = 197,
    EventType_reserved_198 = 198,
    EventType_reserved_199 = 199,
    EventType_reserved_200 = 200,
    EventType_reserved_201 = 201,
    EventType_reserved_202 = 202,
    EventType_reserved_203 = 203,
    EventType_reserved_204 = 204,
    EventType_reserved_205 = 205,
    EventType_reserved_206 = 206,
    EventType_reserved_207 = 207,
    EventType_reserved_208 = 208,
    EventType_reserved_209 = 209,
    EventType_reserved_210 = 210,
    EventType_reserved_211 = 211,
    EventType_reserved_212 = 212,
    EventType_reserved_213 = 213,
    EventType_reserved_214 = 214,
    EventType_reserved_215 = 215,
    EventType_reserved_216 = 216,
    EventType_reserved_217 = 217,
    EventType_reserved_218 = 218,
    EventType_reserved_219 = 219,
    EventType_reserved_220 = 220,
    EventType_reserved_221 = 221,
    EventType_reserved_222 = 222,
    EventType_reserved_223 = 223,
    EventType_reserved_224 = 224,
    EventType_reserved_225 = 225,
    EventType_reserved_226 = 226,
    EventType_reserved_227 = 227,
    EventType_reserved_228 = 228,
    EventType_reserved_229 = 229,
    EventType_reserved_230 = 230,
    EventType_reserved_231 = 231,
    EventType_reserved_232 = 232,
    EventType_reserved_233 = 233,
    EventType_reserved_234 = 234,
    EventType_reserved_235 = 235,
    EventType_reserved_236 = 236,
    EventType_reserved_237 = 237,
    EventType_reserved_238 = 238,
    EventType_reserved_239 = 239,
    EventType_reserved_240 = 240,
    EventType_reserved_241 = 241,
    EventType_reserved_242 = 242,
    EventType_reserved_243 = 243,
    EventType_reserved_244 = 244,
    EventType_reserved_245 = 245,
    EventType_reserved_246 = 246,
    EventType_reserved_247 = 247,
    EventType_reserved_248 = 248,
    EventType_reserved_249 = 249,
    EventType_reserved_250 = 250,
    EventType_reserved_251 = 251,
    EventType_reserved_252 = 252,
    EventType_reserved_253 = 253,
    EventType_reserved_254 = 254,
    EventType_reserved_255 = 255,
} EventType;

typedef struct {
    EventType event_type;
    uint8_t event_id;
    uint8_t sequence_number;
    uint8_t number_of_faults_or_notifications;
    struct { FaultNotificationRecord *data; size_t len; } records;
} EventIndication;

typedef struct {
    PC_ID pc_id;
    SEQ_ID seq_id;
    struct { uint8_t *data; size_t len; } bit_sequence;
} BitSequence;

typedef struct {
    MessageType tag;
    union {
        IQData iq_data;
        BitSequence bit_sequence;
        RealTimeControlData real_time_control_data;
        GenericDataTransfer generic_data_transfer;
        RemoteMemoryAccess remote_memory_access;
        OneWayDelayMeasurement one_way_delay_measurement;
        RemoteReset remote_reset;
        EventIndication event_indication;
        OpaquePayload iwf_start_up;
        OpaquePayload iwf_operation;
        OpaquePayload iwf_mapping;
        OpaquePayload iwf_delay_control;
        OpaquePayload reserved;
        OpaquePayload vendor_specific;
    } body;
} Payload;

typedef struct {
    Header header;
    Payload payload;
} Message;

typedef struct {
    Message message;
    struct { uint8_t *data; size_t len; } inter_message_padding;
} PduMessage;

typedef struct {
    struct { PduMessage *data; size_t len; } messages;
} Pdu;

int ecpri_header_encode(
    const Header *in,
    uint16_t payload_size,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    if (cap < 4) return -1;
    for (size_t i = 0; i < 4; ++i) buf[i] = 0;
    buf[0] |= (uint8_t)((in->revision & 0xFu) << 4);
    buf[0] |= (uint8_t)((0 & 0x7u) << 1);
    buf[0] |= (uint8_t)((in->concatenation & 0x1u) << 0);
    buf[1] = (uint8_t)(in->message_type & 0xFFu);
    buf[2] = (uint8_t)((payload_size >> 8) & 0xFFu);
    buf[3] = (uint8_t)((payload_size >> 0) & 0xFFu);
    *out_len = 4;
    return 0;
}

int ecpri_header_decode(
    const uint8_t *buf,
    size_t len,
    uint16_t *payload_size,
    Header *out
) {
    if (len < 4) return -1;
    out->revision = (ProtocolRevision)((buf[0] >> 4) & 0xFu);
    out->reserved = (uint8_t)((buf[0] >> 1) & 0x7u);
    out->concatenation = (uint8_t)((buf[0] >> 0) & 0x1u);
    out->message_type = (MessageType)(buf[1] & 0xFFu);
    *payload_size = (uint16_t)(((uint16_t)buf[2] << 8) | buf[3]);
    return 0;
}

int ecpri_ptp_timestamp_encode(
    const PtpTimestamp *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    if (cap < 10) return -1;
    for (size_t i = 0; i < 10; ++i) buf[i] = 0;
    buf[0] = (uint8_t)((in->seconds >> 40) & 0xFFu);
    buf[1] = (uint8_t)((in->seconds >> 32) & 0xFFu);
    buf[2] = (uint8_t)((in->seconds >> 24) & 0xFFu);
    buf[3] = (uint8_t)((in->seconds >> 16) & 0xFFu);
    buf[4] = (uint8_t)((in->seconds >> 8) & 0xFFu);
    buf[5] = (uint8_t)((in->seconds >> 0) & 0xFFu);
    buf[6] = (uint8_t)((in->nanoseconds >> 24) & 0xFFu);
    buf[7] = (uint8_t)((in->nanoseconds >> 16) & 0xFFu);
    buf[8] = (uint8_t)((in->nanoseconds >> 8) & 0xFFu);
    buf[9] = (uint8_t)((in->nanoseconds >> 0) & 0xFFu);
    *out_len = 10;
    return 0;
}

int ecpri_ptp_timestamp_decode(
    const uint8_t *buf,
    size_t len,
    PtpTimestamp *out
) {
    if (len < 10) return -1;
    uint64_t __mek_seconds = 0;
    __mek_seconds = (uint64_t)((__mek_seconds << 8) | buf[0]);
    __mek_seconds = (uint64_t)((__mek_seconds << 8) | buf[1]);
    __mek_seconds = (uint64_t)((__mek_seconds << 8) | buf[2]);
    __mek_seconds = (uint64_t)((__mek_seconds << 8) | buf[3]);
    __mek_seconds = (uint64_t)((__mek_seconds << 8) | buf[4]);
    __mek_seconds = (uint64_t)((__mek_seconds << 8) | buf[5]);
    out->seconds = __mek_seconds;
    uint32_t __mek_nanoseconds = 0;
    __mek_nanoseconds = (uint32_t)((__mek_nanoseconds << 8) | buf[6]);
    __mek_nanoseconds = (uint32_t)((__mek_nanoseconds << 8) | buf[7]);
    __mek_nanoseconds = (uint32_t)((__mek_nanoseconds << 8) | buf[8]);
    __mek_nanoseconds = (uint32_t)((__mek_nanoseconds << 8) | buf[9]);
    out->nanoseconds = __mek_nanoseconds;
    return 0;
}

int ecpri_fault_notification_record_encode(
    const FaultNotificationRecord *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    if (cap < 8) return -1;
    for (size_t i = 0; i < 8; ++i) buf[i] = 0;
    buf[0] = (uint8_t)((in->element_id >> 8) & 0xFFu);
    buf[1] = (uint8_t)((in->element_id >> 0) & 0xFFu);
    buf[2] |= (uint8_t)((in->raise_cease & 0xFu) << 4);
    {
        uint64_t __bits = (uint64_t)(in->fault_notification_number) & ((1ull << 12) - 1);
        for (uint32_t __i = 0; __i < 12; ++__i) {
            uint32_t __bit = 20u + __i;
            uint32_t __byte = __bit / 8u;
            uint32_t __pos = 7u - (__bit % 8u);
            if ((__bits >> (12u - 1u - __i)) & 1ull)
                buf[__byte] |= (uint8_t)(1u << __pos);
        }
    }
    buf[4] = (uint8_t)((in->additional_information >> 24) & 0xFFu);
    buf[5] = (uint8_t)((in->additional_information >> 16) & 0xFFu);
    buf[6] = (uint8_t)((in->additional_information >> 8) & 0xFFu);
    buf[7] = (uint8_t)((in->additional_information >> 0) & 0xFFu);
    *out_len = 8;
    return 0;
}

int ecpri_fault_notification_record_decode(
    const uint8_t *buf,
    size_t len,
    FaultNotificationRecord *out
) {
    if (len < 8) return -1;
    out->element_id = (uint16_t)(((uint16_t)buf[0] << 8) | buf[1]);
    out->raise_cease = (RaiseCease)((buf[2] >> 4) & 0xFu);
    {
        uint64_t __mek_fault_notification_number = 0;
        for (uint32_t __i = 0; __i < 12; ++__i) {
            uint32_t __bit = 20u + __i;
            uint32_t __byte = __bit / 8u;
            uint32_t __pos = 7u - (__bit % 8u);
            if (buf[__byte] & (1u << __pos))
                __mek_fault_notification_number |= 1ull << (12u - 1u - __i);
        }
        out->fault_notification_number = (uint16_t)(__mek_fault_notification_number & 0xFFFull);
    }
    uint32_t __mek_additional_information = 0;
    __mek_additional_information = (uint32_t)((__mek_additional_information << 8) | buf[4]);
    __mek_additional_information = (uint32_t)((__mek_additional_information << 8) | buf[5]);
    __mek_additional_information = (uint32_t)((__mek_additional_information << 8) | buf[6]);
    __mek_additional_information = (uint32_t)((__mek_additional_information << 8) | buf[7]);
    out->additional_information = __mek_additional_information;
    return 0;
}

int ecpri_opaque_payload_encode(
    const OpaquePayload *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    if (cap < in->data.len) return -1;
    if (in->data.len > 0) {
        if (in->data.data == NULL) return -1;
        memcpy(buf, in->data.data, in->data.len);
    }
    *out_len = in->data.len;
    return 0;
}

int ecpri_opaque_payload_decode(
    const uint8_t *buf,
    size_t len,
    uint16_t payload_size,
    OpaquePayload *out
) {
    if (len < (size_t)payload_size) return -1;
    out->data.len = payload_size;
    if (payload_size > 0) {
        if (out->data.data == NULL) return -1;
        memcpy(out->data.data, buf, payload_size);
    }
    return 0;
}

int ecpri_iq_data_encode(
    const IQData *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    const uint16_t payload_size = (uint16_t)(4u + (uint16_t)in->iq_samples.len);
    if (cap < (size_t)payload_size) return -1;
    buf[0] = (uint8_t)((in->pc_id >> 8) & 0xFFu);
    buf[1] = (uint8_t)(in->pc_id & 0xFFu);
    buf[2] = (uint8_t)((in->seq_id >> 8) & 0xFFu);
    buf[3] = (uint8_t)(in->seq_id & 0xFFu);
    if (in->iq_samples.len > 0) {
        if (in->iq_samples.data == NULL) return -1;
        memcpy(buf + 4, in->iq_samples.data, in->iq_samples.len);
    }
    *out_len = payload_size;
    return 0;
}

int ecpri_iq_data_decode(
    const uint8_t *buf,
    size_t len,
    uint16_t payload_size,
    IQData *out
) {
    if (payload_size < 4 || len < (size_t)payload_size) return -1;
    out->pc_id = (uint16_t)(((uint16_t)buf[0] << 8) | buf[1]);
    out->seq_id = (uint16_t)(((uint16_t)buf[2] << 8) | buf[3]);
    const size_t sample_len = payload_size - 4;
    out->iq_samples.len = sample_len;
    if (sample_len > 0) {
        if (out->iq_samples.data == NULL) return -1;
        memcpy(out->iq_samples.data, buf + 4, sample_len);
    }
    return 0;
}

int ecpri_bit_sequence_encode(
    const BitSequence *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    const uint16_t payload_size = (uint16_t)(4u + (uint16_t)in->bit_sequence.len);
    if (cap < (size_t)payload_size) return -1;
    buf[0] = (uint8_t)((in->pc_id >> 8) & 0xFFu);
    buf[1] = (uint8_t)(in->pc_id & 0xFFu);
    buf[2] = (uint8_t)((in->seq_id >> 8) & 0xFFu);
    buf[3] = (uint8_t)(in->seq_id & 0xFFu);
    if (in->bit_sequence.len > 0) {
        if (in->bit_sequence.data == NULL) return -1;
        memcpy(buf + 4, in->bit_sequence.data, in->bit_sequence.len);
    }
    *out_len = payload_size;
    return 0;
}

int ecpri_bit_sequence_decode(
    const uint8_t *buf,
    size_t len,
    uint16_t payload_size,
    BitSequence *out
) {
    if (payload_size < 4 || len < (size_t)payload_size) return -1;
    out->pc_id = (uint16_t)(((uint16_t)buf[0] << 8) | buf[1]);
    out->seq_id = (uint16_t)(((uint16_t)buf[2] << 8) | buf[3]);
    const size_t sample_len = payload_size - 4;
    out->bit_sequence.len = sample_len;
    if (sample_len > 0) {
        if (out->bit_sequence.data == NULL) return -1;
        memcpy(out->bit_sequence.data, buf + 4, sample_len);
    }
    return 0;
}

int ecpri_real_time_control_data_encode(
    const RealTimeControlData *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    const uint16_t payload_size = (uint16_t)(4u + (uint16_t)in->rt_control_data.len);
    if (cap < (size_t)payload_size) return -1;
    buf[0] = (uint8_t)((in->rtc_id >> 8) & 0xFFu);
    buf[1] = (uint8_t)(in->rtc_id & 0xFFu);
    buf[2] = (uint8_t)((in->seq_id >> 8) & 0xFFu);
    buf[3] = (uint8_t)(in->seq_id & 0xFFu);
    if (in->rt_control_data.len > 0) {
        if (in->rt_control_data.data == NULL) return -1;
        memcpy(buf + 4, in->rt_control_data.data, in->rt_control_data.len);
    }
    *out_len = payload_size;
    return 0;
}

int ecpri_real_time_control_data_decode(
    const uint8_t *buf,
    size_t len,
    uint16_t payload_size,
    RealTimeControlData *out
) {
    if (payload_size < 4 || len < (size_t)payload_size) return -1;
    out->rtc_id = (uint16_t)(((uint16_t)buf[0] << 8) | buf[1]);
    out->seq_id = (uint16_t)(((uint16_t)buf[2] << 8) | buf[3]);
    const size_t sample_len = payload_size - 4;
    out->rt_control_data.len = sample_len;
    if (sample_len > 0) {
        if (out->rt_control_data.data == NULL) return -1;
        memcpy(out->rt_control_data.data, buf + 4, sample_len);
    }
    return 0;
}

int ecpri_generic_data_transfer_encode(
    const GenericDataTransfer *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    const uint16_t payload_size = (uint16_t)(8u + (uint16_t)in->data_transferred.len);
    if (cap < (size_t)payload_size) return -1;
    buf[0] = (uint8_t)((in->pc_id >> 24) & 0xFFu);
    buf[1] = (uint8_t)((in->pc_id >> 16) & 0xFFu);
    buf[2] = (uint8_t)((in->pc_id >> 8) & 0xFFu);
    buf[3] = (uint8_t)(in->pc_id & 0xFFu);
    buf[4] = (uint8_t)((in->seq_id >> 24) & 0xFFu);
    buf[5] = (uint8_t)((in->seq_id >> 16) & 0xFFu);
    buf[6] = (uint8_t)((in->seq_id >> 8) & 0xFFu);
    buf[7] = (uint8_t)(in->seq_id & 0xFFu);
    if (in->data_transferred.len > 0) {
        if (in->data_transferred.data == NULL) return -1;
        memcpy(buf + 8, in->data_transferred.data, in->data_transferred.len);
    }
    *out_len = payload_size;
    return 0;
}

int ecpri_generic_data_transfer_decode(
    const uint8_t *buf,
    size_t len,
    uint16_t payload_size,
    GenericDataTransfer *out
) {
    if (payload_size < 8 || len < (size_t)payload_size) return -1;
    out->pc_id = (uint32_t)(((uint32_t)buf[0] << 24) | ((uint32_t)buf[1] << 16) | ((uint32_t)buf[2] << 8) | buf[3]);
    out->seq_id = (uint32_t)(((uint32_t)buf[4] << 24) | ((uint32_t)buf[5] << 16) | ((uint32_t)buf[6] << 8) | buf[7]);
    const size_t sample_len = payload_size - 8;
    out->data_transferred.len = sample_len;
    if (sample_len > 0) {
        if (out->data_transferred.data == NULL) return -1;
        memcpy(out->data_transferred.data, buf + 8, sample_len);
    }
    return 0;
}

int ecpri_remote_memory_access_encode(
    const RemoteMemoryAccess *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    const uint16_t payload_size = (uint16_t)(12u + (uint16_t)in->data.len);
    if (cap < (size_t)payload_size) return -1;
    for (size_t i = 0; i < 12; ++i) buf[i] = 0;
    buf[0] = (uint8_t)(in->remote_memory_access_id & 0xFFu);
    buf[1] |= (uint8_t)((in->read_write & 0xFu) << 4);
    buf[1] |= (uint8_t)((in->req_resp & 0xFu) << 0);
    buf[2] = (uint8_t)((in->element_id >> 8) & 0xFFu);
    buf[3] = (uint8_t)((in->element_id >> 0) & 0xFFu);
    buf[4] = (uint8_t)((in->address >> 40) & 0xFFu);
    buf[5] = (uint8_t)((in->address >> 32) & 0xFFu);
    buf[6] = (uint8_t)((in->address >> 24) & 0xFFu);
    buf[7] = (uint8_t)((in->address >> 16) & 0xFFu);
    buf[8] = (uint8_t)((in->address >> 8) & 0xFFu);
    buf[9] = (uint8_t)((in->address >> 0) & 0xFFu);
    buf[10] = (uint8_t)((in->length >> 8) & 0xFFu);
    buf[11] = (uint8_t)((in->length >> 0) & 0xFFu);
    if (in->data.len > 0) {
        if (in->data.data == NULL) return -1;
        memcpy(buf + 12, in->data.data, in->data.len);
    }
    *out_len = payload_size;
    return 0;
}

int ecpri_remote_memory_access_decode(
    const uint8_t *buf,
    size_t len,
    uint16_t payload_size,
    RemoteMemoryAccess *out
) {
    if (payload_size < 12 || len < (size_t)payload_size) return -1;
    out->remote_memory_access_id = (uint8_t)(buf[0] & 0xFFu);
    out->read_write = (RMAReadWrite)((buf[1] >> 4) & 0xFu);
    out->req_resp = (RMAReqResp)((buf[1] >> 0) & 0xFu);
    out->element_id = (uint16_t)(((uint16_t)buf[2] << 8) | buf[3]);
    uint64_t __mek_address = 0;
    __mek_address = (uint64_t)((__mek_address << 8) | buf[4]);
    __mek_address = (uint64_t)((__mek_address << 8) | buf[5]);
    __mek_address = (uint64_t)((__mek_address << 8) | buf[6]);
    __mek_address = (uint64_t)((__mek_address << 8) | buf[7]);
    __mek_address = (uint64_t)((__mek_address << 8) | buf[8]);
    __mek_address = (uint64_t)((__mek_address << 8) | buf[9]);
    out->address = __mek_address;
    out->length = (uint16_t)(((uint16_t)buf[10] << 8) | buf[11]);
    const size_t sample_len = payload_size - 12;
    out->data.len = sample_len;
    if (sample_len > 0) {
        if (out->data.data == NULL) return -1;
        memcpy(out->data.data, buf + 12, sample_len);
    }
    return 0;
}

int ecpri_one_way_delay_measurement_encode(
    const OneWayDelayMeasurement *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    const uint16_t payload_size = (uint16_t)(20u + (uint16_t)in->dummy_bytes.len);
    if (cap < (size_t)payload_size) return -1;
    buf[0] = (uint8_t)(in->measurement_id & 0xFFu);
    buf[1] = (uint8_t)(in->action_type & 0xFFu);
    {
        size_t nested_len = 0;
        if (ecpri_ptp_timestamp_encode(&in->timestamp, buf + 2, cap - 2, &nested_len) != 0) return -1;
        if (nested_len != 10u) return -1;
    }
    buf[12] = (uint8_t)((in->compensation_value >> 56) & 0xFFu);
    buf[13] = (uint8_t)((in->compensation_value >> 48) & 0xFFu);
    buf[14] = (uint8_t)((in->compensation_value >> 40) & 0xFFu);
    buf[15] = (uint8_t)((in->compensation_value >> 32) & 0xFFu);
    buf[16] = (uint8_t)((in->compensation_value >> 24) & 0xFFu);
    buf[17] = (uint8_t)((in->compensation_value >> 16) & 0xFFu);
    buf[18] = (uint8_t)((in->compensation_value >> 8) & 0xFFu);
    buf[19] = (uint8_t)(in->compensation_value & 0xFFu);
    if (in->dummy_bytes.len > 0) {
        if (in->dummy_bytes.data == NULL) return -1;
        memcpy(buf + 20, in->dummy_bytes.data, in->dummy_bytes.len);
    }
    *out_len = payload_size;
    return 0;
}

int ecpri_one_way_delay_measurement_decode(
    const uint8_t *buf,
    size_t len,
    uint16_t payload_size,
    OneWayDelayMeasurement *out
) {
    if (payload_size < 20 || len < (size_t)payload_size) return -1;
    out->measurement_id = (uint8_t)(buf[0] & 0xFFu);
    out->action_type = (uint8_t)(buf[1] & 0xFFu);
    if (ecpri_ptp_timestamp_decode(buf + 2, len - 2, &out->timestamp) != 0) return -1;
    out->compensation_value = (uint64_t)(((uint64_t)buf[12] << 56) | ((uint64_t)buf[13] << 48) | ((uint64_t)buf[14] << 40) | ((uint64_t)buf[15] << 32) | ((uint64_t)buf[16] << 24) | ((uint64_t)buf[17] << 16) | ((uint64_t)buf[18] << 8) | buf[19]);
    const size_t sample_len = payload_size - 20;
    out->dummy_bytes.len = sample_len;
    if (sample_len > 0) {
        if (out->dummy_bytes.data == NULL) return -1;
        memcpy(out->dummy_bytes.data, buf + 20, sample_len);
    }
    return 0;
}

int ecpri_remote_reset_encode(
    const RemoteReset *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    const uint16_t payload_size = (uint16_t)(3u + (uint16_t)in->vendor_specific_payload.len);
    if (cap < (size_t)payload_size) return -1;
    buf[0] = (uint8_t)((in->reset_id >> 8) & 0xFFu);
    buf[1] = (uint8_t)(in->reset_id & 0xFFu);
    buf[2] = (uint8_t)(in->reset_code_op & 0xFFu);
    if (in->vendor_specific_payload.len > 0) {
        if (in->vendor_specific_payload.data == NULL) return -1;
        memcpy(buf + 3, in->vendor_specific_payload.data, in->vendor_specific_payload.len);
    }
    *out_len = payload_size;
    return 0;
}

int ecpri_remote_reset_decode(
    const uint8_t *buf,
    size_t len,
    uint16_t payload_size,
    RemoteReset *out
) {
    if (payload_size < 3 || len < (size_t)payload_size) return -1;
    out->reset_id = (uint16_t)(((uint16_t)buf[0] << 8) | buf[1]);
    out->reset_code_op = (uint8_t)(buf[2] & 0xFFu);
    const size_t sample_len = payload_size - 3;
    out->vendor_specific_payload.len = sample_len;
    if (sample_len > 0) {
        if (out->vendor_specific_payload.data == NULL) return -1;
        memcpy(out->vendor_specific_payload.data, buf + 3, sample_len);
    }
    return 0;
}

int ecpri_event_indication_encode(
    const EventIndication *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    const uint16_t payload_size = (uint16_t)(4u + (uint16_t)(in->records.len * 8));
    if (cap < (size_t)payload_size) return -1;
    for (size_t i = 0; i < 4; ++i) buf[i] = 0;
    buf[0] = (uint8_t)(in->event_type & 0xFFu);
    buf[1] = (uint8_t)(in->event_id & 0xFFu);
    buf[2] = (uint8_t)(in->sequence_number & 0xFFu);
    buf[3] = (uint8_t)(in->number_of_faults_or_notifications & 0xFFu);
    for (size_t i = 0; i < in->records.len; ++i) {
        size_t elem_len = 0;
        if (ecpri_fault_notification_record_encode(&in->records.data[i], buf + 4 + i * 8, cap - (4 + i * 8), &elem_len) != 0)
            return -1;
        if (elem_len != 8) return -1;
    }
    *out_len = payload_size;
    return 0;
}

int ecpri_event_indication_decode(
    const uint8_t *buf,
    size_t len,
    uint16_t payload_size,
    EventIndication *out
) {
    if (payload_size < 4 || len < (size_t)payload_size) return -1;
    out->event_type = (EventType)(buf[0] & 0xFFu);
    out->event_id = (uint8_t)(buf[1] & 0xFFu);
    out->sequence_number = (uint8_t)(buf[2] & 0xFFu);
    out->number_of_faults_or_notifications = (uint8_t)(buf[3] & 0xFFu);
    const size_t tail_bytes = payload_size - 4;
    if (tail_bytes % 8 != 0) return -1;
    const size_t count = tail_bytes / 8;
    out->records.len = count;
    for (size_t i = 0; i < count; ++i) {
        if (ecpri_fault_notification_record_decode(buf + 4 + i * 8, 8, &out->records.data[i]) != 0)
            return -1;
    }
    return 0;
}

int ecpri_payload_encode(
    const Payload *in,
    MessageType tag,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    switch (tag) {
    case MessageType_iq_data:
        return ecpri_iq_data_encode(&in->body.iq_data, buf, cap, out_len);
    case MessageType_bit_sequence:
        return ecpri_bit_sequence_encode(&in->body.bit_sequence, buf, cap, out_len);
    case MessageType_real_time_control_data:
        return ecpri_real_time_control_data_encode(&in->body.real_time_control_data, buf, cap, out_len);
    case MessageType_generic_data_transfer:
        return ecpri_generic_data_transfer_encode(&in->body.generic_data_transfer, buf, cap, out_len);
    case MessageType_remote_memory_access:
        return ecpri_remote_memory_access_encode(&in->body.remote_memory_access, buf, cap, out_len);
    case MessageType_one_way_delay_measurement:
        return ecpri_one_way_delay_measurement_encode(&in->body.one_way_delay_measurement, buf, cap, out_len);
    case MessageType_remote_reset:
        return ecpri_remote_reset_encode(&in->body.remote_reset, buf, cap, out_len);
    case MessageType_event_indication:
        return ecpri_event_indication_encode(&in->body.event_indication, buf, cap, out_len);
    case MessageType_iwf_start_up:
        return ecpri_opaque_payload_encode(&in->body.iwf_start_up, buf, cap, out_len);
    case MessageType_iwf_operation:
        return ecpri_opaque_payload_encode(&in->body.iwf_operation, buf, cap, out_len);
    case MessageType_iwf_mapping:
        return ecpri_opaque_payload_encode(&in->body.iwf_mapping, buf, cap, out_len);
    case MessageType_iwf_delay_control:
        return ecpri_opaque_payload_encode(&in->body.iwf_delay_control, buf, cap, out_len);
    case MessageType_reserved_12:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_13:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_14:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_15:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_16:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_17:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_18:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_19:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_20:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_21:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_22:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_23:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_24:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_25:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_26:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_27:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_28:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_29:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_30:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_31:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_32:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_33:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_34:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_35:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_36:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_37:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_38:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_39:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_40:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_41:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_42:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_43:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_44:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_45:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_46:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_47:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_48:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_49:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_50:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_51:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_52:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_53:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_54:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_55:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_56:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_57:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_58:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_59:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_60:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_61:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_62:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_reserved_63:
        return ecpri_opaque_payload_encode(&in->body.reserved, buf, cap, out_len);
    case MessageType_vendor_specific_64:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_65:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_66:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_67:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_68:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_69:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_70:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_71:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_72:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_73:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_74:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_75:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_76:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_77:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_78:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_79:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_80:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_81:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_82:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_83:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_84:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_85:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_86:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_87:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_88:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_89:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_90:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_91:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_92:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_93:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_94:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_95:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_96:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_97:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_98:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_99:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_100:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_101:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_102:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_103:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_104:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_105:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_106:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_107:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_108:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_109:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_110:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_111:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_112:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_113:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_114:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_115:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_116:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_117:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_118:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_119:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_120:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_121:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_122:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_123:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_124:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_125:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_126:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_127:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_128:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_129:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_130:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_131:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_132:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_133:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_134:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_135:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_136:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_137:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_138:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_139:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_140:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_141:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_142:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_143:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_144:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_145:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_146:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_147:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_148:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_149:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_150:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_151:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_152:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_153:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_154:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_155:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_156:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_157:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_158:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_159:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_160:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_161:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_162:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_163:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_164:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_165:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_166:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_167:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_168:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_169:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_170:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_171:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_172:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_173:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_174:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_175:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_176:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_177:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_178:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_179:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_180:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_181:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_182:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_183:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_184:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_185:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_186:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_187:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_188:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_189:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_190:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_191:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_192:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_193:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_194:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_195:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_196:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_197:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_198:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_199:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_200:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_201:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_202:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_203:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_204:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_205:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_206:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_207:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_208:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_209:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_210:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_211:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_212:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_213:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_214:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_215:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_216:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_217:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_218:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_219:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_220:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_221:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_222:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_223:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_224:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_225:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_226:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_227:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_228:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_229:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_230:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_231:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_232:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_233:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_234:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_235:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_236:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_237:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_238:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_239:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_240:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_241:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_242:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_243:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_244:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_245:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_246:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_247:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_248:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_249:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_250:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_251:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_252:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_253:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_254:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    case MessageType_vendor_specific_255:
        return ecpri_opaque_payload_encode(&in->body.vendor_specific, buf, cap, out_len);
    default:
        return -1;
    }
}

int ecpri_payload_decode(
    const uint8_t *buf,
    size_t len,
    MessageType tag,
    uint16_t payload_size,
    Payload *out
) {
    out->tag = tag;
    switch (tag) {
    case MessageType_iq_data:
        return ecpri_iq_data_decode(buf, len, payload_size, &out->body.iq_data);
    case MessageType_bit_sequence:
        return ecpri_bit_sequence_decode(buf, len, payload_size, &out->body.bit_sequence);
    case MessageType_real_time_control_data:
        return ecpri_real_time_control_data_decode(buf, len, payload_size, &out->body.real_time_control_data);
    case MessageType_generic_data_transfer:
        return ecpri_generic_data_transfer_decode(buf, len, payload_size, &out->body.generic_data_transfer);
    case MessageType_remote_memory_access:
        return ecpri_remote_memory_access_decode(buf, len, payload_size, &out->body.remote_memory_access);
    case MessageType_one_way_delay_measurement:
        return ecpri_one_way_delay_measurement_decode(buf, len, payload_size, &out->body.one_way_delay_measurement);
    case MessageType_remote_reset:
        return ecpri_remote_reset_decode(buf, len, payload_size, &out->body.remote_reset);
    case MessageType_event_indication:
        return ecpri_event_indication_decode(buf, len, payload_size, &out->body.event_indication);
    case MessageType_iwf_start_up:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.iwf_start_up);
    case MessageType_iwf_operation:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.iwf_operation);
    case MessageType_iwf_mapping:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.iwf_mapping);
    case MessageType_iwf_delay_control:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.iwf_delay_control);
    case MessageType_reserved_12:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_13:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_14:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_15:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_16:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_17:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_18:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_19:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_20:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_21:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_22:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_23:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_24:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_25:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_26:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_27:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_28:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_29:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_30:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_31:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_32:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_33:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_34:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_35:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_36:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_37:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_38:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_39:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_40:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_41:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_42:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_43:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_44:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_45:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_46:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_47:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_48:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_49:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_50:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_51:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_52:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_53:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_54:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_55:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_56:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_57:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_58:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_59:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_60:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_61:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_62:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_reserved_63:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.reserved);
    case MessageType_vendor_specific_64:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_65:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_66:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_67:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_68:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_69:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_70:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_71:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_72:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_73:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_74:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_75:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_76:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_77:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_78:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_79:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_80:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_81:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_82:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_83:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_84:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_85:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_86:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_87:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_88:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_89:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_90:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_91:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_92:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_93:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_94:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_95:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_96:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_97:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_98:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_99:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_100:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_101:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_102:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_103:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_104:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_105:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_106:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_107:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_108:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_109:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_110:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_111:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_112:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_113:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_114:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_115:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_116:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_117:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_118:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_119:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_120:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_121:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_122:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_123:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_124:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_125:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_126:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_127:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_128:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_129:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_130:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_131:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_132:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_133:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_134:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_135:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_136:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_137:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_138:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_139:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_140:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_141:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_142:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_143:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_144:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_145:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_146:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_147:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_148:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_149:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_150:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_151:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_152:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_153:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_154:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_155:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_156:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_157:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_158:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_159:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_160:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_161:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_162:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_163:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_164:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_165:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_166:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_167:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_168:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_169:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_170:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_171:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_172:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_173:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_174:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_175:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_176:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_177:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_178:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_179:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_180:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_181:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_182:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_183:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_184:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_185:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_186:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_187:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_188:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_189:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_190:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_191:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_192:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_193:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_194:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_195:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_196:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_197:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_198:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_199:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_200:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_201:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_202:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_203:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_204:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_205:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_206:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_207:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_208:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_209:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_210:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_211:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_212:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_213:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_214:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_215:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_216:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_217:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_218:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_219:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_220:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_221:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_222:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_223:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_224:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_225:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_226:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_227:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_228:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_229:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_230:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_231:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_232:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_233:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_234:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_235:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_236:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_237:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_238:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_239:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_240:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_241:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_242:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_243:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_244:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_245:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_246:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_247:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_248:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_249:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_250:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_251:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_252:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_253:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_254:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    case MessageType_vendor_specific_255:
        return ecpri_opaque_payload_decode(buf, len, payload_size, &out->body.vendor_specific);
    default:
        return -1;
    }
}

int ecpri_message_encode(
    const Message *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    size_t body_len = 0;
    if (cap < 4) return -1;
    if (ecpri_payload_encode(&in->payload, in->header.message_type, buf + 4, cap - 4, &body_len) != 0)
        return -1;
  {
        Header hdr = in->header;
        size_t hdr_len = 0;
        if (ecpri_header_encode(&hdr, (uint16_t)body_len, buf, 4, &hdr_len) != 0)
            return -1;
    }
    *out_len = 4 + body_len;
    return 0;
}

int ecpri_message_decode(
    const uint8_t *buf,
    size_t len,
    Message *out
) {
    uint16_t body_size = 0;
    if (ecpri_header_decode(buf, len, &body_size, &out->header) != 0)
        return -1;
    if (len < (size_t)4 + body_size)
        return -1;
    return ecpri_payload_decode(buf + 4, body_size, out->header.message_type, body_size, &out->payload);
}

static size_t pdu_message_padding_len(size_t msg_len, int concat) {
    if (!concat) return 0;
    return (size_t)((4u - (msg_len % 4u)) % 4u);
}

int ecpri_pdu_message_encode(
    const PduMessage *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    size_t msg_len = 0;
    if (ecpri_message_encode(&in->message, buf, cap, &msg_len) != 0)
        return -1;
    const size_t padding = pdu_message_padding_len(msg_len, (int)in->message.header.concatenation);
    if (cap < msg_len + padding) return -1;
    if (padding > 0)
        memset(buf + msg_len, 0, padding);
    *out_len = msg_len + padding;
    return 0;
}

int ecpri_pdu_message_decode(
    const uint8_t *buf,
    size_t len,
    PduMessage *out,
    size_t *consumed
) {
    uint16_t body_size = 0;
    if (ecpri_header_decode(buf, len, &body_size, &out->message.header) != 0)
        return -1;
    if (ecpri_message_decode(buf, len, &out->message) != 0)
        return -1;
    const size_t msg_len = 4u + (size_t)body_size;
    const size_t padding = pdu_message_padding_len(msg_len, (int)out->message.header.concatenation);
    if (len < msg_len + padding) return -1;
    *consumed = msg_len + padding;
    return 0;
}

int ecpri_pdu_encode(
    const Pdu *in,
    uint8_t *buf,
    size_t cap,
    size_t *out_len
) {
    size_t offset = 0;
    for (size_t i = 0; i < in->messages.len; ++i) {
        PduMessage entry = in->messages.data[i];
        if (i + 1 < in->messages.len) {
            entry.message.header.concatenation = 1;
        } else {
            entry.message.header.concatenation = 0;
        }
        size_t n = 0;
        if (ecpri_pdu_message_encode(&entry, buf + offset, cap - offset, &n) != 0)
            return -1;
        offset += n;
    }
    *out_len = offset;
    return 0;
}

int ecpri_pdu_decode(
    const uint8_t *buf,
    size_t len,
    Pdu *out
) {
    size_t offset = 0;
    size_t count = 0;
    while (offset < len) {
        if (count >= out->messages.len) return -1;
        size_t consumed = 0;
        if (ecpri_pdu_message_decode(buf + offset, len - offset, &out->messages.data[count], &consumed) != 0)
            return -1;
        offset += consumed;
        count++;
    }
    out->messages.len = count;
    return 0;
}

