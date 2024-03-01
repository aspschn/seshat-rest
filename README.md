Seshat Web Demo REST API
========================

## Browse

### Browse Blocks

**GET** `/api/v3/unicode/browse/blocks`


### Browse the Code Points in the Block

**GET** `/api/v3/unicode/browse/blocks/<block>`

`<block>` is an abbreviated name.


## Properties

### Get Properties

**GET** `/api/v3/unicode/properties/<code_point>`

`<code_point>` is a Unicode character code notation without "U+".
The valid range is from 0000 to 10FFFF.


## Properties (v2)

/api/v2/unicode/properties/\<codepoint\>

### response example
```
{
    "codepoint": "U+AC00",
    "properties": {
        "numeric": {},
        "string": {},
        "misc": {
            "name": "HANGUL SYLLABLE GA"
        },
        "catalog": {
            "age": "2.0",
            "block": {
                "full": "",
                "abbr": ""
            },
            "script": {
                "full": "Hangul",
                "abbr": ""
            }
        },
        "enum": {
            "canonical_combining_class": 000,
            "grapheme_cluster_break": ""
        },
        "binary": {
            "math": false,
            "lowercase": false
        }
    },
    "seshat_version": "0.0.11"
}
```

## Segmentation (v2)

/api/v2/unicode/segmentation/graphemes/\<example text here\>

### response example
`
{
    "text": "example text here",
    "type": "grapheme",
    "breaks": [
    ],
    "seshat_version": "0.0.11"
}
`
