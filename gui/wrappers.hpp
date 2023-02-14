#include <string>
#include <vector>

class Request {
public:
  Request(const std::string);
  ~Request();
  Response send();

private:
  void* raw;
};



class Response {
public:
  Response(void* raw) : raw(raw) {}
  ~Response();
  std::vector<char> read_body();
private:
  void *raw;
}
