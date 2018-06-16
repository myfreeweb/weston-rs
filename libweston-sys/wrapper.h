#define REQUIRE_LIBWESTON_API_VERSION 0x5000
#include "config/config.h"
#include "weston/libweston/compositor.h"
#include "weston/libweston/compositor-drm.h"
#include "weston/libweston/compositor-wayland.h"
#include "weston/libweston/compositor-x11.h"
#include "weston/libweston/compositor-headless.h"
#include "weston/libweston/windowed-output-api.h"
#include "weston/libweston/gl-renderer.h"
#include "weston/libweston/launcher-impl.h"
extern const struct launcher_interface launcher_logind_iface;
#include "weston/libweston/pixel-formats.h"
#include "weston/libweston/pixman-renderer.h"
#include "weston/libweston/timeline-object.h"
#include "weston/libweston/windowed-output-api.h"
#include "weston/libweston-desktop/libweston-desktop.h"
