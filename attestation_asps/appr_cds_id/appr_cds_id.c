#include <stdio.h>
#include "../../lib/Copland.h"

int main(int argc, char **argv)
{
  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  const char *preamble = "appr_cds(\0";
  const char *postamble = ")\0";
  if (req.raw_ev == NULL)
  {
    char *resp_str = "appr_cds()";
    RawEv_T *ev = build_RawEv_T(to_Hex(resp_str));
    ASPRunResponse resp = {false, ev};
    printf("%s", ASPRunResponse_to_string(resp));
    return 0;
  }
  // Raw ev <> null here
  unsigned char *req_ev_val = from_Hex(req.raw_ev->ev_val);
  size_t resp_ev_size = strlen(preamble) + strlen(req_ev_val) + strlen(postamble);
  char *resp_ev = (char *)malloc(sizeof(char) * resp_ev_size);
  memset(resp_ev, 0, resp_ev_size);
  strcat(resp_ev, preamble);
  strcat(resp_ev, req_ev_val);
  strcat(resp_ev, postamble);
  char *resp_str = to_Hex(resp_ev);
  ASPRunResponse resp = {true, build_RawEv_T(resp_str)};
  printf("%s", ASPRunResponse_to_string(resp));
  return 0;
}
