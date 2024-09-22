; Comments
(comment) @comment

; Keys
(block_mapping_pair
  key: (_) @property)
(flow_mapping
  (_ 
    key: (_) @property))

; String values (excluding those used as keys)
(block_mapping_pair
  value: (flow_node [
    (double_quote_scalar)
    (single_quote_scalar)
    (plain_scalar (string_scalar))
  ]) @string)
(flow_mapping
  (_ 
    value: (flow_node [
      (double_quote_scalar)
      (single_quote_scalar)
      (plain_scalar (string_scalar))
    ]) @string))
(flow_sequence
  (flow_node [
    (double_quote_scalar)
    (single_quote_scalar)
    (plain_scalar (string_scalar))
  ]) @string)

; List items
(block_sequence_item
  (flow_node [
    (double_quote_scalar)
    (single_quote_scalar)
    (plain_scalar)
  ]) @string)

; Numeric values
(integer_scalar) @number
(float_scalar) @number

; Boolean values
((boolean_scalar) @boolean)

; Null values
((null_scalar) @constant.builtin)

; Multi-line strings
(block_scalar) @string

; Other YAML elements
(anchor_name) @label
(alias_name) @label
(tag) @type
(yaml_directive) @attribute
(tag_directive) @attribute
(reserved_directive) @attribute

; Punctuation
["," "-" ":" ">" "?" "|"] @punctuation.delimiter
["[" "]" "{" "}"] @punctuation.bracket
["*" "&" "---" "..."] @punctuation.special