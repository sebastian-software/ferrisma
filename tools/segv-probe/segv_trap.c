// LD_PRELOAD: on SIGSEGV/SIGBUS print fault address, registers' PC and a raw backtrace plus the exe mapping base.
#define _GNU_SOURCE
#include <signal.h>
#include <execinfo.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <ucontext.h>
#include <stdlib.h>
static char trappath[512];
static void handler(int sig, siginfo_t *si, void *ctx) {
  char buf[256]; int fd = 2;
  if (trappath[0]) { int f = open(trappath, O_WRONLY|O_CREAT|O_APPEND, 0644); if (f >= 0) fd = f; }
  ucontext_t *uc = ctx;
#if defined(__x86_64__)
  unsigned long pc = uc->uc_mcontext.gregs[REG_RIP], sp = uc->uc_mcontext.gregs[REG_RSP];
#elif defined(__aarch64__)
  unsigned long pc = uc->uc_mcontext.pc, sp = uc->uc_mcontext.sp;
#endif
  int n = snprintf(buf, sizeof buf, "SEGVTRAP sig=%d code=%d addr=%p pc=%#lx sp=%#lx\n", sig, si->si_code, si->si_addr, pc, sp);
  write(fd, buf, n);
  void *bt[64]; int k = backtrace(bt, 64);
  for (int i = 0; i < k; i++) { n = snprintf(buf, sizeof buf, "SEGVTRAP bt %p\n", bt[i]); write(fd, buf, n); }
  int m = open("/proc/self/maps", O_RDONLY);
  if (m >= 0) { ssize_t r; while ((r = read(m, buf, sizeof buf)) > 0) write(fd, buf, r); close(m); }
  signal(sig, SIG_DFL); raise(sig);
}
__attribute__((constructor)) static void init(void) {
  const char *dir = getenv("SEGVTRAP_DIR");
  if (dir) snprintf(trappath, sizeof trappath, "%s/trap.%d.txt", dir, (int)getpid());
  static char altstack[65536];
  stack_t ss = { .ss_sp = altstack, .ss_size = sizeof altstack };
  sigaltstack(&ss, 0);
  struct sigaction sa; memset(&sa, 0, sizeof sa);
  sa.sa_sigaction = handler; sa.sa_flags = SA_SIGINFO | SA_ONSTACK;
  sigaction(SIGSEGV, &sa, 0); sigaction(SIGBUS, &sa, 0);
}
