// Copyright 2021 1Conan (@1Conan)
// SPDX-License-Identifier: GPL-2.0-or-later
#include <stdint.h>
#include "gpio.h"
#include "config.h"
#include "SPI.h"
#include "iton.h"
#include "quantum.h"

#ifndef ITON_IRQ_PIN
#define ITON_IRQ_PIN A0
#endif

#ifndef ITON_INT_PIN
#define ITON_INT_PIN A1
#endif

#ifndef ITON_MAX_PROFILES
#define ITON_MAX_PROFILES 3
#endif

#ifdef KEYBOARD_SHARED_EP
#define HID_REPORT_OFFSET 1
#else
#define HID_REPORT_OFFSET 0
#endif

#ifndef ITON_BT5
enum iton_cmd {
    // mcu to iton
    hid_report      = 0xA1,
    nkro_report     = 0xA2,
    media_report    = 0xA3,
    system_report   = 0xA4,
    fn_report       = 0xA5,
    control         = 0xA6,
    set_name        = 0xA7,
    set_power_conf  = 0xA8,

    // iton to mcu
    led_state       = 0xB1,
    notification    = 0xB6,
};
#else
enum iton_cmd {
    // mcu to iton
    hid_report      = 0x51,
    nkro_report     = 0x52,
    media_report    = 0x53,
    system_report   = 0x54,
    fn_report       = 0x55,
    control         = 0x56,
    set_name        = 0x57,
    set_alarm_volt  = 0x58,

    // iton to mcu
    led_state       = 0x61,
    notification    = 0x66,
};
#endif

void iton_init() {
    setPinOutput(ITON_IRQ_PIN);
    writePinLow(ITON_IRQ_PIN);
    setPinInput(ITON_INT_PIN);
}

void iton_write(uint8_t cmd, uint8_t *data, uint8_t len) {
    writePinHigh(ITON_IRQ_PIN);

    SPI0_Write1(cmd);
    SPI0_Write(data, len);
    SPI0_Write_End();

    writePinLow(ITON_IRQ_PIN);
}

void iton_write2(uint8_t cmd, uint8_t b1, uint8_t b2) {
    uint8_t data[] = {b1, b2};
    iton_write(cmd, &data[0], 2);
}

void iton_control(uint8_t cmd, uint8_t param) {
    iton_write2(control, cmd, param);
}

void iton_mode_usb() {
    iton_control(0x58, 0x01);
}

void iton_mode_bt() {
    iton_control(0x51, 0x62);
}

void iton_reset_pairing() {
    iton_control(0x51, 0x70);
}

void iton_enter_pairing() {
    iton_control(0x51, 0x89);
}

// profile - 1 to ITON_MAX_PROFILES
void iton_switch_profile(uint8_t profile) {
    if (profile < ITON_MAX_PROFILES) {
        profile = ITON_MAX_PROFILES - 1;
    }

    iton_control(0x51, 0x81 + profile);
}

void iton_os_mode(iton_os_type_t type){
    iton_control(0x51, type);
}

void iton_send_keyboard(report_keyboard_t *report) {
#ifdef NKRO_ENABLE
    if (!keymap_config.nkro) {
#endif
        iton_write(hid_report, &report->raw[HID_REPORT_OFFSET], 8);
#ifdef NKRO_ENABLE
        return;
    }

    static uint8_t iton_last_mod_sent = 0;
    if (iton_last_mod_sent != report->nkro.mods) {
        uint8_t iton_mod_report[8] = {report->nkro.mods, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
        iton_write(hid_report, &iton_mod_report, 8);
        iton_last_mod_sent = report->nkro.mods;
    }
    iton_write(nkro_report, &report->nkro.bits[0], 16);
#endif
}

void iton_send_consumer(uint16_t data) {
    iton_write2(media_report, (uint8_t)(data >> 8), (uint8_t)(data & 0x00ff));
}

void iton_send_system(uint16_t data) {
    uint8_t key = (uint8_t)(data >> 8);
    iton_write(system_report, &key, 1);
}
