#pragma once
#include "uhal/ValMem.hpp"
#include <uhal/uhal.hpp>
#include <stdint.h>
#include <vector>

typedef std::vector<uint32_t> vec32;
typedef uhal::ValVector<uint32_t> ValVector32;

inline uint32_t value(const uhal::ValWord<uint32_t>& word) {
  return word.value();
}

inline bool ready(uhal::ValWord<uint32_t>& word) {
  return word.valid();
}

inline vec32 value_valvec(void* vector) {
  return (*((uhal::ValVector<uint32_t>*)vector)).value();
}

inline bool ready_valvec(void* vector) {
  return (*((uhal::ValVector<uint32_t>*)vector)).valid();
}
