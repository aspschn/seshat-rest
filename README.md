Seshat Web Demo REST API
========================

Properties
-----------

/api/unicode/properties/\<codepoint\>

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

Segmentation
------------
/api/unicode/segmentation/grapheme/\<example text here\>

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