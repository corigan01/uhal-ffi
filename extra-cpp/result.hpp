#pragma once

#include "uhal/ValMem.hpp"
#include <cstdint>
#include <memory>
#include <string>
#include <uhal/uhal.hpp>

namespace uhal {
  typedef ValWord<uint32_t> ValWord32;
  
  inline std::unique_ptr<uhal::ConnectionManager> new_connection_manager_result(const std::string &string) {
    return std::unique_ptr<uhal::ConnectionManager>(new uhal::ConnectionManager(string));
  }

  inline std::unique_ptr<uhal::HwInterface> get_device_from_connection_manager_result(std::unique_ptr<uhal::ConnectionManager> &cm, const std::string &string) {
    return std::make_unique<uhal::HwInterface>(cm->getDevice(string));
  }
}
