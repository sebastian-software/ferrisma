# The WP5 probe links WP2's support library in place of probe_support alone:
# the suspending source, the recording error manager and the pool printer.
# wp2_support brings probe_support with it.
set_property(TARGET wp5_decode PROPERTY LINK_LIBRARIES wp2_support)
