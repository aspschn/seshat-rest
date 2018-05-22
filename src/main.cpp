/*
//  main.cpp
//
//  Author:     hardboiled65
//  Created:    2018. 03. 10. 22:31
//  Copyright (c) 2018 hardboiled65. All rights reserved.
//
//
*/
#include <iostream>
#include <string>

#include <fcgio.h>

#include <libyuarel/yuarel.h>

#include "paths.h"
#include "unicode.h"

#define API_PATH_API_VERSION "api"
#define API_PATH_UNICODE    "unicode"
#define API_PATH_PROPERTIES "properties"
#define MAX_PATH_LENGTH     16

int main()
{
    std::streambuf *cin_streambuf = std::cin.rdbuf();
    std::streambuf *cout_streambuf = std::cout.rdbuf();
    std::streambuf *cerr_streambuf = std::cerr.rdbuf();

    FCGX_Request request;

    FCGX_Init();
    FCGX_InitRequest(&request, 0, 0);

    while (FCGX_Accept_r(&request) == 0) {
        fcgi_streambuf cin_fcgi_streambuf(request.in);
        fcgi_streambuf cout_fcgi_streambuf(request.out);
        fcgi_streambuf cerr_fcgi_streambuf(request.err);

        std::cin.rdbuf(&cin_fcgi_streambuf);
        std::cout.rdbuf(&cout_fcgi_streambuf);
        std::cerr.rdbuf(&cerr_fcgi_streambuf);

        char *env = FCGX_GetParam("REQUEST_URI", request.envp);

        std::string uri = (env) ? env : "";

        // Parse paths.
        char *path_parts[MAX_PATH_LENGTH];
        int parsed = yuarel_split_path(
            const_cast<char*>(uri.c_str()), path_parts, MAX_PATH_LENGTH);
        if (parsed == -1) {
            return 1;
        }

        // Call functions.
        if (parsed > 0 && std::string(path_parts[1]) == API_PATH_UNICODE) {
            int err = seshat_rest::unicode(path_parts, parsed);
        }
    }

    return 0;
}
