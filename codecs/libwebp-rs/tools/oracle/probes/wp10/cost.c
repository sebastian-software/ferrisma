// WP10 probe: the level costs and residual coding of the VP8 encoder:
// VP8CalculateLevelCosts, VP8InitResidual, VP8SetResidualCoeffs,
// VP8GetResidualCost and VP8RecordCoeffs (src/enc/cost_enc.c, src/dsp/cost.c),
// and the token buffer: VP8TBufferInit, VP8RecordCoeffTokens,
// VP8EstimateTokenSize and VP8EmitTokens (src/enc/token_enc.c) (ADR 0002).
//
// Standard input is a sequence of scripts, all fields little-endian:
//
//   random_probas (int32), stats_init (uint32), page_size (int32),
//   num_blocks (int32), then 1056 probability bytes if random_probas,
//   then num_blocks blocks of: coeff_type first ctx (int32), coeffs[16] (int16)
//
// Two encoders start from VP8DefaultProbas, or from the given probabilities,
// and have every statistic set to stats_init. For each block the probe prints
// `block=` with the last position, the residual cost, the VP8RecordCoeffs
// result (first encoder) and the VP8RecordCoeffTokens result (second
// encoder). Then it prints `level_cost=` and `remapped=` (the band of each
// remapped row), `stats=` and `token_stats=` (hex words), `estimate=`, and
// `emitted=`, the finished bytes of VP8EmitTokens with the second encoder's
// probabilities. tests/wp10_cost.rs builds the scripts and the expected lines.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/enc/cost_enc.h"
#include "src/enc/vp8i_enc.h"
#include "src/utils/bit_writer_utils.h"

static int ReadI32(int32_t* value) {
  uint8_t b[4];
  if (fread(b, 1, 4, stdin) != 4) return 0;
  *value = (int32_t)((uint32_t)b[0] | ((uint32_t)b[1] << 8) |
                     ((uint32_t)b[2] << 16) | ((uint32_t)b[3] << 24));
  return 1;
}

static int32_t MustReadI32(void) {
  int32_t value;
  if (!ReadI32(&value)) exit(2);
  return value;
}

static void SetStats(VP8Encoder* const enc, uint32_t value) {
  int t, b, c, p;
  for (t = 0; t < NUM_TYPES; ++t) {
    for (b = 0; b < NUM_BANDS; ++b) {
      for (c = 0; c < NUM_CTX; ++c) {
        for (p = 0; p < NUM_PROBAS; ++p) enc->proba.stats[t][b][c][p] = value;
      }
    }
  }
}

static void PrintStats(const char* key, const VP8Encoder* const enc) {
  int t, b, c, p;
  printf("%s=", key);
  for (t = 0; t < NUM_TYPES; ++t) {
    for (b = 0; b < NUM_BANDS; ++b) {
      for (c = 0; c < NUM_CTX; ++c) {
        for (p = 0; p < NUM_PROBAS; ++p) {
          printf("%08x", enc->proba.stats[t][b][c][p]);
        }
      }
    }
  }
  printf("\n");
}

int main(int argc, char* argv[]) {
  int32_t random_probas;
  ProbeCheckArgs(argc, 0, 0, "cost < scripts.bin");
  (void)argv;
  VP8EncDspCostInit();  // VP8GetResidualCost and VP8SetResidualCoeffs
  while (ReadI32(&random_probas)) {
    const uint32_t stats_init = (uint32_t)MustReadI32();
    const int32_t page_size = MustReadI32();
    const int32_t num_blocks = MustReadI32();
    VP8Encoder* const enc = (VP8Encoder*)calloc(1, sizeof(*enc));
    VP8Encoder* const enc2 = (VP8Encoder*)calloc(1, sizeof(*enc2));
    VP8TBuffer tokens;
    VP8BitWriter bw;
    int i, t, b, c;
    if (enc == NULL || enc2 == NULL) return 2;
    VP8DefaultProbas(enc);
    VP8DefaultProbas(enc2);
    if (random_probas) {
      if (fread(enc->proba.coeffs, 1, sizeof(enc->proba.coeffs), stdin) !=
          sizeof(enc->proba.coeffs)) {
        return 2;
      }
      memcpy(enc2->proba.coeffs, enc->proba.coeffs, sizeof(enc->proba.coeffs));
    }
    VP8CalculateLevelCosts(&enc->proba);
    VP8CalculateLevelCosts(&enc2->proba);
    SetStats(enc, stats_init);
    SetStats(enc2, stats_init);
    VP8TBufferInit(&tokens, page_size);

    for (i = 0; i < num_blocks; ++i) {
      const int32_t coeff_type = MustReadI32();
      const int32_t first = MustReadI32();
      const int32_t ctx = MustReadI32();
      int16_t coeffs[16];
      VP8Residual res, res2;
      int k, cost, recorded, tokened;
      for (k = 0; k < 16; ++k) coeffs[k] = (int16_t)MustReadI32();
      VP8InitResidual(first, coeff_type, enc, &res);
      VP8SetResidualCoeffs(coeffs, &res);
      cost = VP8GetResidualCost(ctx, &res);
      recorded = VP8RecordCoeffs(ctx, &res);
      VP8InitResidual(first, coeff_type, enc2, &res2);
      VP8SetResidualCoeffs(coeffs, &res2);
      tokened = VP8RecordCoeffTokens(ctx, &res2, &tokens);
      printf("block=%d,%d,%d,%d\n", res.last, cost, recorded, tokened);
    }

    printf("level_cost=");
    for (t = 0; t < NUM_TYPES; ++t) {
      for (b = 0; b < NUM_BANDS; ++b) {
        for (c = 0; c < NUM_CTX; ++c) {
          for (i = 0; i <= MAX_VARIABLE_LEVEL; ++i) {
            printf("%04x", enc->proba.level_cost[t][b][c][i]);
          }
        }
      }
    }
    printf("\nremapped=");
    for (t = 0; t < NUM_TYPES; ++t) {
      for (b = 0; b < 16; ++b) {
        for (c = 0; c < NUM_CTX; ++c) {
          const uint16_t* const row = enc->proba.remapped_costs[t][b][c];
          const ptrdiff_t rows =
              (row - &enc->proba.level_cost[t][0][0][0]) / (MAX_VARIABLE_LEVEL + 1);
          printf("%s%d", (t || b || c) ? "," : "", (int)(rows / NUM_CTX));
          if (rows % NUM_CTX != c) return 3;
        }
      }
    }
    printf("\n");
    PrintStats("stats", enc);
    PrintStats("token_stats", enc2);
    printf("estimate=%zu\n",
           VP8EstimateTokenSize(&tokens, (const uint8_t*)enc2->proba.coeffs));
    if (!VP8BitWriterInit(&bw, 0)) return 2;
    printf("emit=%d\n", VP8EmitTokens(&tokens, &bw,
                                      (const uint8_t*)enc2->proba.coeffs, 1));
    VP8BitWriterFinish(&bw);
    ProbePrintHex("emitted", VP8BitWriterBuf(&bw), VP8BitWriterSize(&bw));
    VP8BitWriterWipeOut(&bw);
    VP8TBufferClear(&tokens);
    free(enc);
    free(enc2);
    fflush(stdout);
  }
  return 0;
}
