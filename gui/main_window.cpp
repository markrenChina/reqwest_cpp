#include "main_window.hpp"
#include "client.hpp"
#include "wrappers.hpp"
#include <iostream>
#include <exception>
#include <ostream>
#include <vector>
#include "wrappers.hpp"

ffi::Client* clinet = nullptr;

void MainWindow::test_full() {
  //std::cout << "Creating the request" << std::endl;
    try{
      auto cb = ffi::ClientBuilder::New();
      auto headerMap = ffi::HeaderMap::New();
      headerMap->insert("default","value");
      if (!clinet){
        clinet = ffi::ClientBuilder::New()
                     ->user_agent("Rust/1.0.0")
                     ->default_headers(headerMap)
                     ->default_headers({{"de","he"}})
                     ->redirect(10)
                     //->proxy(ffi::proxy::http("http://192.168.1.37:8888"))
                     ->timeout(nullptr)
                     ->build();
      }
      ffi::Response* resp = clinet
                             ->get("http://192.168.1.29:8023/c9/xx")
                             ->basic_auth("admin","password")
                             ->header("Test1","abv")
                             ->header("Test2","abv")
                             ->query({{"3","4"},{"5","6"}})
                             ->body("123456")
//                             ->json({{"name","markrenChina"}})
//                             ->json("{\"test\":123}")
                             //->file_body("rest_client.log")
                             ->timeout(1000)
                             ->send();
      auto headermap2 = resp->headers();
      //headmap2->get("content-type");
      std::cout << headermap2->get("content-type") << std::endl;
      std::cout << headermap2->keys() << std::endl;
      std::cout << headermap2->values() << std::endl;
//      std::string body = resp->text_and_destory();
//      std::cout << body << std::endl;
      headermap2->destory();
      ffi::Bytes::ptr b = resp->bytes_and_destory();
    }catch (const ffi::WrapperException& e){
      std::cout << e.what() << std::endl;
    }
}

void MainWindow::test_destructor() {
}

MainWindow::MainWindow(QWidget* parent): QMainWindow(parent) {

  button = new QPushButton("test", this);

  connect(button, SIGNAL(released()), this, SLOT(test_full()));
}