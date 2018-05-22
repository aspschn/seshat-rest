/*
//  paths.h
//
//  Author:     <OWNER>
//  Created:    2018. 03. 11. 09:18
//  Copyright (c) 2016 <OWNER>. All rights reserved.
//
//
*/
#ifndef _SESHAT_REST_PATHS_H
#define _SESHAT_REST_PATHS_H

// '/{version}' e.g: '/api'
#define API_PATH_API_VERSION_POS    0
#define API_PATH_API_VERSION        "api"
// '/api/unicode'
#define API_PATH_UNICODE_POS        1
#define API_PATH_UNICODE            "unicode"
// '/api/unicode/properties'
#define API_PATH_PROPERTIES_POS     2
#define API_PATH_PROPERTIES         "properties"
// '/api/unicode/segmentation'
#define API_PATH_SEGMENTATION_POS   2
#define API_PATH_SEGMENTATION       "segmentation"
// '/api/unicode/segmentation/{type}'
#define API_PATH_SEGMENTATION_TYPE_POS  3
// '/api/unicode/segmentation/grapheme'
#define API_PATH_GRAPHEME_POS       3
#define API_PATH_GRAPHEME           "grapheme"
// '/api/unicode/segmentation/{type}/{text}'
#define API_PATH_SEGMENTATION_TEXT_POS  4
#define API_PATH_GRAPHEME_TEXT_POS      4

#define MAX_PATH_LENGTH         16

#endif /* _SESHAT_REST_PATHS_H */
