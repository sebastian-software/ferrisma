/* WP1 probe: the message table, format_message, the error and trace macros
 * of jerror.h and the default emit_message (jerror.c).
 *
 * Usage: messages COMMAND...
 *
 *   table                    print every entry of jerror.h's message list:
 *                            `name<code>=<JMESSAGE name>` and
 *                            `text<code>=<string, hex>`, and the table and
 *                            last_jpeg_message that jpeg_std_error installs
 *   std                      jpeg_std_error() again; prints its fields
 *   code N                   err.msg_code = N
 *   ints I0 .. I7            err.msg_parm.i[0..7]
 *   str TEXT                 err.msg_parm.s: zeroed, then TEXT (at most 80
 *                            bytes, NUL-terminated only if shorter)
 *   trace N                  err.trace_level = N
 *   warnings N               err.num_warnings = N
 *   addon FIRST LAST         install the addon table below with these codes
 *   noaddon                  remove it
 *   format                   (*err.format_message)(); prints `message=`
 *   emit LEVEL               (*err.emit_message)()
 *   reset                    (*err.reset_error_mgr)()
 *   errexit CODE [P1..]      ERREXIT, ERREXIT1, 2, 3, 4 or 6, by the number
 *                            of parameters
 *   errexits CODE TEXT       ERREXITS
 *   warnms CODE [P1 [P2]]    WARNMS, WARNMS1, WARNMS2
 *   tracems LEVEL CODE [P1..] TRACEMS, TRACEMS1 to 5 and TRACEMS8
 *   tracemss LEVEL CODE TEXT TRACEMSS
 *
 * The error manager is jpeg_std_error()'s, except that output_message
 * records the formatted message instead of writing to stderr, and
 * error_exit records that it ran, calls output_message and returns, as the
 * Rust port's default error_exit does (ADR 0005), so that a script can
 * continue after an ERREXIT. Every command prints `step=<n>:<command>`, the
 * recorded calls (`output=<hex>`, `error_exit=<msg_code>`) and the state:
 * msg_code, msg_parm as 80 hex bytes, trace_level and num_warnings. Messages
 * are printed as hex up to their NUL, since %c may produce any byte.
 * tests/wp1_messages.rs runs the same scripts on the Rust port.
 */

#include "probe_support.h"
#include "jerror.h"
#include "jversion.h"

#include <stdlib.h>
#include <string.h>

/* The addon table: printf conversions with defined behavior that a message
 * string may use, beyond those of jerror.h. format_message passes the string
 * parameter only when the first conversion is exactly %s, and the eight int
 * parameters otherwise, so every other string converts at most eight ints. */
static const char * const addon_table[] = {
  "Addon %d and %d",
  "%-5d|%5d|%+d|% d|%05d|%.3d|%8.3d|%-8.3d|",
  "%u|%x|%X|%#x|%#X|%o|%#o|%08x|",
  "%c%c|%5c|%-3c|%c|",
  "%hd|%hhd|%hu|%hhu|%hx|%hhx|%%|%i|",
  "%*d|%-*d|%.*d|%0*d|",
  "%#.0o|%.0d|%.0x|%#.3o|%+.0d|% .0d|%#5o|%-#8x|",
  "%s|",
  "%+05d|% 05d|%-05d|%#08o|%.5u|%3.5x|%+i|%%%d",
  NULL,
  "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
  "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
  "xxxxxxxxxxxxxxxxxx%s",
  "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"
  "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"
  "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyy%d%d%d%d%d%d%d%d"
};

static void (*std_error_exit) (j_common_ptr cinfo);

static void RecordOutput(j_common_ptr cinfo)
{
  char buffer[JMSG_LENGTH_MAX];

  memset(buffer, 0x55, sizeof(buffer));
  (*cinfo->err->format_message) (cinfo, buffer);
  ProbePrintHex("output", (const unsigned char *)buffer, strlen(buffer));
}

static void RecordErrorExit(j_common_ptr cinfo)
{
  printf("error_exit=%d\n", cinfo->err->msg_code);
  (*cinfo->err->output_message) (cinfo);
}

static void Install(struct jpeg_error_mgr *err)
{
  jpeg_std_error(err);
  std_error_exit = err->error_exit;
  err->error_exit = RecordErrorExit;
  err->output_message = RecordOutput;
}

