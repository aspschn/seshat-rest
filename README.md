# Seshat Web REST API

This is the official REST API and a reference implementation of Seshat Unicode library.

Current API version is v3.

## Build and Run

The API server is written in Rust, and [Rocket](https://rocket.rs).

Simply done with

```shell
$ cargo run --release
```


## API Specifications

### Browse

Browse by blocks, scripts, etc.

APIs in this category are heavy since they iterate through all code points. However, this is Rust, fast enough.

#### Browse Blocks

**GET** `/api/v3/unicode/browse/blocks`


#### Browse the Code Points in the Block

**GET** `/api/v3/unicode/browse/blocks/<block>`

`<block>` is an abbreviated name.


### Properties

Get the properties of the single code point.

#### Get Properties

**GET** `/api/v3/unicode/properties/<code_point>`

`<code_point>` is a Unicode character code notation without "U+".
The valid range is from 0000 to 10FFFF.


### Segmentation

Grapheme, word, line break of the given text.

#### Grapheme

**GET** `/api/v3/unicode/segmentation/grapheme/<text>`


### Normalization

Normalize the given text in NFD, NFC, NFKD and NFKC forms.

#### NFD

**GET** `/api/v3/unicode/normalization/nfd/<text>`

#### NFC

**GET** `/api/v3/unicode/normalization/nfc/<text>`

#### NFKD

**GET** `/api/v3/unicode/normalization/nfkd/<text>`

#### NFKC

**GET** `/api/v3/unicode/normalization/nfkc/<text>`


________________________
Below are the old v2 APIs
________________________


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
