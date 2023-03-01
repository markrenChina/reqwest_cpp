#pragma once

#include <cstdint>
#include <string>
#include <sys/types.h>
#include <vector>
#include <string>
#include <initializer_list>
#include <memory>

#include "ffi.hpp"

namespace ffi{

//using Bytes = std::shared_ptr<const uint8_t>;

struct Bytes {
  using ptr = std::shared_ptr<Bytes>;
  const uint8_t* m_data;
  const uint64_t m_len;
  Bytes(const uint8_t* data, const uint64_t len):m_data(data),m_len(len){};
  ~Bytes();
};

std::string last_error_message();

class WrapperException: std::exception {
public:
  WrapperException(const std::string& msg) : msg(msg) {};
  static WrapperException Last_error();

  const char* what() const throw() {
    return msg.c_str();
  }
private:
  std::string msg = "";
};

struct HeaderMap {

  using ptr_const = std::shared_ptr<const HeaderMap>;

  static HeaderMap* New();

  bool insert(const std::string& key, const std::string& value);

  /// Inserts a key-value pair into the map.
  ///
  /// If the map did not previously have this key present, then `false` is
  /// returned.
  ///
  /// If the map did have this key present, the new value is pushed to the end
  /// of the list of values currently associated with the key. The key is not
  /// updated, though; this matters for types that can be `==` without being
  /// identical.
  bool append(const std::string& key,const std::string& value);

  /// Returns the number of headers the map can hold without reallocating.
  ///
  /// This number is an approximation as certain usage patterns could cause
  /// additional allocations before the returned capacity is filled.
  uintptr_t capacity() const;

  /// Clears the map, removing all key-value pairs. Keeps the allocated memory
  /// for reuse.
  void clear();

  /// bk true if the map contains a value for the specified key.
  /// Return -1 if function failed.
  /// why not use bool? because bk is bool.
  /// If only return false,can't show function failed or isn't contains.
  bool contains_key(const std::string& key);

  std::string get(const std::string& key) const;

  /// Returns a view of all values associated with a key.
  ///
  /// The returned view does not incur any allocations and allows iterating
  /// the values associated with the key.  See [`GetAll`] for more details.
  /// Returns `None` if there are no values associated with the key.
  std::string get_all(const std::string& key) const;

  std::string keys() const;

  /// Returns the number of keys stored in the map.
  ///
  /// This number will be less than or equal to `len()` as each key may have
  /// more than one associated value.
  ///
  uintptr_t keys_len() const;

  /// Returns the number of headers stored in the map.
  ///
  /// This number represents the total number of **values** stored in the map.
  /// This number can be greater than or equal to the number of **keys**
  /// stored given that a single key may have more than one associated value.
  uintptr_t len() const;

  /// Removes a key from the map, returning the value associated with the key.
  ///
  /// Returns `None` if the map does not contain the key. If there are
  /// multiple values associated with the key, then the first one is returned.
  /// See `remove_entry_mult` on `OccupiedEntry` for an API that yields all
  /// values.
  bool remove(std::string& key);

  /// Reserves capacity for at least `additional` more headers to be inserted
  /// into the `HeaderMap`.
  ///
  /// The header map may reserve more space to avoid frequent reallocations.
  /// Like with `with_capacity`, this will be a "best effort" to avoid
  /// allocations until `additional` more headers are inserted. Certain usage
  /// patterns could cause additional allocations before the number is
  /// reached.
  void reserve(uintptr_t additional);

  std::string values() const;

  void destory() const;

private:
  HeaderMap()=delete;
  ~HeaderMap()=delete;
};


struct ClientBuilder {

  static ClientBuilder* New();

  /// Add a custom root certificate.
  ///
  /// This allows connecting to a server that has a self-signed
  /// certificate for example. This **does not** replace the existing
  /// trusted store.
  ClientBuilder* add_root_certificate(const std::string& cert_path);