static void PrintState(struct jpeg_error_mgr *err)
{
  printf("msg_code=%d\n", err->msg_code);
  ProbePrintHex("msg_parm", (const unsigned char *)err->msg_parm.s,
                JMSG_STR_PARM_MAX);
  printf("trace_level=%d\n", err->trace_level);
  printf("num_warnings=%ld\n", err->num_warnings);
}

static long Integer(const char *text)
{
  char *end;
  long value = strtol(text, &end, 10);

  if (*end != '\0') {
    fprintf(stderr, "probe: bad integer %s\n", text);
    exit(2);
  }
  return value;
}

/* The integer arguments of a command: up to the next word that is not an
 * integer. */
static int Integers(int argc, char *argv[], int next, int p[], int max)
{
  int count = 0;

  while (next + count < argc && count < max) {
    char *end;

    (void)strtol(argv[next + count], &end, 10);
    if (*end != '\0' || argv[next + count][0] == '\0')
      break;
    p[count] = (int)Integer(argv[next + count]);
    count++;
  }
  return count;
}

static void Need(int argc, int next, int count, const char *command)
{
  if (next + count > argc) {
    fprintf(stderr, "probe: %s needs %d argument(s)\n", command, count);
    exit(2);
  }
}

#define JMESSAGE(code, string)  { #code, (int)code, string },

static const struct {
  const char *name;
  int code;
  const char *text;
} message_list[] = {
#include "jerror.h"
};

