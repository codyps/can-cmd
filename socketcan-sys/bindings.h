#include <linux/can.h>
#include <linux/can/bcm.h>
#include <linux/can/error.h>
// NOTE: omitted because the 2 structs it defines are marked as `packed` and
// contain `can_frame`s marked as align. rustc currently emits an error when
// this occurs. It's likely the _intention_ here is to avoid having extra
// padding in `cgw_frame_mod` when it only appends a single byte. There may be
// some manual workaround avaliable to us.
//#include <linux/can/gw.h>
#include <linux/can/j1939.h>
#include <linux/can/netlink.h>
#include <linux/can/raw.h>
#include <linux/can/vxcan.h>
