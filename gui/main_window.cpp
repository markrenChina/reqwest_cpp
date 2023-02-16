#include "main_window.hpp"
#include "wrappers.hpp"
#include <iostream>
#include "client.hpp"


void MainWindow::onClick() {
  //std::cout << "Creating the request" << std::endl;
}

MainWindow::MainWindow(QWidget* parent): QMainWindow(parent) {
  button = new QPushButton("Click Me", this);

  connect(button, SIGNAL(released()), this, SLOT(onClick()));
}