  /// Controls the use of certificate validation.
  ///
  /// Defaults to `false`.
  ///
  /// # Warning
  ///
  /// You should think very carefully before using this method. If
  /// invalid certificates are trusted, *any* certificate for *any* site
  /// will be trusted for use. This includes expired certificates. This
  /// introduces significant vulnerabilities, and should only be used
  /// as a last resort.
  ClientBuilder* danger_accept_invalid_certs(bool accept_invalid_certs);

  /// Sets the default headers for every request.
  ClientBuilder* default_headers(HeaderMap* headermap);
  ClientBuilder* default_headers(std::initializer_list<Pair> headers);

  /// Allow HTTP/0.9 responses
  ClientBuilder* http09_responses();

  /// Set whether HTTP/1 connections will accept obsolete line folding for
  /// header values.
  ///
  /// Newline codepoints (`\r` and `\n`) will be transformed to spaces when
  /// parsing.
  ClientBuilder* http1_allow_obsolete_multiline_headers_in_responses(bool val);

  /// Only use HTTP/1.
  ClientBuilder* http1_only();

  /// Sets the maximum idle connection per host allowed in the pool.
  ClientBuilder* http1_title_case_headers();

  /// Sets whether to use an adaptive flow control.
  ///
  /// Enabling this will override the limits set in `http2_initial_stream_window_size` and
  /// `http2_initial_connection_window_size`.
  ClientBuilder* http2_adaptive_window(bool enable);

  /// Sets the max connection-level flow control for HTTP2
  ///
  /// Default is currently 65,535 but may change internally to optimize for common uses.
  ClientBuilder* http2_initial_connection_window_size(uint32_t *size = nullptr);
  ClientBuilder* http2_initial_connection_window_size(uint32_t size);

  /// Sets the `SETTINGS_INITIAL_WINDOW_SIZE` option for HTTP2 stream-level flow control.
  ///
  /// Default is currently 65,535 but may change internally to optimize for common uses.
  ClientBuilder* http2_initial_stream_window_size(uint32_t *size = nullptr);
  ClientBuilder* http2_initial_stream_window_size(uint32_t size);

  /// Sets the maximum frame size to use for HTTP2.
  ///
  /// Default is currently 16,384 but may change internally to optimize for common uses.
  ClientBuilder* http2_max_frame_size(uint32_t *size = nullptr);
  ClientBuilder* http2_max_frame_size(uint32_t size);

  /// Only use HTTP/2.
  ClientBuilder* http2_prior_knowledge();

  /// Restrict the Client to be used with HTTPS only requests.
  ///
  /// Defaults to false.
  ClientBuilder* https_only(bool enable);

  /// Bind to a local IP Address.
  ClientBuilder* local_address(const std::string& local_address);

  /// Set the maximum allowed TLS version for connections.
  ///
  /// By default there's no maximum.
  ///
  /// # Errors
  ///
  /// A value of `tls::Version::TLS_1_3` will cause an error with the
  /// `native-tls`/`default-tls` backend. This does not mean the version
  /// isn't supported, just that it can't be set as a maximum due to
  /// technical limitations.
  ///
  /// # Optional
  ///
  /// This requires the optional `default-tls`, `native-tls`, or `rustls-tls(-...)`
  /// feature to be enabled.
  ClientBuilder* max_tls_version(const std::string& version);

  /// Set the minimum required TLS version for connections.
  ///
  /// By default the TLS backend's own default is used.
  ///
  /// # Errors
  ///
  /// A value of `tls::Version::TLS_1_3` will cause an error with the
  /// `native-tls`/`default-tls` backend. This does not mean the version
  /// isn't supported, just that it can't be set as a minimum due to
  /// technical limitations.
  ///
  /// # Optional
  ///
  /// This requires the optional `default-tls`, `native-tls`, or `rustls-tls(-...)`
  /// feature to be enabled.
  ClientBuilder* min_tls_version(const std::string& version);

