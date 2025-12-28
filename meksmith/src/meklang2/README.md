# bit-exact protocol for encoding & decoding

```
(* ========================================================= *)
(*  File                                                     *)
(* ========================================================= *)

file                = { declaration } ;


(* ========================================================= *)
(*  Declarations                                             *)
(* ========================================================= *)

declaration         = bitstruct_decl
                    | enum_decl
                    | typedef_decl ;


(* ========================================================= *)
(*  Bitstruct (bit-level layout)                              *)
(* ========================================================= *)

bitstruct_decl      = "bitstruct" ident "(" bitorder ")" "{"
                        { bit_item }
                      "}" ;

bitorder            = "msb0" | "lsb0" ;


(* ========================================================= *)
(*  Bitstruct items                                          *)
(* ========================================================= *)

bit_item            = bit_field
                    | bit_union ;


(* --------------------------------------------------------- *)
(*  Simple bit field                                         *)
(* --------------------------------------------------------- *)

bit_field           = "bits" bit_range ident ":" type_ref
                      [ bit_attrs ]
                      ";" ;


(* --------------------------------------------------------- *)
(*  Bit union / overlay                                      *)
(* --------------------------------------------------------- *)

bit_union           = "bits" bit_range
                      "union" ident
                      "when" selector
                      "{"
                          { union_case }
                      "}" ;


(* ========================================================= *)
(*  Bit range                                                *)
(* ========================================================= *)

bit_range           = number
                    | number ".." number
                    | number ".." "*" ;


(* ========================================================= *)
(*  Union selector (expression)                              *)
(* ========================================================= *)

selector            = expression ;


(* --------------------------------------------------------- *)
(*  Expressions                                              *)
(* --------------------------------------------------------- *)

expression          = or_expr ;

or_expr             = and_expr { "||" and_expr } ;

and_expr            = cmp_expr { "&&" cmp_expr } ;

cmp_expr            = primary [ cmp_op primary ] ;

cmp_op              = "==" | "!=" | "<" | "<=" | ">" | ">=" ;


(* --------------------------------------------------------- *)
(*  Expression atoms                                         *)
(* --------------------------------------------------------- *)

primary             = field_ref
                    | bit_test
                    | literal
                    | "(" expression ")" ;


(* --------------------------------------------------------- *)
(*  Field reference (supports nesting)                       *)
(* --------------------------------------------------------- *)

field_ref           = ident { "." ident } ;


(* --------------------------------------------------------- *)
(*  Bit test                                                 *)
(* --------------------------------------------------------- *)

bit_test            = "bits" bit_range "==" number ;


(* ========================================================= *)
(*  Union variant                                            *)
(* ========================================================= *)

union_case          = number "=>" ident ":" type_ref ";" ;


(* ========================================================= *)
(*  Types                                                    *)
(* ========================================================= *)

type_ref            = primitive_type
                    | ident ;


(* --------------------------------------------------------- *)
(*  Primitive types                                          *)
(* --------------------------------------------------------- *)

primitive_type      = unsigned_type
                    | signed_type
                    | "bool"
                    | "bytes" ;

unsigned_type       = "u" number ;
signed_type         = "i" number ;


(* ========================================================= *)
(*  Field attributes                                         *)
(* ========================================================= *)

bit_attrs           = "[" attr { "," attr } "]" ;

attr                = "le"
                    | "be"
                    | "reserved"
                    | "deprecated"
                    | attr_kv ;

attr_kv             = ident "=" literal ;


(* ========================================================= *)
(*  Enums (logical types)                                    *)
(* ========================================================= *)

enum_decl           = "enum" ident "(" enum_repr ")" "{"
                        { enum_variant }
                      "}" ;

enum_repr           = unsigned_type ;

enum_variant        = ident "=" number ";" ;


(* ========================================================= *)
(*  Typedef                                                  *)
(* ========================================================= *)

typedef_decl        = "type" ident "=" type_ref ";" ;


(* ========================================================= *)
(*  Literals                                                 *)
(* ========================================================= *)

literal             = number
                    | string
                    | "true"
                    | "false" ;


(* ========================================================= *)
(*  Lexical elements                                         *)
(* ========================================================= *)

ident               = letter { letter | digit | "_" } ;

number              = decimal
                    | hex ;

decimal             = digit { digit } ;

hex                 = "0x" hex_digit { hex_digit } ;

string              = "\"" { character } "\"" ;


letter              = "A"…"Z" | "a"…"z" ;
digit               = "0"…"9" ;
hex_digit           = digit | "A"…"F" | "a"…"f" ;
character           = ? any unicode character except " ? ;


(* ========================================================= *)
(*  Comments (ignored by parser)                             *)
(* ========================================================= *)

comment             = "//" { character } newline
                    | "/*" { character } "*/" ;

```


```
file                = { declaration } ;

declaration         = bitstruct_decl | enum_decl | typedef_decl ;

bitstruct_decl      = "bitstruct" ident "(" bitorder ")" "{" { bit_item } "}" ;
bitorder            = "msb0" | "lsb0" ;
bit_item            = bit_field | bit_union ;
bit_field           = "bits" bit_range ident ":" type_ref [ bit_attrs ] ";" ;
bit_union           = "bits" bit_range "union" ident "when" selector "{" { union_case } "}" ;
bit_range           = number | number ".." number | number ".." "*" ;

selector            = expression ;
expression          = or_expr ;
or_expr             = and_expr { "||" and_expr } ;
and_expr            = cmp_expr { "&&" cmp_expr } ;
cmp_expr            = primary [ cmp_op primary ] ;
cmp_op              = "==" | "!=" | "<" | "<=" | ">" | ">=" ;

primary             = field_ref | bit_test | literal | "(" expression ")" ;
field_ref           = ident { "." ident } ;
bit_test            = "bits" bit_range "==" number ;

union_case          = number "=>" ident ":" type_ref ";" ;
type_ref            = primitive_type | ident ;

primitive_type      = unsigned_type | signed_type | "bool" | "bytes" ;
unsigned_type       = "u" number ;
signed_type         = "i" number ;

bit_attrs           = "[" attr { "," attr } "]" ;
attr                = "le" | "be" | "reserved" | "deprecated" | attr_kv ;
attr_kv             = ident "=" literal ;

enum_decl           = "enum" ident "(" enum_repr ")" "{" { enum_variant } "}" ;
enum_repr           = unsigned_type ;
enum_variant        = ident "=" number ";" ;

typedef_decl        = "type" ident "=" type_ref ";" ;

literal             = number | string | "true" | "false" ;

ident               = letter { letter | digit | "_" } ;
number              = decimal | binary | hex ;

decimal             = digit { digit } ;
binary              = "0b" binary_digit { binary_digit };
hexadecimal         = "0x" hexadecimal_digit { hexadecimal_digit } ;

digit               = "0"…"9" ;
binary_digit        = "0" | "1" ;
hexadecimal_digit   = digit | "A"…"F" | "a"…"f" ;

string              = "\"" { character } "\"" ;
letter              = "A"…"Z" | "a"…"z" ;
character           = ? any unicode character except " ? ;

comment             = "//" { character } newline | "/*" { character } "*/" ;
```
