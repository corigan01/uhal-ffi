#pragma once

#include "uhal/HwInterface.hpp"
#include <memory>
#include <uhal/uhal.hpp>

class ConnectionManager;

class HwInterface {
private:
  uhal::HwInterface m_real;

public:
  HwInterface(std::unique_ptr<ConnectionManager> connection_manager, std::string device_name);

};

std::unique_ptr<HwInterface> new_hwinterface(std::unique_ptr<ConnectionManager> connection_manager, std::string device_name);
