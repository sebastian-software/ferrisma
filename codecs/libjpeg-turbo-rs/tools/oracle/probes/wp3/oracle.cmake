# The WP3 probes link WP2's support library in place of probe_support alone:
# zeroed system memory, the suspending source and the recording error
# manager. wp2_support brings probe_support with it.
set_property(TARGET wp3_entropy PROPERTY LINK_LIBRARIES wp2_support)
