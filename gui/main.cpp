#include "main_window.hpp"
#include <QtWidgets/QApplication>
#include <iostream>
#include "wrappers.hpp"


int main(int argc,char **argv) {
  {
    std::string body = ffi::newClientBuilder()
        ->user_agent("Rust/1.0.0")
        ->build()
        ->get("http://www.baidu.com/")
        ->header("Test1","abv")
        ->header("Test2","abv")
        ->sendRequest()
        ->text();
    std::cout << body << std::endl;
  }
  std::cout << "==============1============" << std::endl;
  //ffi::send(rb);
//  char* bk = ffi::text(r);
//  if(bk){
//    std::cout << bk << std::endl;
//  }else {
//    std::cout << "null" << std::endl;
//  }
  //char * body = ffi::text(ffi::send(ffi::get(ffi::buildClient(ffi::newClientBuilder()), "https://www.baidu.com/")));
  //std::cout << body << std::endl;
  //ffi::free_string(bk);

  QApplication app(argc, argv);

  MainWindow mainWindow;
  mainWindow.show();


  app.exec();
  std::cout << "===============2===========" << std::endl;
}