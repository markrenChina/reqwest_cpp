#include "wrappers.hpp"
#include <cassert>

extern "C" {
void* request_create(const char*);
void request_destroy(void *);
void* request_send(void *);

void response_destroy(void *);
int response_body_length(void *);
int response_body(void *, char *, int);
}

Request::Request(const std::string url) {
  raw = request_create(url.c_str());
  if( raw == nullptr) {
    throw "Invalid URL";
  }
}

Request::~Request() { request_destroy(raw); }





std::vector<char> Response::read_body() {
  int length = response_body_length(raw);
  if (length < 0) {
    throw "Response body's length was less than zero";
  }

  std::vector<char> buffer(length);

  int bytes_written = response_body(raw,buffer.data(),buffer.size());
  if (bytes_written != length) {
    throw "Response body was a different size than what we expected";
  }

  return buffer;
}

Response Request::send(){
  void* raw_response = request_send(raw);

  if(raw_response == nullptr) {
    throw "Request failed";
  }

  return Response(raw_response);
}
