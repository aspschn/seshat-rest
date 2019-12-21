/*
//  properties.cpp
//
//  Author:     hardboiled65
//  Created:    2018. 02. 18. 04:21
//  Copyright (c) 2018 hardboiled65. All rights reserved.
//
//
*/
#include <cstdlib>
#include <cstdint>
#include <iostream>
#include <string>

#include <libyuarel/yuarel.h>

#include <nlohmann/json.hpp>

#include <seshat/codepoint.h>
#include <seshat/unicode/casing.h>
#include <seshat/unicode/name.h>
#include <seshat/unicode/normalization_props.h>
#include <seshat/unicode/properties.h>
#include <seshat/unicode/version.h>
#include <seshat/utils.h>

#include "property_name.h"

#define QUERY_PARAM_MAX 16

using namespace seshat;
using nlohmann::json;

int main()
{
    uint32_t cp;
    const char *got;
    const char *env = getenv("QUERY_STRING");

    std::string query = (env) ? env : "";

    // Parse query string.
    struct yuarel_param params[QUERY_PARAM_MAX] = {0};
    int count = yuarel_parse_query(
        const_cast<char*>(query.c_str()), '&', params, QUERY_PARAM_MAX);
    if (count == -1) return 1;
    // Find _cp value.
    unsigned int idx_cp;
    for (idx_cp = 0; idx_cp < count; ++idx_cp) {
        if (std::string("_cp") == params[idx_cp].key) break;
    }
    if (idx_cp >= count) {
        return 1;
    }

    // Convert code point string to integer.
    char *end;
    cp = std::strtoul(params[idx_cp].val, &end, 16);

    // Verify cp value.
    try {
        CodePoint c(cp);
    } catch (const IllegalCodePoint& e) {
        json json_except;
        json_except["exception"] = std::string(e.what());
        std::cout << "Content-type: application/json\n";
        std::cout << "Access-Control-Allow-Origin: *\n" << std::endl;
        std::cout << json_except.dump();
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
        { "enum", json::object() },
        { "binary", json::object() },
    };

    // Name
    try {
        res["properties"]["misc"]["name"] = unicode::name(cp);
    } catch (const unicode::NoName &e) {
        res["properties"]["misc"]["name"] = "<none>";
    }

    // Age
    auto age = unicode::age(cp);
    res["properties"]["catalog"]["age"] =
        (age == (unicode::Version { 0, 0, 0 })) ? "NA" :
            std::to_string(age.major) + "." + std::to_string(age.minor);

    // Block
    // Script

    // ============================
    // Enumerated
    // ============================
    //
    // ----- deprecated ------
    // 'enum' is Deprecated! Use 'enumerated' instead.
    res["properties"]["enum"] = {
    // Canonical_Combining_Class
        { "canonical_combining_class", unicode::ccc(cp) },
    // Decomposition_Type
        {
            "decomposition_type", 
            dt_name_full_table[static_cast<uint8_t>(unicode::dt(cp))]
        },
    };
    // ----- end deprecated -----
    res["properties"]["enumerated"] = {
    // Canonical_Combining_Class
        { "canonical_combining_class", unicode::ccc(cp) },
    // Decomposition_Type
        {
            "decomposition_type", 
            dt_name_full_table[static_cast<uint8_t>(unicode::dt(cp))]
        },
    };
    // General_Category
    // Grapheme_Cluster_Break
    // Hangul_Syllable_Type

    // ============================
    // Binary
    // ============================
    //
    res["properties"]["binary"] = {
    // Cased
        { "cased", unicode::cased(cp) },
    // Case_Ignorable
        { "case_ignorable", unicode::case_ignorable(cp) },
    // Full_Composition_Exclusion
        { "full_composition_exclusion", unicode::comp_ex(cp) },
    // Default_Ignorable_Code_Point
        {
            "default_ignorable_code_point", 
            unicode::default_ignorable_code_point(cp)
        },
    // Grapheme_Extend
        { "grapheme_extend", unicode::grapheme_extend(cp) },
    // Lowercase
        { "lowercase", unicode::lowercase(cp) },
    // Prepended_Concatenation_Mark
        {
            "prepended_concatenation_mark",
            unicode::prepended_concatenation_mark(cp)
        },
    // Uppercase
        { "uppercase", unicode::uppercase(cp) },
    // Variation_Selector
        { "variation_selector", unicode::variation_selector(cp) },
    // White_Space
        { "white_space", unicode::white_space(cp) },
    };

    std::cout << "Content-type: application/json\n";
    std::cout << "Access-Control-Allow-Origin: *\n" << std::endl;
    std::cout << res.dump();

    return 0;
}
