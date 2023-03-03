#pragma once

#include "client.hpp"

namespace ffi{

struct RString {
  const char *c_str;

  RString(const char *const& cStr)
      : c_str(cStr)
  {}
  ~RString(){ free_string(c_str); }

    std::string toStdString() const{ return c_str ? std::string(c_str) : ""; }
};
}