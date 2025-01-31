//
// Created by Ivan Chien on 2025/1/31.
//

#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QObject>
#include <QMainWindow>
#include "settings.h"
#include <memory>
#include <vector>
#include <cstdint>

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();
    void init_connection();

    struct Room {
        int32_t roomId;
        std::string roomName;
        int32_t playerCnt;
        Room(int32_t roomId = 0, std::string roomName = "", int32_t playerCnt = 0) : roomId(roomId), roomName(roomName), playerCnt(playerCnt) {}
    };

signals:
    void sig_roomsUpdated();

public slots:
    void slt_loginClicked();
    void slt_refreshClicked();
    void slt_roomsUpdated();

private:
    Ui::MainWindow *ui;

    std::shared_ptr<Settings> m_pSettings_;
    bool m_bLoggedIn_ = false;
    std::vector<Room> m_rooms_;
};


#endif //MAINWINDOW_H

