#include "../../lib/Copland.h"
#include "../../lib/OpenSSL_Helper.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv)
{
  // Initialize OpenSSL
  initialize_openssl();

  // Load private key
  EVP_PKEY *pkey = load_public_key("../common_files/unsecure_pub_key_dont_use.pem");
  if (!pkey)
  {
    fprintf(stderr, "Error loading public key\n");
    return 1;
  }

  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  // Data to be checked for a good signature
  char *inp_ev = req.raw_ev->ev_val;

  // Check the signature
  unsigned char *sig_check = NULL;
  unsigned int sig_len;
  SHA256_digest_sign(inp_ev, 

  // Sign the input evidence
  char *resp_ev = (char *)malloc(sizeof(char) * sig_len);

  ASPRunResponse resp = {true, build_RawEv_T(resp_ev)};
  printf("%s", ASPRunResponse_to_string(resp));

  // Cleanup
  OPENSSL_free(sig);
  EVP_PKEY_free(pkey);
  EVP_cleanup();
  ERR_free_strings();

  return 0;
}
