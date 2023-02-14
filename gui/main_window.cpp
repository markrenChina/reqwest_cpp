#include "main_window.hpp"
#include "wrappers.hpp"
#include <iostream>
#include "client.hpp"


void MainWindow::onClick() {
  std::cout << "Creating the request" << std::endl;
  //hello_world();
  Request req("https://www.baidu.com/");
  std::cout << "Request created in C++" << std::endl;
  Response res = req.send();
  std::cout << "Received Response" << std::endl;

  std::vector<char> raw_body = res.read_body();
  std::string body(raw_body.begin(), raw_body.end());
  std::cout << body << std::endl;
}

MainWindow::MainWindow(QWidget* parent): QMainWindow(parent) {
  button = new QPushButton("Click Me", this);

  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}