  /// Disables the trust-dns async resolver.
  ///
  /// This method exists even if the optional `trust-dns` feature is not enabled.
  /// This can be used to ensure a `Client` doesn't use the trust-dns async resolver
  /// even if another dependency were to enable the optional `trust-dns` feature.
  ClientBuilder* no_trust_dns();

  /// Set an optional timeout for idle sockets being kept-alive.
  ///
  /// Pass `None` to disable timeout.
  ///
  /// Default is 90 seconds.
  ClientBuilder* pool_idle_timeout(const uint64_t* millisecond = nullptr);
  ClientBuilder* pool_idle_timeout(const uint64_t millisecond);

  /// Sets the maximum idle connection per host allowed in the pool.
  ClientBuilder* pool_max_idle_per_host(uintptr_t max);

  /// Add a `Proxy` to the list of proxies the `Client` will use.
  ///
  /// # Note
  ///
  /// Adding a proxy will disable the automatic usage of the "system" proxy.
  ClientBuilder* proxy(Proxy* proxy);

  /// Set a `redirect::Policy` for this client.
  ///
  /// Default will follow redirects up to a maximum of 10.
  ClientBuilder* redirect(uintptr_t policy);

  /// Enable or disable automatic setting of the `Referer` header.
  ///
  /// Default is `true`.
  ClientBuilder* referer(bool enable);

  /// Override DNS resolution for specific domains to a particular IP address.
  ///
  /// Warning
  ///
  /// Since the DNS protocol has no notion of ports, if you wish to send
  /// traffic to a particular port you must include this port in the URL
  /// itself, any port in the overridden addr will be ignored and traffic sent
  /// to the conventional port for the given scheme (e.g. 80 for http).
  ClientBuilder* resolve(const std::string& domain,const std::string& socket_addr);

  /// Override DNS resolution for specific domains to particular IP addresses.
  ///
  /// Warning
  ///
  /// Since the DNS protocol has no notion of ports, if you wish to send
  /// traffic to a particular port you must include this port in the URL
  /// itself, any port in the overridden addresses will be ignored and traffic sent
  /// to the conventional port for the given scheme (e.g. 80 for http).
  ClientBuilder* resolve_to_addrs(const std::string& domain,
                                  std::vector<const char *>& socket_addr_array);
  ClientBuilder* resolve_to_addrs(const std::string& domain,
                                  std::initializer_list<const char *>& socket_addr_array);

  /// Set that all sockets have `SO_KEEPALIVE` set with the supplied duration.
  ///
  /// If `None`, the option will not be set.
  ClientBuilder* tcp_keepalive(const uint64_t *millisecond = nullptr);
  ClientBuilder* tcp_keepalive(const uint64_t millisecond);

  /// Set whether sockets have `TCP_NODELAY` enabled.
  ///
  /// Default is `true`.
  ClientBuilder* tcp_nodelay(bool enable);

  /// Set a timeout for connect, read and write operations of a `Client`.
  ///
  /// Default is 30 seconds.
  ///
  /// Pass `None` to disable timeout.
  ClientBuilder* timeout(const uint64_t *millisecond = nullptr);
  ClientBuilder* timeout(const uint64_t millisecond);

  /// Controls the use of built-in system certificates during certificate validation.
  ///
  /// Defaults to `true` -- built-in system certs will be used.
  ///
  /// # Optional
  ///
  /// This requires the optional `default-tls`, `native-tls`, or `rustls-tls(-...)`
  /// feature to be enabled.
  ClientBuilder* tls_built_in_root_certs(bool tls_built_in_root_certs);

  /// Controls the use of TLS server name indication.
  ///
  /// Defaults to `true`.
  ClientBuilder* tls_sni(bool tls_sni);

  /// Sets the `User-Agent` header to be used by this client.
  ClientBuilder* user_agent(const std::string& value);

  /// Returns a `Client` that uses this `ClientBuilder` configuration.
  ///
  /// # Errors
  ///
  /// This method fails if TLS backend cannot be initialized, or the resolver
  /// cannot load the system configuration.
  Client* build();
  void destory();
private:
  ClientBuilder() = delete;
  ~ClientBuilder() = delete;
};

struct Client {

