// Copyright 2021 1Conan (@1Conan)
// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
#include "quantum.h"

typedef enum {
    ITON_OS_MAC = 0x74,
    ITON_OS_WIN = 0x75,
} iton_os_type_t;

void iton_init(void);

// control
void iton_mode_usb(void);
void iton_mode_bt(void);
void iton_reset_pairing(void);
void iton_enter_pairing(void);
void iton_switch_profile(uint8_t profile);
void iton_os_mode(iton_os_type_t type);

void iton_send_keyboard(report_keyboard_t *report);
void iton_send_consumer(uint16_t data);
void iton_send_system(uint16_t data);
