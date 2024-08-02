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
  unsigned char *sig = SHA256_digest_sign_with_key(signing_ev, ("./common_files/unsecure_priv_key_dont_use.pem"));
  if (sig == NULL)
  {
    fprintf(stderr, "Error signing the digest\n");
    return 1;
  }

  // Sign the input evidence
  char *resp_ev_val = (char *)malloc(sizeof(char) * (strlen(sig) + 1));
  memset(resp_ev_val, 0, strlen(sig) + 1);
  strcat(resp_ev_val, sig);
  char *resp_ev_hex = to_Hex(resp_ev_val);

  RawEv_T *resp_ev = build_RawEv_T(resp_ev_hex);
  // We are extending the previous evidence with the sign
  resp_ev->next = req.raw_ev;

  ASPRunResponse resp = {true, resp_ev};
  printf("%s", ASPRunResponse_to_string(resp));
  return 0;
}