  /// Convenience method to make a `GET` request to a URL.
  ///
  /// # Errors
  ///
  /// This method fails whenever supplied `Url` cannot be parsed.
  RequestBuilder* get(const std::string& url);
  
  /// Convenience method to make a `DELETE` request to a URL.
  ///
  /// # Errors
  ///
  /// This method fails whenever supplied `Url` cannot be parsed.
  RequestBuilder* delete_(const std::string& url);

  /// Convenience method to make a `HEAD` request to a URL.
  ///
  /// # Errors
  ///
  /// This method fails whenever supplied `Url` cannot be parsed.
  RequestBuilder* head(const std::string& url);

  /// Convenience method to make a `PATCH` request to a URL.
  ///
  /// # Errors
  ///
  /// This method fails whenever supplied `Url` cannot be parsed.
  RequestBuilder* patch(const std::string& url);

  /// Convenience method to make a `POST` request to a URL.
  ///
  /// # Errors
  ///
  /// This method fails whenever supplied `Url` cannot be parsed.
  RequestBuilder* post(const std::string& url);

  /// Convenience method to make a `PUT` request to a URL.
  ///
  /// # Errors
  ///
  /// This method fails whenever supplied `Url` cannot be parsed.
  RequestBuilder* put(const std::string& url);

  /// Start building a `Request` with the `Method` and `Url`.
  ///
  /// Returns a `RequestBuilder`, which will allow setting headers and
  /// request body before sending.
  ///
  /// # Errors
  ///
  /// This method fails whenever supplied `Url` cannot be parsed.
  RequestBuilder* request(const std::string& method, const std::string& url);


  /// Executes a `Request`.
  ///
  /// A `Request` can be built manually with `Request::new()` or obtained
  /// from a RequestBuilder with `RequestBuilder::build()`.
  ///
  /// You should prefer to use the `RequestBuilder` and
  /// `RequestBuilder::send()`.
  ///
  /// # Errors
  ///
  /// This method fails if there was an error while sending request,
  /// or redirect limit was exhausted.
  Response* execute(Request *request);
  void destory();
private:
  Client() = delete;
  ~Client() = delete;
};

struct RequestBuilder {
  /// Enable HTTP basic authentication.
  RequestBuilder* basic_auth(const std::string& username,
                                             const std::string& password);
  /// Enable HTTP bearer authentication.
  RequestBuilder* bearer_auth(const std::string& token);

  /// Set the request body from u8 array.
  RequestBuilder* body(const std::vector<uint8_t>& bytes);

  /// Set the request body from file.
  RequestBuilder* file_body(const std::string& file_path);

  /// Set the request body from UTF-8 text.
  RequestBuilder* body(const std::string& str);

  /// Send a form body.
  ///
  /// Sets the body to the url encoded serialization of the passed value,
  /// and also sets the `Content-Type: application/x-www-form-urlencoded`
  /// header.
  RequestBuilder* form(const std::vector<Pair>& querys);
  RequestBuilder* form(const std::initializer_list<Pair>& querys);

  /// Add a `Header` to this Request.
  RequestBuilder* header(const std::string& key, const std::string& value);

  /// Add a `Header` to this Request.
  RequestBuilder* headers(HeaderMap* headers);

  /// Send a smaple JSON body.
  ///
  /// Sets the body to the JSON serialization of the passed value, and
  /// also sets the `Content-Type: application/json` header.
  RequestBuilder* json(const std::vector<Pair>& pairs);
  RequestBuilder* json(const std::initializer_list<Pair>& pairs);

  /// It is same to use header(content-type,application/json).body(json)
  RequestBuilder* json(const std::string& json);

