#include <stdio.h>
#include "../../lib/Copland.h"

int main(int argc, char **argv)
{
  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  const char *preamble = "certifying(\0";
  const char *postamble = ")\0";
  size_t resp_ev_size = strlen(preamble) + (req.raw_ev == NULL ? 0 : strlen(req.raw_ev->ev_val)) + strlen(postamble);
  char *resp_ev = (char *)malloc(sizeof(char) * resp_ev_size);
  strcat(resp_ev, preamble);
  if (req.raw_ev != NULL)
  {
    strcat(resp_ev, req.raw_ev->ev_val);
  }
  strcat(resp_ev, postamble);
  ASPRunResponse resp = {true, build_RawEv_T(resp_ev)};
  printf("%s", ASPRunResponse_to_string(resp));
  return 0;
}
