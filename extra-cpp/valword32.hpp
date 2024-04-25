#pragma once
#include "uhal/ValMem.hpp"
#include <uhal/uhal.hpp>
#include <stdint.h>


inline uint32_t value(const uhal::ValWord<uint32_t>& word) {
  return word.value();
}

inline bool ready(uhal::ValWord<uint32_t>& word) {
  return word.valid();
}

