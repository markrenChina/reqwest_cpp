#include "wrappers.hpp"
#include "client.hpp"
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <memory>
#include <string>
#include <sys/types.h>
#include <vector>
#include "client.hpp"

namespace ffi {

#define IF_NULL_THROW(value)  value ? value : throw WrapperException::Last_error();

#define RETURN_SELF_NULL_THROW(value) \
auto _res = value; \
return IF_NULL_THROW(_res);

#define RETURN_STRING_AND_FREE(fun) \
const char * _v = fun;  \
if(_v) {std::string _res(_v);  \
ffi::free_string(_v); \
return _res;} else return "";



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

Bytes::~Bytes() {
  ffi::free_vec_u8(m_data,m_len);
}

ClientBuilder* ClientBuilder::New() {
  RETURN_SELF_NULL_THROW(ffi::new_client_builder())
}

ClientBuilder* ClientBuilder::add_root_certificate(const std::string& cert_path){
  RETURN_SELF_NULL_THROW(ffi::client_builder_add_root_certificate(this,cert_path.c_str()));
}

ClientBuilder* ClientBuilder::danger_accept_invalid_certs(bool accept_invalid_certs){
  RETURN_SELF_NULL_THROW(ffi::client_builder_danger_accept_invalid_certs(this,accept_invalid_certs))
}

ClientBuilder* ClientBuilder::default_headers(HeaderMap* headerMap){
  RETURN_SELF_NULL_THROW(ffi::client_builder_default_headers(this,headerMap))
}

ClientBuilder* ClientBuilder::default_headers(std::initializer_list<Pair> headers){
  HeaderMap* headerMap = HeaderMap::New();
  for (auto &item : headers){
    headerMap->insert(item.key,item.value);
  }
  RETURN_SELF_NULL_THROW(ffi::client_builder_default_headers(this,headerMap))
}

ClientBuilder* ClientBuilder::http09_responses(){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http09_responses(this))
}

ClientBuilder* ClientBuilder::http1_allow_obsolete_multiline_headers_in_responses(bool val){
  RETURN_SELF_NULL_THROW(
      ffi::client_builder_http1_allow_obsolete_multiline_headers_in_responses(this,val))
}

ClientBuilder* ClientBuilder::http1_only(){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http1_only(this))
}

ClientBuilder* ClientBuilder::http1_title_case_headers(){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http1_title_case_headers(this))
}

ClientBuilder* ClientBuilder::http2_adaptive_window(bool enable){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http2_adaptive_window(this, enable))
}

ClientBuilder* ClientBuilder::http2_initial_connection_window_size(uint32_t *size){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http2_initial_connection_window_size(this, size))
}
ClientBuilder* ClientBuilder::http2_initial_connection_window_size(uint32_t size){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http2_initial_connection_window_size(this, &size))
}

ClientBuilder* ClientBuilder::http2_initial_stream_window_size(uint32_t *size){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http2_initial_stream_window_size(this, size))
}
ClientBuilder* ClientBuilder::http2_initial_stream_window_size(uint32_t size){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http2_initial_stream_window_size(this, &size))
}

ClientBuilder* ClientBuilder::http2_max_frame_size(uint32_t *size){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http2_max_frame_size(this, size))
}
ClientBuilder* ClientBuilder::http2_max_frame_size(uint32_t size){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http2_max_frame_size(this, &size))
}

ClientBuilder* ClientBuilder::http2_prior_knowledge(){
  RETURN_SELF_NULL_THROW(ffi::client_builder_http2_prior_knowledge(this))
}

ClientBuilder* ClientBuilder::https_only(bool enable){
  RETURN_SELF_NULL_THROW(ffi::client_builder_https_only(this,enable))
}

ClientBuilder* ClientBuilder::local_address(const std::string& local_address){
  RETURN_SELF_NULL_THROW(ffi::client_builder_local_address(this,local_address.c_str()))
}

ClientBuilder* ClientBuilder::max_tls_version(const std::string& version){
  RETURN_SELF_NULL_THROW(ffi::client_builder_max_tls_version(this,version.c_str()))
}

