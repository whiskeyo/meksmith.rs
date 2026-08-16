# meksmith

## Language features

- user friendly syntax, which is easy to read for engineers not strictly related to coding
- padding-free definitions for binary protocols: no explicit padding = no unexpected payload in the message
- attributes for defining field size, optionality, dependencies to other fields, etc.
- support for both MSB and LSB (most/least significant bit) definitions to follow frame formats easily

## Language syntax

### Language definition

A few assumptions for easier reading of the notation below:

- `"some_text"` is a precise input, e.g. `"enumerated"` must match the string
- `|` is "or",
- `{ ... }` is a repetition, e.g. a list of items
- `[ ... ]` is an optional field, e.g. trailing comma at the end of the list

```
enumerated = "enumerated" "(" bit_order "," integer bit_bits ")" identifier "{"
                 enumerated_field { "," enumerated_field } [ "," ]
             "}"

enumerated_field = identifier "=" integer
                 | identifier "=" range

choice = "choice" "(" bit_order ")" identifier "(" [ typed_argument_list ] ")" "on" identifier "{"
             choice_field { "," choice_field } [ "," ]
         "}"

choice_field = ( integer | identifier "::" identifier ) "=>"

typed_argument_list = typed_argument { "," typed_argument } [ "," ]
typed_argument = identifier ":" identifier

argument_list = identifier { "," identifier } [ "," ]

bit_bits = "bit" | "bits"
bit_order = "msb0" | "lsb0"
range = integer ".." integer

integer = dec_int | hex_int | bin_int
dec_int = { "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" }
hex_int = "0x" { "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
               | "a" | "b" | "c" | "d" | "e" | "f" | "A" | "B" | "C" | "D" | "E" | "F" }
bin_int = "0b" { "0" | "1" }
```

### Protocol metadata syntax

...

### `structure` syntax

`structure` can be represented as:

```
structure(BIT_ORDER) NAME(ARGUMENTS) {
    FIELD_NAME (FIELD_LENGTH bit/bits): UNDERLYING_TYPE [ATTRIBUTES_LIST],
    ...
}
```

where:

- `BIT_ORDER` represents the order of the frame format, it is either `msb0` or `lsb0`,
- `NAME` represents the name of the structure,
- `ARGUMENTS` represents the dependency to other fields
- `FIELD_NAME` represents the name of the field,
- `FIELD_LENGTH` represents the length of the field; that part in parentheses is optional,
- `UNDERLYING_TYPE` represents the type that is used in the generated code for the field,
- `ATTRIBUTES_LIST` represents the list of attributes of a field.

Each `structure` can have many fields.

## TODOs to consider

### Arrays

- `bytearray [length=some_length]` would be cool for payloads, but another solution can be considered to avoid attributes, e.g.
- `StaticArray<byte, SIZE>` and `DynamicArray<byte, DYNAMIC_SIZE>`, which would wrap the whole type into an array, and attributes could be used for other purposes, such as `[no_debug]`, `[unused]`, etc.