  /// Modify the query string of the URL.
  ///
  /// Modifies the URL of this request, adding the parameters provided.
  /// This method appends and does not overwrite. This means that it can
  /// be called multiple times and that existing query parameters are not
  /// overwritten if the same key is used. The key will simply show up
  /// twice in the query string.
  /// Calling `.query(&[("foo", "a"), ("foo", "b")])` gives `"foo=a&foo=b"`.
  RequestBuilder* query(const std::vector<Pair>& querys);
  RequestBuilder* query(const std::initializer_list<Pair>& querys);

  /// Constructs the Request and sends it the target URL, returning a Response.
  ///
  /// # Errors
  ///
  /// This method fails if there was an error while sending request,
  /// redirect loop was detected or redirect limit was exhausted.
  Response* send();

  /// Enables a request timeout.
  ///
  /// The timeout is applied from when the request starts connecting until the
  /// response body has finished. It affects only this request and overrides
  /// the timeout configured using `ClientBuilder::timeout()`.
  RequestBuilder* timeout(uint64_t millisecond);

  //if return null clone failed
  RequestBuilder* try_clone();

  /// Set HTTP version
  RequestBuilder* version(const std::string& version);


  /// Build a `Request`, which can be inspected, modified and executed with
  /// `Client::execute()`.
  Request* build();

  void destory();
private:
  RequestBuilder() = delete;
  ~RequestBuilder() = delete;
};

struct Response {

  //This fun Consumption ownership
  /// Get the response text.
  ///
  /// This method decodes the response body with BOM sniffing
  /// and with malformed sequences replaced with the REPLACEMENT CHARACTER.
  /// Encoding is determined from the `charset` parameter of `Content-Type` header,
  /// and defaults to `utf-8` if not presented.
  std::string text_and_destory();

  /// Get the response text given a specific encoding.
  ///
  /// This method decodes the response body with BOM sniffing
  /// and with malformed sequences replaced with the REPLACEMENT CHARACTER.
  /// You can provide a default encoding for decoding the raw message, while the
  /// `charset` parameter of `Content-Type` header is still prioritized. For more information
  /// about the possible encoding name, please go to [`encoding_rs`] docs.
  ///
  /// [`encoding_rs`]: https://docs.rs/encoding_rs/0.8/encoding_rs/#relationship-with-windows-code-pages
  std::string text_with_charset_and_destory(
      const std::string& default_encoding);

  /// Get the full response body as `Bytes`.
  /// The difference from copy_to is : This fun Consumption ownership
  Bytes::ptr bytes_and_destory();

  /// todo extensions.
  /// Get the content-length of the response, if it is known.
  uint64_t content_length();

  /// Copy the response body into a writer.
  /// Don't forget free
  ///
  /// This function internally uses [`std::io::copy`] and hence will continuously read data from
  /// the body and then write it into writer in a streaming fashion until EOF is met.
  ///
  /// On success, the total number of bytes that were copied to `writer` is returned.
  ///
  /// [`std::io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
  Bytes::ptr copy_to();

  /// Get the `Headers` of this `Response`.
  const HeaderMap* headers();

  /// Get the remote address used to get this `Response`.
  std::string remote_addr();

  /// Get the `StatusCode` of this `Response`.
  uint16_t status();

  /// Get the final `Url` of this `Response`.
  std::string url();

  /// Get the HTTP `Version` of this `Response`.
  /// Don't forget free string
  ///Version::HTTP_09 => "HTTP/0.9",
  ///Version::HTTP_10 => "HTTP/1.0",
  ///Version::HTTP_11 => "HTTP/1.1",
  ///Version::HTTP_2 => "HTTP/2.0",
  ///Version::HTTP_3 => "HTTP/3.0",
  ///_ => "unreachable"
  std::string version();

  void destory();
private:
  Response() = delete;
  ~Response() = delete;
};


namespace  proxy {
  Proxy* http(const std::string& proxy_scheme);
  Proxy* https(const std::string& proxy_scheme);
  Proxy* all(const std::string& proxy_scheme);
  void destory(Proxy* p);
};

}




