VIA_ENABLE = yes
ENCODER_MAP_ENABLE = yes
# Reduce RAM usage
ifeq ($(strip $(RGB_MATRIX_ENABLE)), yes)
USE_PROCESS_STACKSIZE = 0x500
USE_EXCEPTIONS_STACKSIZE = 0x250
endif
