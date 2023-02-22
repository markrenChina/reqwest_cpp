#include "wrappers.hpp"
#include "client.hpp"
#include <cassert>
#include <iostream>

namespace ffi {

#define IF_NULL_THROW(value)  value ? value : throw WrapperException::Last_error();

#define RETURN_SELF_NULL_THROW(value) \
auto _res = value; \
return IF_NULL_THROW(_res);


std::string last_error_message() {
  int error_length = ffi::last_error_length();

  if (error_length == 0){
    return {};
  }

  std::string msg(error_length, '\0');
  int ret = ffi::last_error_message(&msg[0], msg.length());
  if (ret <= 0) {
    throw new WrapperException("Fetching error message failed");
  }
  return msg;
}

WrapperException WrapperException::Last_error() {
  std::string msg = last_error_message();

  if (msg.length() == 0){
    return WrapperException("(no err available)");
  } else {
    return WrapperException(msg);
  }
}

ClientBuilder* ClientBuilder::New() {
  RETURN_SELF_NULL_THROW(ffi::new_client_builder())
}

ClientBuilder* ClientBuilder::user_agent(const std::string value){
  RETURN_SELF_NULL_THROW(ffi::client_builder_user_agent(this,value.c_str()))
}

ClientBuilder* ClientBuilder::default_headers(HeaderMap* headerMap){
  RETURN_SELF_NULL_THROW(ffi::client_builder_default_headers(this,headerMap))
}

ClientBuilder* ClientBuilder::redirect(size_t max){
  RETURN_SELF_NULL_THROW(ffi::client_builder_redirect(this,max))
}

ClientBuilder* ClientBuilder::referer(bool enable){
  RETURN_SELF_NULL_THROW(ffi::client_builder_referer(this,enable))
}

ClientBuilder* ClientBuilder::proxy(Proxy *proxy){
  RETURN_SELF_NULL_THROW(ffi::client_builder_proxy(this,proxy))
}

ClientBuilder* ClientBuilder::timeout(uint64_t* millisecond){
  RETURN_SELF_NULL_THROW(ffi::client_builder_timeout(this,millisecond))
}

ClientBuilder* ClientBuilder::timeout(uint64_t millisecond){
  RETURN_SELF_NULL_THROW(ffi::client_builder_timeout(this,&millisecond))
}

ClientBuilder* ClientBuilder::pool_idle_timeout(uint64_t* millisecond){
  RETURN_SELF_NULL_THROW(ffi::client_builder_timeout(this,millisecond))
}

ClientBuilder* ClientBuilder::pool_idle_timeout(uint64_t millisecond){
  RETURN_SELF_NULL_THROW(ffi::client_builder_timeout(this,&millisecond))
}

RequestBuilder* RequestBuilder::header(
    const std::string& key, const std::string& value){

  RETURN_SELF_NULL_THROW(request_builder_header(this,key.c_str(),value.c_str()))
}

Client* ClientBuilder::build() {
  RETURN_SELF_NULL_THROW(client_builder_build_client(this))
}

void ClientBuilder::destory(ClientBuilder *cb) {
  client_builder_destory(cb);
}

RequestBuilder* Client::get(const std::string& url){
  RETURN_SELF_NULL_THROW(client_get(this,url.c_str()))
}

Response* RequestBuilder::sendRequest(){
  RETURN_SELF_NULL_THROW(request_builder_send(this))
}

std::string Response::text(){
  char * text = ffi::response_text(this);
  std::string res(text);
  ffi::free_string(text);
  return res;
}

int32_t HeaderMap::insert(const std::string& key, const std::string& value){
  return ffi::header_map_insert(this,key.c_str(),value.c_str());
}

Proxy* proxy::http(const std::string& proxy_scheme){
  RETURN_SELF_NULL_THROW(proxy_reqwest_http(proxy_scheme.c_str()));
}
Proxy* proxy::https(const std::string& proxy_scheme){
  RETURN_SELF_NULL_THROW(proxy_reqwest_https(proxy_scheme.c_str()));
}
Proxy* proxy::all(const std::string& proxy_scheme){
  RETURN_SELF_NULL_THROW(proxy_reqwest_all(proxy_scheme.c_str()));
}

}