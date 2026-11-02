/* Supplementary native dependency probe; this does not implement codec code. */
#include <errno.h>
#include <stdio.h>

#include <libvmaf/feature.h>
#include <libvmaf/libvmaf.h>
#include <libvmaf/model.h>

int main(int argc, char **argv) {
  if (argc != 2 && argc != 3) return 2;
  VmafContext *context = NULL;
  VmafModel *model = NULL;
  VmafConfiguration config = { .log_level = VMAF_LOG_LEVEL_NONE };
  VmafModelConfig model_config = {
    .name = "vmaf", .flags = VMAF_MODEL_FLAG_DISABLE_CLIP
  };
  if (vmaf_init(&context, config)) return 3;
  if (vmaf_model_load_from_path(&model, &model_config, argv[1])) {
    vmaf_close(context);
    return 4;
  }
  if (argc == 3) {
    VmafFeatureDictionary *motion = NULL;
    if (vmaf_feature_dictionary_set(&motion, "motion_force_zero", argv[2]))
      return 5;
    /* The original libaom overload uses "1" for this boolean option. */
    if (vmaf_model_feature_overload(model, "float_motion", motion)) return 6;
  }
  const int features = vmaf_use_features_from_model(context, model);
  vmaf_model_destroy(model);
  if (vmaf_close(context)) return 7;
  printf("{\"features\":%d,\"invalid_argument\":%d}\n", features, -EINVAL);
  return 0;
}
