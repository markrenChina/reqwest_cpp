#pragma once

#include <cstdint>
#include <string>
#include <vector>
#include <string>
#include "client.hpp"

namespace ffi{

std::string last_error_message();

class WrapperException: std::exception {
public:
  WrapperException(const std::string& msg) : msg(msg) {};
  static WrapperException Last_error();

  const char* what() const throw() {
    return msg.c_str();
  }
private:
  std::string msg;
};



struct ClientBuilder {

  static ClientBuilder* New();
  ClientBuilder* user_agent(const std::string value);
  ClientBuilder* default_headers(HeaderMap* headerMap);
  ClientBuilder* redirect(size_t max);
  ClientBuilder* referer(bool enable);
  ClientBuilder* proxy(Proxy* proxy);
  ClientBuilder* timeout(uint64_t* millisecond);
  ClientBuilder* timeout(uint64_t millisecond);
  ClientBuilder* pool_idle_timeout(uint64_t* millisecond);
  ClientBuilder* pool_idle_timeout(uint64_t millisecond);

  Client* build();
  void destory(ClientBuilder* cb);
  ~ClientBuilder();
};

struct Client {
  RequestBuilder* get(const std::string& url);
  
  ~Client();
};

struct RequestBuilder {
  RequestBuilder* header(const std::string& key, const std::string& value);
  Response* sendRequest();
  ~RequestBuilder();
};

struct Response {
  std::string text();
  ~Response();
};

struct HeaderMap {
  int32_t insert(const std::string& key, const std::string& value);
  ~HeaderMap();
};

namespace  proxy {
  Proxy* http(const std::string& proxy_scheme);
  Proxy* https(const std::string& proxy_scheme);
  Proxy* all(const std::string& proxy_scheme);
};

}




