#include "../../lib/Copland.h"
#include "../../lib/OpenSSL_Helper.h"

int main(int argc, char **argv)
{
  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  char *resp_ev = SHA256_hash_full_ev(req.raw_ev);
  ASPRunResponse resp = {true, build_RawEv_T(resp_ev)};
  printf("%s", ASPRunResponse_to_string(resp));
  return 0;
}