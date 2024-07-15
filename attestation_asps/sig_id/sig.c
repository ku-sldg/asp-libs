#include "../../json_headers/Copland.h"
#include <openssl/evp.h>
#include <openssl/pem.h>
#include <openssl/err.h>
#include <openssl/sha.h>
#include <stdio.h>
#include <stdlib.h>

// Initialize OpenSSL
void initialize_openssl()
{
  OpenSSL_add_all_algorithms();
  ERR_load_crypto_strings();
}

// Load a private key from a PEM file
EVP_PKEY *load_private_key(const char *filename)
{
  FILE *fp = fopen(filename, "r");
  if (!fp)
    return NULL;
  EVP_PKEY *pkey = PEM_read_PrivateKey(fp, NULL, NULL, NULL);
  fclose(fp);
  return pkey;
}

// Sign the digest with the private key
unsigned char *sign_digest(EVP_PKEY *pkey, const unsigned char *digest, unsigned int digest_len, unsigned int *sig_len)
{
  EVP_MD_CTX *mdctx = EVP_MD_CTX_new();
  EVP_PKEY_CTX *pkey_ctx = NULL;
  unsigned char *sig = (unsigned char *)OPENSSL_malloc(EVP_PKEY_size(pkey));

  EVP_SignInit(mdctx, EVP_sha256());
  EVP_SignUpdate(mdctx, digest, digest_len);
  EVP_SignFinal(mdctx, sig, sig_len, pkey);

  EVP_MD_CTX_free(mdctx);
  return sig;
}

int main(int argc, char **argv)
{
  // Initialize OpenSSL
  initialize_openssl();

  // Load private key
  EVP_PKEY *pkey = load_private_key("../common_files/unsecure_priv_key_dont_use.pem");
  if (!pkey)
  {
    fprintf(stderr, "Error loading private key\n");
    return 1;
  }

  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  // Data to be signed
  char *inp_ev = req.raw_ev->ev_val;

  // Sign the digest
  unsigned int sig_len;
  unsigned char *sig = sign_digest(pkey, inp_ev, strlen(inp_ev), &sig_len);

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
