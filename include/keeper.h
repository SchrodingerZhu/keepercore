//
// Created by schrodinger on 2/5/20.
//

#ifndef KEEPERCORE_INCLUDE_H
#define KEEPERCORE_INCLUDE_H
#include <cstddef>
typedef struct {
  size_t length;
  size_t capacity;
  const char** vector;
} result_t;

extern "C" result_t fetch_list(const char* password);
extern "C" result_t fetch_password(const char* name, const char* password);
extern "C" result_t add_password(const char* name, const char* content, const char* password);
extern "C" result_t delete_password(const char* name, const char* password);
extern "C" void clean_result(result_t res);
extern "C" result_t botan_version();
extern "C" result_t get_server();
extern "C" result_t generate_password(const char* name, const char* password);
#endif // KEEPERCORE_INCLUDE_INCLUDE_H
