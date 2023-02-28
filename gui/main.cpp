#include "main_window.hpp"
#include <QtWidgets/QApplication>
#include <iostream>
#include "wrappers.hpp"


int main(int argc,char **argv) {
  ffi::initialize_logging();

  QApplication app(argc, argv);

  MainWindow mainWindow;
  mainWindow.show();

  app.exec();
  std::cout << "===============app exit===========" << std::endl;
}