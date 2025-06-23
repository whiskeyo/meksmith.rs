```plaintext
<protocol> ::= (<definition> | <comment>)+
<comment> ::= '#' <text> '\n'
<definition> ::=
      <enumeration_definition>
    | <structure_definition>
    | <union_definition>
    | <type_definition>

<enumeration_definition> ::= 'enum' <identifier> <left_brace> <enumeration_field>+ <right_brace> <semicolon>
<enumeration_field> ::= <identifier> <equal> (<unsigned_integer> | <range>) <semicolon>

<structure_definition> ::= 'struct' <identifier> <left_brace> <structure_field>+ <right_brace> <semicolon>
<structure_field> ::= [<attributes>] <identifier> <colon> <type_identifier> <semicolon>

<union_definition> ::= 'union' <identifier> <left_brace> <union_field>+ <right_brace> <semicolon>
<union_field> ::= (<unsigned_integer> | <range>) <maps_to> <identifier> <colon> <type_identifier> <semicolon>

<attribute> ::=
      'discriminated_by' <equal> <identifier>
    | 'bits' <equal> <unsigned_integer>
    | 'bytes' <equal> <unsigned_integer>
<attribute_tail> ::= <comma> <attribute>
<attributes> ::= <left_bracket> <attribute> <attribute_tail>* <right_bracket>

<type_definition> ::= 'using' <identifier> <equal> <type_identifier> <semicolon>

<type_identifier> ::=
      <builtin_type>
    | <user_defined_type>
    | <static_array_type>
    | <dynamic_array_type>

<builtin_type> ::=
      'int8' | 'int16' | 'int32' | 'int64'
    | 'uint8' | 'uint16' | 'uint32' | 'uint64'
    | 'float32' | 'float64'
    | 'bit' | 'byte'
<user_defined_type> ::= <identifier>
<static_array_type> ::=
      <builtin_type> <left_bracket> <unsigned_integer> <right_bracket>
    | <user_defined_type> <left_bracket> <unsigned_integer> <right_bracket>
<dynamic_array_type> ::=
      <builtin_type> <left_bracket> <right_bracket>
    | <user_defined_type> <left_bracket> <right_bracket>

<range> ::= <unsigned_integer> <double_dot> <unsigned_integer>
<identifier> ::= [a-zA-Z_][a-zA-Z0-9_]*

<unsigned_integer> ::= <hexadecimal> | <binary> | <decimal>
<hexadecimal> ::= "0x" [0-9a-fA-F]+
<binary> ::= "0b" [01]+
<decimal> ::= [0-9]+

<text> ::= [^\n]*

<left_brace> ::= '{'
<right_brace> ::= '}'
<left_bracket> ::= '['
<right_bracket> ::= ']'
<semicolon> ::= <semicolon>
<colon> ::= ':'
<maps_to> ::= '=>'
<equal> ::= '='
<comma> ::= ','
<double_dot> ::= '..'
```
