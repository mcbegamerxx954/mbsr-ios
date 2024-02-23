TARGET := iphone:clang:11.4
INSTALL_TARGET_PROCESSES = Minecraft


include $(THEOS)/makefiles/common.mk

TWEAK_NAME = mbsr

mbsr_FILES = Tweak.x
mbsr_CFLAGS = -fobjc-arc
mbsr_LDFLAGS = libmcbe_r.a

include $(THEOS_MAKE_PATH)/tweak.mk
