#include "../../json_headers/Copland.h"
#include <openssl/sha.h>

int main(int argc, char **argv)
{
  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  char *inp_ev = req.raw_ev->ev_val;
  // Hash inp_ev
  char *resp_ev = (char *)malloc(sizeof(char) * SHA256_DIGEST_LENGTH);
  SHA256((unsigned char *)inp_ev, strlen(inp_ev), resp_ev);
  ASPRunResponse resp = {true, build_RawEv_T(resp_ev)};
  printf("%s", ASPRunResponse_to_string(resp));
  return 0;
}