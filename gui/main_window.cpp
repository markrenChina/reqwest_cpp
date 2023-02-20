#include "main_window.hpp"
#include "wrappers.hpp"
#include <iostream>
#include <exception>
#include "wrappers.hpp"

ffi::Client* clinet = nullptr;

void MainWindow::onClick() {
  //std::cout << "Creating the request" << std::endl;
  {
    try{
      ffi::HeaderMap* headerMap = ffi::new_header_map();
      headerMap->insert("default","value");
      if (!clinet){
        clinet = ffi::ClientBuilder::New()
                     ->user_agent("Rust/1.0.0")
                     ->default_headers(headerMap)
                     ->build();
      }
      std::string body = clinet
                             ->get("http://192.168.1.29:8023/")
                             ->header("Test1","abv")
                             ->header("Test2","abv")
                             ->sendRequest()
                             ->text();
      std::cout << body << std::endl;
    }catch (const ffi::WrapperException& e){
      std::cout <<  e.what() << std::endl;
    }
  }
}

MainWindow::MainWindow(QWidget* parent): QMainWindow(parent) {
  button = new QPushButton("Click Me", this);

  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}