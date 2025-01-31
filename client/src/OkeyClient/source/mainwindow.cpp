//
// Created by Ivan Chien on 2025/1/31.
//
#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <glog/logging.h>
#include <cpr/cpr.h>
#include <json/json.h>
#include <sstream>
#include <glog/logging.h>
#include <fmt/format.h>
#include <ranges>
#include <algorithm>
namespace rg = std::ranges;

MainWindow::MainWindow(QWidget* parent) : QMainWindow(parent), ui(new Ui::MainWindow), m_pSettings_(new Settings) {
    ui->setupUi(this);
    setWindowTitle("Okey");

    init_connection();
}

MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::init_connection() {
    connect(ui->pushButtonLogin, &QPushButton::clicked, this, &MainWindow::slt_loginClicked);
    connect(ui->pushButtonRefresh, &QPushButton::clicked, this, &MainWindow::slt_refreshClicked);
    connect(this, &MainWindow::sig_roomsUpdated, this, &MainWindow::slt_roomsUpdated);
}

void MainWindow::slt_loginClicked() {
    if (m_bLoggedIn_) {
        ui->textEditUsername->setText("");
        ui->pushButtonLogin->setText("登录");
        ui->textEditUsername->setDisabled(false);
        m_pSettings_->username = "";
    } else {
        ui->pushButtonLogin->setText("登出");
        ui->textEditUsername->setDisabled(true);
        m_pSettings_->username = ui->textEditUsername->toPlainText();
    }
    ui->labelUsername->setText(m_pSettings_->username.isEmpty() ? "" : "登录名：" + m_pSettings_->username);
    m_bLoggedIn_ = !m_bLoggedIn_;
}

void MainWindow::slt_refreshClicked() {
    auto resp = cpr::Post(cpr::Url{"http://localhost:3000/api/v1/lobby/rooms"});
    if (resp.status_code == 200) {
        Json::Value respBody;
        std::istringstream(resp.text) >> respBody;
        LOG(INFO) << fmt::format("resp: {}", respBody.toStyledString());

        m_rooms_.clear();
        for (Json::Value& rawRoom: respBody["rooms"]) {
            m_rooms_.emplace_back(rawRoom["id"].asInt(), rawRoom["name"].asString(), rg::count_if(rawRoom["players"], [](auto&) { return true; }));
        }

        rg::sort(m_rooms_, [](Room& lhs, Room& rhs) {
            return lhs.roomId < rhs.roomId;
        });

        Q_EMIT sig_roomsUpdated();
    } else {
        LOG(WARNING) << fmt::format("Request failed");
    }
}

void MainWindow::slt_roomsUpdated() {
    while (ui->tableWidgetRooms->rowCount()) {
        ui->tableWidgetRooms->removeRow(0);
    }
    for (const auto& [roomId, roomName, playerCnt]: m_rooms_) {
        auto rowCount = ui->tableWidgetRooms->rowCount();
        ui->tableWidgetRooms->insertRow(rowCount);
        ui->tableWidgetRooms->setItem(rowCount, 0, new QTableWidgetItem(QString::number(roomId)));
        ui->tableWidgetRooms->setItem(rowCount, 1, new QTableWidgetItem(QString::fromStdString(roomName)));
        ui->tableWidgetRooms->setItem(rowCount, 2, new QTableWidgetItem(QString::number(playerCnt) + " / 4"));
    }
}
