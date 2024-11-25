#include "../../../lib/Copland.h"
#include "../../../lib/OpenSSL_Helper.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv)
{
  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  // Data to be checked for a good signature
  RawEv_T *ev_head = req.raw_ev;
  if (ev_head == NULL || ev_head->next == NULL)
  {
    fprintf(stderr, "Error: no evidence (or insufficient evidence) provided\n");
    return 1;
  }
  char *signing_ev = concat_all_RawEv(ev_head->next);
  char *sig = from_Hex(ev_head->ev_val);

  // Check the signature
  bool verified = SHA256_digest_verify_with_key(signing_ev, sig, ("/Users/adampetz/Documents/Summer_2024/asp-libs/common_files/unsecure_pub_key_dont_use.pem"));

  // Sign the input evidence

  char *resp_ev = (verified ? "true\0" : "false\0"); //from_Hex(verified ? "true\0" : "false\0");

  ASPRunResponse resp = {true, build_RawEv_T(resp_ev)};
  printf("%s", ASPRunResponse_to_string(resp));

  return 0;
}