int main(int argc, char *argv[])
{
  struct jpeg_decompress_struct cinfo;
  struct jpeg_error_mgr err;
  int next = 1, step = 0;

  memset(&cinfo, 0, sizeof(cinfo));
  Install(&err);
  cinfo.err = &err;
  cinfo.is_decompressor = TRUE;

  while (next < argc) {
    const char *command = argv[next++];
    int p[8] = { 0 }, count;

    printf("step=%d:%s\n", step++, command);
    if (strcmp(command, "table") == 0) {
      int i;

      printf("count=%d\n", (int)JMSG_LASTMSGCODE);
      printf("last_jpeg_message=%d\n", err.last_jpeg_message);
      for (i = 0; i < (int)(sizeof(message_list) / sizeof(message_list[0]));
           i++) {
        printf("name%d=%s\n", message_list[i].code, message_list[i].name);
        ProbePrintHex("text", (const unsigned char *)message_list[i].text,
                      strlen(message_list[i].text));
        if (err.jpeg_message_table[i] != message_list[i].text &&
            strcmp(err.jpeg_message_table[i], message_list[i].text) != 0)
          printf("table_mismatch=%d\n", i);
      }
      printf("table_end_is_null=%d\n",
             err.jpeg_message_table[JMSG_LASTMSGCODE] == NULL);
    } else if (strcmp(command, "std") == 0) {
      Install(&err);
      printf("last_jpeg_message=%d\n", err.last_jpeg_message);
      printf("addon_message_table_is_null=%d\n",
             err.addon_message_table == NULL);
      printf("first_addon_message=%d\n", err.first_addon_message);
      printf("last_addon_message=%d\n", err.last_addon_message);
    } else if (strcmp(command, "code") == 0) {
      Need(argc, next, 1, command);
      err.msg_code = (int)Integer(argv[next++]);
    } else if (strcmp(command, "ints") == 0) {
      int i;

      Need(argc, next, 8, command);
      for (i = 0; i < 8; i++)
        err.msg_parm.i[i] = (int)Integer(argv[next++]);
    } else if (strcmp(command, "str") == 0) {
      size_t length;

      Need(argc, next, 1, command);
      length = strlen(argv[next]);
      memset(err.msg_parm.s, 0, JMSG_STR_PARM_MAX);
      memcpy(err.msg_parm.s, argv[next],
             length < JMSG_STR_PARM_MAX ? length : JMSG_STR_PARM_MAX);
      next++;
    } else if (strcmp(command, "trace") == 0) {
      Need(argc, next, 1, command);
      err.trace_level = (int)Integer(argv[next++]);
    } else if (strcmp(command, "warnings") == 0) {
      Need(argc, next, 1, command);
      err.num_warnings = Integer(argv[next++]);
    } else if (strcmp(command, "addon") == 0) {
      Need(argc, next, 2, command);
      err.addon_message_table = addon_table;
      err.first_addon_message = (int)Integer(argv[next++]);
      err.last_addon_message = (int)Integer(argv[next++]);
    } else if (strcmp(command, "noaddon") == 0) {
      err.addon_message_table = NULL;
    } else if (strcmp(command, "format") == 0) {
      char buffer[JMSG_LENGTH_MAX];

      memset(buffer, 0x55, sizeof(buffer));
      (*err.format_message) ((j_common_ptr)&cinfo, buffer);
      ProbePrintHex("message", (const unsigned char *)buffer, strlen(buffer));
    } else if (strcmp(command, "emit") == 0) {
      Need(argc, next, 1, command);
      (*err.emit_message) ((j_common_ptr)&cinfo, (int)Integer(argv[next++]));
    } else if (strcmp(command, "reset") == 0) {
      (*err.reset_error_mgr) ((j_common_ptr)&cinfo);
    } else if (strcmp(command, "errexit") == 0) {
      int code;

      Need(argc, next, 1, command);
      code = (int)Integer(argv[next++]);
      count = Integers(argc, argv, next, p, 6);
      next += count;
      switch (count) {
      case 0: ERREXIT(&cinfo, code); break;
      case 1: ERREXIT1(&cinfo, code, p[0]); break;
      case 2: ERREXIT2(&cinfo, code, p[0], p[1]); break;
      case 3: ERREXIT3(&cinfo, code, p[0], p[1], p[2]); break;
      case 4: ERREXIT4(&cinfo, code, p[0], p[1], p[2], p[3]); break;
      case 6: ERREXIT6(&cinfo, code, p[0], p[1], p[2], p[3], p[4], p[5]);
        break;
      default:
        fprintf(stderr, "probe: no ERREXIT with %d parameters\n", count);
        return 2;
      }
    } else if (strcmp(command, "errexits") == 0) {
      int code;

      Need(argc, next, 2, command);
      code = (int)Integer(argv[next++]);
      ERREXITS(&cinfo, code, argv[next]);
      next++;
    } else if (strcmp(command, "warnms") == 0) {
      int code;

      Need(argc, next, 1, command);
      code = (int)Integer(argv[next++]);
      count = Integers(argc, argv, next, p, 2);
      next += count;
      switch (count) {
      case 0: WARNMS(&cinfo, code); break;
      case 1: WARNMS1(&cinfo, code, p[0]); break;
      default: WARNMS2(&cinfo, code, p[0], p[1]); break;
      }
    } else if (strcmp(command, "tracems") == 0) {
      int level, code;

      Need(argc, next, 2, command);
      level = (int)Integer(argv[next++]);
      code = (int)Integer(argv[next++]);
      count = Integers(argc, argv, next, p, 8);
      next += count;
      switch (count) {
      case 0: TRACEMS(&cinfo, level, code); break;
      case 1: TRACEMS1(&cinfo, level, code, p[0]); break;
      case 2: TRACEMS2(&cinfo, level, code, p[0], p[1]); break;
      case 3: TRACEMS3(&cinfo, level, code, p[0], p[1], p[2]); break;
      case 4: TRACEMS4(&cinfo, level, code, p[0], p[1], p[2], p[3]); break;
      case 5: TRACEMS5(&cinfo, level, code, p[0], p[1], p[2], p[3], p[4]);
        break;
      case 8:
        TRACEMS8(&cinfo, level, code, p[0], p[1], p[2], p[3], p[4], p[5],
                 p[6], p[7]);
        break;
      default:
        fprintf(stderr, "probe: no TRACEMS with %d parameters\n", count);
        return 2;
      }
    } else if (strcmp(command, "tracemss") == 0) {
      int level, code;

      Need(argc, next, 3, command);
      level = (int)Integer(argv[next++]);
      code = (int)Integer(argv[next++]);
      TRACEMSS(&cinfo, level, code, argv[next]);
      next++;
    } else {
      fprintf(stderr, "probe: unknown command %s\n", command);
      return 2;
    }
    PrintState(&err);
  }
  (void)std_error_exit;
  return 0;
}
