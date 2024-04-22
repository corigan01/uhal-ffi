#pragma once

#include "uhal/ConnectionManager.hpp"
#include <memory>
#include <uhal/uhal.hpp>
#include "HwInterface.hpp"

class ConnectionManager {
private:
  uhal::ConnectionManager m_real;

public:
  ConnectionManager(std::string path);

  std::unique_ptr<HwInterface> get_device(std::string device_name);  
};

std::unique_ptr<ConnectionManager> new_connection_manager(std::string path);
