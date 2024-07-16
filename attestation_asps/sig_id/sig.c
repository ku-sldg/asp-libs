#include "../../lib/Copland.h"
#include "../../lib/OpenSSL_Helper.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv)
{
  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  // Data to be signed
  char *signing_ev = concat_all_RawEv(req.raw_ev);

  // Sign the digest
  unsigned char *sig = SHA256_digest_sign_with_key(signing_ev, ("../common_files/unsecure_priv_key_dont_use.pem"));

  // Sign the input evidence
  char *resp_ev_val = (char *)malloc(sizeof(char) * strlen(sig));
  RawEv_T *resp_ev = build_RawEv_T(resp_ev_val);
  // We are extending the previous evidence with the sign
  resp_ev->next = req.raw_ev;

  ASPRunResponse resp = {true, resp_ev};
  printf("%s", ASPRunResponse_to_string(resp));
  return 0;
}
