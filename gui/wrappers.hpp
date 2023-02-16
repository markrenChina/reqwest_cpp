#include <string>
#include <vector>
#include <string>
#include "client.hpp"

namespace ffi{

struct ClientBuilder {
  ClientBuilder* user_agent(const std::string value);
  Client* build();
  ~ClientBuilder();
};

struct Client {
  RequestBuilder* get(const std::string url);
  ~Client();
};

struct RequestBuilder {
  RequestBuilder* header(const std::string key, const std::string value);

  Response* sendRequest();
  ~RequestBuilder();
};

struct Response {
  std::string text();
  ~Response();
};

}


//class Response {
//public:
//  Response(void* raw) : raw(raw) {}
//  ~Response();
//  std::vector<char> read_body();
//private:
//  void *raw;
//};
//
//class Request {
//public:
//  Request(const std::string);
//  ~Request();
//  Response send();
//
//private:
//  void* raw;
//};