ClientBuilder* ClientBuilder::min_tls_version(const std::string& version){
  RETURN_SELF_NULL_THROW(ffi::client_builder_min_tls_version(this,version.c_str()))
}

ClientBuilder* ClientBuilder::no_trust_dns(){
  RETURN_SELF_NULL_THROW(ffi::client_builder_no_trust_dns(this))
}

ClientBuilder* ClientBuilder::pool_idle_timeout(const uint64_t* millisecond){
  RETURN_SELF_NULL_THROW(ffi::client_builder_pool_idle_timeout(this,millisecond))
}
ClientBuilder* ClientBuilder::pool_idle_timeout(const uint64_t millisecond){
  RETURN_SELF_NULL_THROW(ffi::client_builder_pool_idle_timeout(this,&millisecond))
}

ClientBuilder* ClientBuilder::pool_max_idle_per_host(uintptr_t max) {
  RETURN_SELF_NULL_THROW(ffi::client_builder_pool_max_idle_per_host(this,max))
}

ClientBuilder* ClientBuilder::proxy(Proxy *proxy){
  RETURN_SELF_NULL_THROW(ffi::client_builder_proxy(this,proxy))
}

ClientBuilder* ClientBuilder::redirect(size_t max){
  RETURN_SELF_NULL_THROW(ffi::client_builder_redirect(this,max))
}

ClientBuilder* ClientBuilder::referer(bool enable){
  RETURN_SELF_NULL_THROW(ffi::client_builder_referer(this,enable))
}

ClientBuilder* ClientBuilder::resolve(const std::string& domain,const std::string& socket_addr){
  RETURN_SELF_NULL_THROW(ffi::client_builder_resolve(this,domain.c_str(),socket_addr.c_str()))
}

ClientBuilder* ClientBuilder::resolve_to_addrs(const std::string& domain,
                                std::vector<const char *>& socket_addr_array){
  RETURN_SELF_NULL_THROW(ffi::client_builder_resolve_to_addrs(
      this,domain.c_str(),&socket_addr_array[0],socket_addr_array.size()))
}
ClientBuilder* ClientBuilder::resolve_to_addrs(const std::string& domain,
                                std::initializer_list<const char *>& socket_addr_array){
  std::vector<const char *> tmp(socket_addr_array);
  return this->resolve_to_addrs(domain,tmp);
}


ClientBuilder* ClientBuilder::tcp_keepalive(const uint64_t *millisecond){
  RETURN_SELF_NULL_THROW(ffi::client_builder_tcp_keepalive(this,millisecond))
}
ClientBuilder* ClientBuilder::tcp_keepalive(const uint64_t millisecond){
  RETURN_SELF_NULL_THROW(ffi::client_builder_tcp_keepalive(this,&millisecond))
}

ClientBuilder* ClientBuilder::tcp_nodelay(bool enable){
  RETURN_SELF_NULL_THROW(ffi::client_builder_tcp_nodelay(this,enable))
}

ClientBuilder* ClientBuilder::timeout(const uint64_t* millisecond){
  RETURN_SELF_NULL_THROW(ffi::client_builder_timeout(this,millisecond))
}

ClientBuilder* ClientBuilder::timeout(const uint64_t millisecond){
  RETURN_SELF_NULL_THROW(ffi::client_builder_timeout(this,&millisecond))
}

ClientBuilder* ClientBuilder::tls_built_in_root_certs(bool tls_built_in_root_certs){
  RETURN_SELF_NULL_THROW(ffi::client_builder_tls_built_in_root_certs(this,tls_built_in_root_certs))
}

ClientBuilder* ClientBuilder::tls_sni(bool tls_sni){
  RETURN_SELF_NULL_THROW(ffi::client_builder_tls_sni(this,tls_sni))
}

