#pragma once

#include "uhal/ClientInterface.hpp"
#include "uhal/Node.hpp"
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

  inline const uhal::Node& get_node_from_node(const Node& node, const std::string& string) {
    return node.getNode(string);
  }

  
  inline std::unique_ptr<std::vector<uint32_t>> read_block_from_node(const uhal::Node& node, uint32_t size) {
    uhal::ValVector<uint32_t> valvec = node.readBlock(size);

    node.getClient().dispatch();

    return std::make_unique<std::vector<uint32_t>>(valvec.value());
  }

  inline void write_block_from_node(const uhal::Node& node, const uint32_t* array_ptr, uint32_t array_len) {
    std::vector<uint32_t> cxx_vector;
    for (const uint32_t* ptr = array_ptr; ptr < array_ptr + array_len; ptr++) {
      cxx_vector.push_back(*ptr);
    }

    node.writeBlock(cxx_vector);
    node.getClient().dispatch();
  }
}
