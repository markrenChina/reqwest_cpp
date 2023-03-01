set(LD_LIBRARY_PATH "/usr/bin/qt-5.12/lib:$LD_LIBRARY_PATH")
set(CMAKE_PREFIX_PATH "/usr/bin/qt-5.12")
set(Qt5_DIR "${CMAKE_PREFIX_PATH}/lib/cmake/Qt5")
set(Qt5Widgets_DIR "${CMAKE_PREFIX_PATH}/lib/cmake/Qt5Widgets")
set(Qt5Network_DIR "${CMAKE_PREFIX_PATH}/lib/cmake/Qt5Network")
set(Qt5LinguistTools_DIR "${CMAKE_PREFIX_PATH}/lib/cmake/Qt5LinguistTools")

# QT 设置
set(CMAKE_CXX_STANDARD 14)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
find_package(Qt5Widgets)

set(CLIENT_BUILD_DIR ${CMAKE_BINARY_DIR}/client)
include_directories(${CLIENT_BUILD_DIR})

set(SOURCE main_window.cpp main_window.hpp wrappers.cpp wrappers.hpp main.cpp )

add_executable(gui ${SOURCE})

get_target_property(CLIENT_DIR client LOCATION)
target_link_libraries(gui Qt5::Widgets)
target_link_libraries(gui ${CLIENT_DIR}/libclient.so)
add_dependencies(gui client)

# 生成so库
set(SRC wrappers.cpp wrappers.hpp ffi.hpp)
add_library(reqwest_cpp SHARED ${SRC})
target_link_libraries(reqwest_cpp ${CLIENT_DIR}/libclient.so)
#add_dependencies(reqwest_cpp client)

# 测试静态链接生成的so库
#set(SRC2 main_window.cpp main_window.hpp wrappers.cpp wrappers.hpp ffi.hpp main.cpp)
#add_executable(test_static ${SRC2})
#target_link_libraries(test_static Qt5::Widgets)
#add_dependencies(test_static reqwest_cpp)
#target_link_libraries(test_static reqwest_cpp)
#add_dependencies(test_static client)
#target_link_libraries(test_static ${CLIENT_DIR}/libclient.a)