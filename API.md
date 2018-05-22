root
-----
/api/

unicode
-----
/api/unicode/

properties
-----
/api/unicode/properties
/api/unicode/properties/AC00

### response
`
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
			"math": ,
			"lowercase":
		}
	}
}
`

segmentation
------------
/api/unicode/segmentation/grapheme/example text here

### response
`
{
	"text": "example text here",
	"type": "grapheme",
	"breaks": [
	]
}
`
