#include <nlohmann/json.hpp>
#include <yuarel.h>

#include <iostream>
#include <string>

int main()
{
    std::string // char
    query_string
    // [64]
    // = "name=smith&time=18";
    = "name=%EA%B8%B8%EB%8F%99&time=18";

    struct yuarel_param params[10] = {0};

    int count = yuarel_parse_query(
        const_cast<char*>(query_string.c_str()), '&', params, 10);

    for (int i = 0; i < count; ++i) {
        std::cout << params[i].key << std::endl;
        std::cout << params[i].val << std::endl;
    }
    std::cout << query_string << std::endl;

    return 0;
}
