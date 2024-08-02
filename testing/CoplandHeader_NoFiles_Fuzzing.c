#include <stdio.h>
#include <unistd.h>
#include <limits.h>
#include "../lib/Copland.h"

#ifndef __AFL_FUZZ_TESTCASE_LEN

ssize_t fuzz_len;
unsigned char fuzz_buf[1024000];
#define __AFL_FUZZ_TESTCASE_LEN fuzz_len
#define __AFL_FUZZ_TESTCASE_BUF fuzz_buf
#define __AFL_FUZZ_INIT() void sync(void)
#define __AFL_LOOP(x) \
  ((fuzz_len = read(0, fuzz_buf, sizeof(fuzz_buff))) > 0 ? 1 : 0)
#define __AFL_INIT() sync()

#endif

__AFL_FUZZ_INIT();
// #pragma clang optimize off
// #pragma GCC optimize("O0")

int main(int argc, char **argv)
{
  unsigned char *buf;

  __AFL_INIT();
  buf = __AFL_FUZZ_TESTCASE_BUF;

  while (__AFL_LOOP(UINT_MAX))
  {
    ASPRunRequest req = ASPRunRequest_from_string((char *)buf);
    RawEv_T *ev;
    if (req.raw_ev == NULL)
    {
      ev = build_RawEv_T("had_empty_evidence");
    }
    else
    {
      ev = build_RawEv_T(req.raw_ev->ev_val);
    }
    ASPRunResponse resp = {true, ev};
    char *resp_str = ASPRunResponse_to_string(resp);
    printf("%s", resp_str);
    free_ASPRunRequest(&req);
    free(ev);
    free(resp_str);
  }
  return 0;
}