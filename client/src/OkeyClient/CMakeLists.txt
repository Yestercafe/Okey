set(EXECUTABLE_NAME OkeyClient)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOUIC_SEARCH_PATHS "${CMAKE_CURRENT_SOURCE_DIR}/ui")
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

file(GLOB source_files
        LIST_DIRECTORIES false
        ${CMAKE_CURRENT_SOURCE_DIR}/source/*.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/ui/*.ui)

find_package(Qt6 REQUIRED COMPONENTS Core Gui Widgets REQUIRED)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

# vcpkg
find_package(fmt CONFIG REQUIRED)
find_package(glog CONFIG REQUIRED)
find_package(cpr CONFIG REQUIRED)
find_package(jsoncpp CONFIG REQUIRED)

add_executable(${EXECUTABLE_NAME} ${source_files}
        source/mainWindow.cpp
        include/mainWindow.h
        include/settings.h)
target_link_libraries(${EXECUTABLE_NAME} PRIVATE
        Qt6::Core
        Qt6::Gui
        Qt6::Widgets
        fmt::fmt
        glog::glog
        JsonCpp::JsonCpp
        cpr::cpr)
