#include "main_window.hpp"
#include "wrappers.hpp"
#include <iostream>
#include "client.hpp"


void MainWindow::onClick() {
  //std::cout << "Creating the request" << std::endl;
  ffi::hello_world();
  ClientBuilder* cb = ffi::newClientBuilder();
    if(!cb){
      std::cout << "cb null" << std::endl;
    }
  Client* c = ffi::buildClient(cb);
  if(!c){
    std::cout << "c null" << std::endl;
  }
  Response* r = ffi::get_test(c,"https://www.baidu.com/");
  if(!r){
    std::cout << "r null" << std::endl;
  }
  //Response* r = ffi::send(rb);
//  char* bk = ffi::text(r);
//  if(bk){
//    std::cout << bk << std::endl;
//  }else {
//    std::cout << "null" << std::endl;
//  }
  //char * body = ffi::text(ffi::send(ffi::get(ffi::buildClient(ffi::newClientBuilder()), "https://www.baidu.com/")));
  //std::cout << body << std::endl;
  //ffi::free_string(body);
}

MainWindow::MainWindow(QWidget* parent): QMainWindow(parent) {
  button = new QPushButton("Click Me", this);

  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}