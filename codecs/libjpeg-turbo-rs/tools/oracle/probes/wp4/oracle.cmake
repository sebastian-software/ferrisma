# The WP4 probes link WP2's support library in place of probe_support alone:
# the suspending source, the recording error manager and the pool printer.
# wp2_support brings probe_support with it.
set_property(TARGET wp4_idct wp4_coefidct PROPERTY LINK_LIBRARIES wp2_support)
