#include "../../lib/Copland.h"
#include "../../lib/OpenSSL_Helper.h"

int main(int argc, char **argv)
{
  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  if (req.raw_ev == NULL)
  {
    char *resp_str = "appraising()";
    RawEv_T *ev = build_RawEv_T(to_Hex(resp_str));
    ASPRunResponse resp = {false, ev};
    printf("%s", ASPRunResponse_to_string(resp));
    return 0;
  }
  // Raw ev <> null here
  char *resp_ev = SHA256_hash_full_ev(req.raw_ev);
  char *resp_hex = to_Hex(resp_ev);
  ASPRunResponse resp = {true, build_RawEv_T(resp_hex)};
  char *resp_str = ASPRunResponse_to_string(resp);
  printf("%s", resp_str);
  free_ASPRunRequest(&req);
  free_ASPRunResponse(&resp);
  free(resp_str);
  return 0;
}