ClientBuilder* ClientBuilder::user_agent(const std::string& value){
//  std::cout << "addr in cpp to rust:" << std::hex << (void *)this << std::endl;
  RETURN_SELF_NULL_THROW(ffi::client_builder_user_agent(this,value.c_str()))
//  auto _res = ffi::client_builder_user_agent(this,value.c_str());
//  std::cout << "addr in cpp out rust:" << std::hex << (void *)_res << std::endl;
//  return IF_NULL_THROW(_res);
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

RequestBuilder* Client::delete_(const std::string &url){
  RETURN_SELF_NULL_THROW(ffi::client_delete(this,url.c_str()))
}

RequestBuilder* Client::head(const std::string &url){
  RETURN_SELF_NULL_THROW(ffi::client_head(this,url.c_str()))
}

RequestBuilder* Client::patch(const std::string &url){
  RETURN_SELF_NULL_THROW(ffi::client_patch(this,url.c_str()))
}

RequestBuilder* Client::post(const std::string &url){
  RETURN_SELF_NULL_THROW(ffi::client_post(this,url.c_str()))
}

RequestBuilder* Client::request(const std::string& method,const std::string &url){
  RETURN_SELF_NULL_THROW(ffi::client_request(this,method.c_str(),url.c_str()))
}

Response* Client::client_execute(Request *request){
  RETURN_SELF_NULL_THROW(ffi::client_execute(this,request))
}

RequestBuilder* RequestBuilder::basic_auth(
    const std::string& username, const std::string& password){
  RETURN_SELF_NULL_THROW(ffi::request_builder_basic_auth(this,username.c_str(),password.c_str()))
}
RequestBuilder* RequestBuilder::bearer_auth(const std::string& token){
  RETURN_SELF_NULL_THROW(ffi::request_builder_bearer_auth(this,token.c_str()))
}
RequestBuilder* RequestBuilder::body(const std::vector<uint8_t>& bytes){
  RETURN_SELF_NULL_THROW(ffi::request_builder_body_bytes(this,&bytes[0],bytes.size()))
}
RequestBuilder* RequestBuilder::file_body(const std::string& file_path){
  RETURN_SELF_NULL_THROW(ffi::request_builder_body_file(this,file_path.c_str()))
}
RequestBuilder* RequestBuilder::body(const std::string& str){
  RETURN_SELF_NULL_THROW(ffi::request_builder_body_string(this,str.c_str()))
}

RequestBuilder* RequestBuilder::form(const std::vector<Pair>& pairs){
  RETURN_SELF_NULL_THROW(ffi::request_builder_form(this,&pairs[0],pairs.size()))
}
RequestBuilder* RequestBuilder::form(const std::initializer_list<Pair>& pairs){
  std::vector<Pair> tmp(pairs);
  return this->form(tmp);
}

RequestBuilder* RequestBuilder::header(
    const std::string& key, const std::string& value){
  RETURN_SELF_NULL_THROW(request_builder_header(this,key.c_str(),value.c_str()))
}

RequestBuilder* RequestBuilder::headers(HeaderMap* headers){
  RETURN_SELF_NULL_THROW(request_builder_headers(this,headers))
}

RequestBuilder* RequestBuilder::json(const std::vector<Pair> &pairs){
  RETURN_SELF_NULL_THROW(ffi::request_builder_json(this,&pairs[0], pairs.size()))
}
RequestBuilder* RequestBuilder::json(const std::initializer_list<Pair>& pairs){
  std::vector<Pair> tmp(pairs);
  return this->json(tmp);
}
//todo
RequestBuilder* RequestBuilder::json(const std::string& json){
  RETURN_SELF_NULL_THROW(this->header("content-type","application/json")->body(json));
}

RequestBuilder* RequestBuilder::query(const std::vector<Pair>& querys){
  RETURN_SELF_NULL_THROW(ffi::request_builder_query(this,&querys[0], querys.size()))
}
RequestBuilder* RequestBuilder::query(const std::initializer_list<Pair>& querys){
  std::vector<Pair> tmp(querys);
  return this->query(tmp);
}

Response* RequestBuilder::send(){
  RETURN_SELF_NULL_THROW(ffi::request_builder_send(this))
}

RequestBuilder* RequestBuilder::timeout(uint64_t millisecond){
  RETURN_SELF_NULL_THROW(ffi::request_builder_timeout(this,millisecond))
}

RequestBuilder* RequestBuilder::try_clone(){
  RETURN_SELF_NULL_THROW(ffi::request_builder_try_clone(this))
}

RequestBuilder* RequestBuilder::version(const std::string& version){
  RETURN_SELF_NULL_THROW(ffi::request_builder_version(this,version.c_str()))
}

std::string Response::text_and_destory(){
  RETURN_STRING_AND_FREE(ffi::response_text(this))
}

std::string Response::text_with_charset_and_destory(
    const std::string& default_encoding){
  RETURN_STRING_AND_FREE(ffi::response_text_with_charset(this,default_encoding.c_str()))
}

Bytes::ptr Response::bytes_and_destory() {
  const uint64_t * len = ffi::response_content_length(this);
  if(!len){
    throw WrapperException::Last_error();
  }
  const uint8_t * ptr = ffi::response_bytes(this);
  return std::make_shared<Bytes>(ptr,*len);
}

uint64_t Response::content_length(){
  const uint64_t * len = ffi::response_content_length(this);
  if(!len){
    throw WrapperException::Last_error();
  }
  return *len;
}

Bytes::ptr Response::copy_to() {
  const uint64_t * len = ffi::response_content_length(this);
  if(!len){
    throw WrapperException::Last_error();
  }
  const uint8_t * ptr = ffi::response_copy_to(this);
  return std::make_shared<Bytes>(ptr,*len);
}

const HeaderMap* Response::headers(){
  const HeaderMap* hp = ffi::response_headers(this);
  return hp;
}

std::string Response::remote_addr(){
  RETURN_STRING_AND_FREE(ffi::response_remote_addr(this))
}

uint16_t Response::status(){
  return ffi::response_status(this);
}

std::string Response::url(){
  RETURN_STRING_AND_FREE(ffi::response_url(this))
}

std::string Response::version(){
  RETURN_STRING_AND_FREE(ffi::response_version(this))
}



HeaderMap* HeaderMap::New() {
  RETURN_SELF_NULL_THROW(ffi::new_header_map())
}

bool HeaderMap::insert(const std::string& key, const std::string& value){
  return ffi::header_map_insert(this,key.c_str(),value.c_str());
}

bool HeaderMap::append(const std::string& key, const std::string& value){
  return ffi::header_map_append(this,key.c_str(),value.c_str());
}

uintptr_t HeaderMap::capacity() const{
  return ffi::header_map_capacity(this);
}

void HeaderMap::clear(){
  return ffi::header_map_clear(this);
}

bool HeaderMap::contains_key(const std::string& key){
  bool bk;
  int res = ffi::header_map_contains_key(this,key.c_str(),&bk);
  if (res != 0){
    throw WrapperException::Last_error();
  }
  return bk;
}

std::string HeaderMap::get(const std::string& key) const{
  RETURN_STRING_AND_FREE(ffi::header_map_get(this,key.c_str()))
}

std::string HeaderMap::get_all (const std::string& key) const{
  RETURN_STRING_AND_FREE(ffi::header_map_get_all(this,key.c_str()))
}

std::string HeaderMap::keys() const{
  RETURN_STRING_AND_FREE(ffi::header_map_keys(this))
}

std::string HeaderMap::values() const{
  RETURN_STRING_AND_FREE(ffi::header_map_values(this))
}

uintptr_t HeaderMap::keys_len() const{
  return ffi::header_map_keys_len(this);
}

uintptr_t HeaderMap::len() const{
  return ffi::header_map_len(this);
}

bool HeaderMap::remove(std::string& key){
  return ffi::header_map_remove(this, key.c_str());
}

void HeaderMap::reserve(uintptr_t additional){
  return ffi::header_map_reserve(this, additional);
}

void HeaderMap::destory() const{
  ffi::header_map_destory(this);
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