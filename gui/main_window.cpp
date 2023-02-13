#include "main_window.hpp"
#include "wrappers.hpp"
#include <iostream>

extern "C" {
    void hello_world();
}

void MainWindow::onClick() {
  std::cout << "Creating the request" << std::endl;
  //hello_world();
  Request req("https://www.baidu.com/");
  std::cout << "Request created in C++" << std::endl;
}

MainWindow::MainWindow(QWidget* parent): QMainWindow(parent) {
  button = new QPushButton("Click Me", this);

  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}