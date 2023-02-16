#include "main_window.hpp"
#include "wrappers.hpp"
#include <iostream>
#include "client.hpp"


void MainWindow::onClick() {
  //std::cout << "Creating the request" << std::endl;
  ffi::hello_world();
  //ffi::ClientBuilder* c = ffi::newClientBuilder();
  char * body = ffi::text(ffi::send(ffi::get(ffi::buildClient(ffi::newClientBuilder()), "https://www.baidu.com/")));
  std::cout << body << std::endl;
  ffi::free_string(body);
}

MainWindow::MainWindow(QWidget* parent): QMainWindow(parent) {
  button = new QPushButton("Click Me", this);

  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}