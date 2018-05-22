/*
//  unicode.cpp
//
//  Author:     hardboiled65
//  Created:    2018. 03. 11. 08:39
//  Copyright (c) 2018 hardboiled65. All rights reserved.
//
//
*/
#include "unicode.h"

#include <iostream>
#include <string>

#include <uriparser/Uri.h>

#include <nlohmann/json.hpp>

#include <seshat/codepoint.h>
#include <seshat/character.h>
#include <seshat/string.h>
#include <seshat/unicode/casing.h>
#include <seshat/unicode/name.h>
#include <seshat/unicode/normalization_props.h>
#include <seshat/unicode/properties.h>
#include <seshat/unicode/property_names.h>
#include <seshat/unicode/version.h>
#include <seshat/utils.h>

#include "paths.h"
#include "property_name.h"

using namespace seshat;
using nlohmann::json;

namespace seshat_rest {

int response_exception(const std::exception& e)
{
    json res;
    res["exception"] = std::string(e.what());
    std::cout << "Status: 403 Bad Request\r\n"
        << "Content-type: application/json\r\n"
        << "Access-control-Allow-Origin: *\r\n\r\n"
        << res.dump();
    return 0;
}

int unicode(char **paths, int len)
{
    if (len > 2 && std::string(paths[2]) == API_PATH_PROPERTIES) {
        return properties(paths, len);
    } else if (len > 2 && std::string(paths[2]) == API_PATH_SEGMENTATION) {
        return segmentation(paths, len);
    }
    return 1; // Error
}

int properties(char **paths, int len)
{
    uint32_t cp;

    if (len > 3) {
        // Convert code point string to integer.
        char *end;
        cp = std::strtoul(paths[3], &end, 16);

        // Verify cp value.
        try {
            CodePoint c(cp);
        } catch (const IllegalCodePoint& e) {
            response_exception(e);
            return 0;
        }

        // Get properties and insert to JSON object.
        json res;
        res["codepoint"] = seshat::code_point_to_string(cp);
        res["properties"] = {
            { "numeric", json::object() },
            { "string", json::object() },
            { "misc", json::object() },
            { "catalog", json::object() },
            { "enumerated", json::object() },
            { "binary", json::object() },
        };

        // Name
        try {
            res["properties"]["misc"]["name"] = unicode::name(cp);
        } catch (const unicode::NoName &e) {
            res["properties"]["misc"]["name"] = "<none>";
        }

        // ============================
        // Catalog
        // ============================
        //
        // Age
        auto age = unicode::age(cp);
        res["properties"]["catalog"]["age"] =
            (age == (unicode::Version { 0, 0, 0 })) ? "NA" :
                std::to_string(age.major) + "." + std::to_string(age.minor);

        // Block
        auto block_value = unicode::property_value_name(
                unicode::block(cp));
        res["properties"]["catalog"]["block"] = {
            { "full", block_value.full },
            { "abbr", block_value.abbr },
        };
        // Script
        auto script_value = unicode::property_value_name(
                unicode::script(cp));
        res["properties"]["catalog"]["script"] = {
            { "full", script_value.full },
            { "abbr", script_value.abbr },
        };

        // ============================
        // Enumerated
        // ============================
        //
        res["properties"]["enumerated"] = {
        // Canonical_Combining_Class
            { "canonical_combining_class", unicode::ccc(cp) },
        };
        // Decomposition_Type
        auto dt_value = unicode::property_value_name(
                unicode::dt(cp));
        res["properties"]["enumerated"]["decomposition_type"] = {
            { "full", dt_value.full },
            { "abbr", dt_value.abbr },
        };
        // General_Category
        auto gc_value = unicode::property_value_name(
                unicode::gc(cp));
        res["properties"]["enumerated"]["general_category"] = {
            { "full", gc_value.full },
            { "abbr", gc_value.abbr },
        };
        // Grapheme_Cluster_Break
        auto gcb_value = unicode::property_value_name(
                unicode::gcb(cp));
        res["properties"]["enumerated"]["grapheme_cluster_break"] = {
            { "full", gcb_value.full },
            { "abbr", gcb_value.abbr },
        };
        // Hangul_Syllable_Type

        // ============================
        // Binary
        // ============================
        //
        res["properties"]["binary"] = {
        // ASCII_Hex_Digit
            { "ascii_hex_digit", unicode::ascii_hex_digit(cp) },
        // Alphabetic
        // Bidi_Control
            { "bidi_control", unicode::bidi_control(cp) },
        // Bidi_Mirrored
        // Cased
            { "cased", unicode::cased(cp) },
        // Composition_Exclusion
        // Case_Ignorable
            { "case_ignorable", unicode::case_ignorable(cp) },
        // Full_Composition_Exclusion
            { "full_composition_exclusion", unicode::comp_ex(cp) },
        // Changes_When_Casefolded
        // Changes_When_Casemapped
        // Changes_When_NFKC_Casefolded
        // Changes_When_Lowercased
        // Changes_When_Titlecased
        // Changes_When_Uppercased
        // Dash
            { "dash", unicode::dash(cp) },
        // Deprecated
            { "deprecated", unicode::deprecated(cp) },
        // Default_Ignorable_Code_Point
            {
                "default_ignorable_code_point", 
                unicode::default_ignorable_code_point(cp)
            },
        // Diacritic
            { "diacritic", unicode::diacritic(cp) },
        // Extender
            { "extender", unicode::extender(cp) },
        // Grapheme_Base
        // Grapheme_Extend
            { "grapheme_extend", unicode::grapheme_extend(cp) },
        // Grapheme_Link
        // Hex_Digit
            { "hex_digit", unicode::hex_digit(cp) },
        // Hyphen
            { "hyphen", unicode::hyphen(cp) },
        // ID_Continue
        // Ideographic
            { "ideographic", unicode::ideographic(cp) },
        // ID_Start
        // IDS_Binary_Operator
            { "ids_binary_operator", unicode::ids_binary_operator(cp) },
        // IDS_Trinary_Operator
            { "ids_trinary_operator", unicode::ids_trinary_operator(cp) },
        // Join_Control
            { "join_control", unicode::join_control(cp) },
        // Logical_Order_Exception
            { "logical_order_exception", unicode::logical_order_exception(cp) },
        // Lowercase
            { "lowercase", unicode::lowercase(cp) },
        // Math
            // { "math", unicode::math(cp) },
        // Noncharacter_Code_Point
            { "noncharacter_code_point", unicode::nchar(cp) },
        // Other_Alphabetic
            { "other_alphabetic", unicode::oalpha(cp) },
        // Other_Default_Ignorable_Code_Point
            { "other_default_ignorable_code_point", unicode::odi(cp) },
        // Other_Grapheme_Extend
            { "other_grapheme_extend", unicode::ogr_ext(cp) },
        // Other_ID_Continue
            { "other_id_continue", unicode::oidc(cp) },
        // Other_ID_Start
            { "other_id_start", unicode::oids(cp) },
        // Other_Lowercase
            { "other_lowercase", unicode::other_lowercase(cp) },
        // Other_Math
            { "other_math", unicode::other_math(cp) },
        // Other_Uppercase
            { "other_uppercase", unicode::other_uppercase(cp) },
        // Pattern_Syntax
            { "pattern_syntax", unicode::pattern_syntax(cp) },
        // Pattern_White_Space
            { "pattern_white_space", unicode::pattern_white_space(cp) },
        // Prepended_Concatenation_Mark
            {
                "prepended_concatenation_mark",
                unicode::prepended_concatenation_mark(cp)
            },
        // Quotation_Mark
            { "quotation_mark", unicode::quotation_mark(cp) },
        // Regional_Indicator
            { "regional_indicator", unicode::regional_indicator(cp) },
        // Radical
            { "radical", unicode::radical(cp) },
        // Soft_Dotted
            { "soft_dotted", unicode::soft_dotted(cp) },
        // Sentence_Terminal
            { "sentence_terminal", unicode::sentence_terminal(cp) },
        // Terminal_Punctuation
            { "terminal_punctuation", unicode::terminal_punctuation(cp) },
        // Unified_Ideograph
            { "unified_ideograph", unicode::unified_ideograph(cp) },
        // Uppercase
            { "uppercase", unicode::uppercase(cp) },
        // Variation_Selector
            { "variation_selector", unicode::variation_selector(cp) },
        // White_Space
            { "white_space", unicode::white_space(cp) },
        // XID_Continue
        // XID_Start
        // Expands_On_NFC
        // Expands_On_NFD
        // Expands_On_NFKC
        // Expands_On_NFKD
        };

        // Send response.
        std::cout << "Content-type: application/json\n";
        std::cout << "Access-Control-Allow-Origin: *\n" << std::endl;
        std::cout << res.dump();

        return 0;
    }
    return 1; // Error
}

int segmentation(char **paths, int len)
{
    // '/{version}/unicode/segmentation/{type}/{text}'
    // minimum len: 5
    if (len > API_PATH_SEGMENTATION_TEXT_POS) {
        // Get text from request.
        String str;
        try {
            // FIXME: Replace below code as commented after libseshat updated.
            // NOTE: operator=(const char*) not implemented!
            //
            // str = paths[API_PATH_SEGMENTATION_TEXT_POS];
            str = String(paths[API_PATH_SEGMENTATION_TEXT_POS]);
        } catch (const std::exception& e) {
            response_exception(e);
            return 0;
        }
        // Decode URL encoded string.
        std::string tmp = str.to_utf8();
        const char *last = uriUnescapeInPlaceA(
                const_cast<char*>(tmp.c_str()));
        *(const_cast<char*>(last)) = '\0'; // Replace '0' to 'null'
        String text = tmp.c_str();

        // ========================
        // Grapheme segmentation
        // ========================
        if (std::string(API_PATH_GRAPHEME) ==
                paths[API_PATH_SEGMENTATION_TYPE_POS]) {
            // Create JSON object.
            json res;
            res["text"] = text.to_utf8();
            res["type"] = "grapheme";
            res["breaks"] = json::array();
            // Get segments.
            for (auto&& ch: text) {
                res["breaks"].push_back(json::array());
                for (auto&& cp: ch.sequence()) {
                    size_t last = res["breaks"].size() - 1;
                    res["breaks"][last].push_back({
                        { "codepoint", cp.to_string() },
                        { "character", Character(cp).to_utf8() }
                    });
                }
            }
            // Send response.
            std::cout << "Content-type: application/json\n";
            std::cout << "Access-Control-Allow-Origin: *\n" << std::endl;
            std::cout << res.dump();

            return 0;
        }
    }
    return 1; // Error
}

} // namesapce seshat_rest
