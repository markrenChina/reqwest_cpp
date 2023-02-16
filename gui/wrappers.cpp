#include "wrappers.hpp"
#include <cassert>

namespace ffi {

ClientBuilder* ClientBuilder::user_agent(const std::string value) {
  return ffi::user_agent(this,value.c_str());
}

Client* ClientBuilder::build() {
  return ffi::buildClient(this);
}

RequestBuilder* Client::get(const std::string url){
  return ffi::get(this,url.c_str());
}

RequestBuilder* RequestBuilder::header(
    const std::string key, const std::string value){
  return ffi::header(this,key.c_str(),value.c_str());
}

Response* RequestBuilder::sendRequest(){
  return ffi::send_request(this);
}

std::string Response::text(){
  return ffi::text(this);
}


}