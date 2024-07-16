#include <stdlib.h>  /* for size_t and NULL */
#include <stdbool.h> /* for bool */

#define OPENSSL_HELPER_HEADER
#include <openssl/evp.h>
#include <openssl/bio.h>
#include <openssl/sha.h>
#include <openssl/pem.h>
#include <openssl/err.h>

#ifndef COPLAND_HEADER
#include "Copland.h"
#endif

unsigned char *SHA256_hash_full_ev(RawEv_T *raw_ev)
{
  char *msg = concat_all_RawEv(raw_ev);
  char *resp_ev = (char *)malloc(sizeof(char) * SHA256_DIGEST_LENGTH);
  return SHA256((const unsigned char *)msg, strlen(msg), (unsigned char *)resp_ev);
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

// Load a public key from a PEM file
EVP_PKEY *load_public_key(const char *filename)
{
  FILE *fp = fopen(filename, "r");
  if (!fp)
    return NULL;
  EVP_PKEY *pkey = PEM_read_PUBKEY(fp, NULL, NULL, NULL);
  fclose(fp);
  return pkey;
}

unsigned char *SHA256_digest_sign(const char *msg, EVP_PKEY *pkey)
{
  /* https://wiki.openssl.org/index.php/EVP_Signing_and_Verifying
   * Message to be signed is contained in `msg` and has length `msg_len`.
   * Signature will be stored in `*sig` and its length will be written to
   * `sig_len`. The private key is held in `pkey`. This function is not called
   * directly by CakeML but is called by `ffisignMsg` which is called by
   * CakeML. Returns `true` upon success and `false` on failure.
   */
  EVP_MD_CTX *mdCtx = NULL;
  unsigned char *sig = NULL;
  size_t *sig_len = (size_t *)malloc(sizeof(size_t));
  bool result = false;
  if (msg == NULL || pkey == NULL)
  {
    goto cleanup;
  }

  mdCtx = EVP_MD_CTX_new();
  if (mdCtx == NULL)
  {
    printf("EVP_MD_CTX_new failed, error 0x%lx\n", ERR_get_error());
    goto cleanup;
  }
  if (EVP_DigestSignInit(mdCtx, NULL, EVP_sha256(), NULL, pkey) != 1)
  {
    printf("EVP_DigestSignInit failed, error 0x%lx\n", ERR_get_error());
    goto cleanup;
  }
  if (EVP_DigestSignUpdate(mdCtx, msg, strlen(msg)) != 1)
  {
    printf("EVP_DigestSignUpdate failed, error 0x%lx\n", ERR_get_error());
    goto cleanup;
  }
  // Call EVP_DigestSignFinal with null signature in order to get signature
  // length
  if (EVP_DigestSignFinal(mdCtx, NULL, sig_len) != 1 || *sig_len == 0)
  {
    printf("EVP_DigestSignFinal failed (1), error 0x%lx\n", ERR_get_error());
    goto cleanup;
  }
  sig = (unsigned char *)OPENSSL_malloc(*sig_len);
  if (sig == NULL)
  {
    printf("OPENSSL_malloc failed, error 0x%lx\n", ERR_get_error());
    goto cleanup;
  }
  // Obtain the signature
  if (EVP_DigestSignFinal(mdCtx, sig, sig_len) != 1)
  {
    printf("EVP_DigestSignFinal failed (2), error 0x%lx\n", ERR_get_error());
    goto cleanup;
  }
  result = true;

cleanup:
  // Cleanup
  if (result != true)
  {
    if (sig != NULL)
    {
      OPENSSL_free(sig);
    }
    throw_error("Error signing the digest");
  }
  if (mdCtx != NULL)
  {
    EVP_MD_CTX_free(mdCtx);
    mdCtx = NULL;
  }
  return sig;
}

unsigned char *SHA256_digest_sign_with_key(const char *msg, const char *key_file_path)
{
  EVP_PKEY *pkey = load_private_key(key_file_path);
  if (!pkey)
  {
    fprintf(stderr, "Error loading private key\n");
    return NULL;
  }

  unsigned char *sig = SHA256_digest_sign(msg, pkey);
  if (sig == NULL)
  {
    fprintf(stderr, "Error signing the digest\n");
  }
  EVP_PKEY_free(pkey);
  return sig;
}

bool SHA256_digest_verify(const char *msg, const unsigned char *given_sig, EVP_PKEY *key)
{
  bool verified = false;
  if (msg == NULL || given_sig == NULL || key == NULL)
  {
    goto cleanup;
  }
  EVP_MD_CTX *mdCtx = EVP_MD_CTX_new();
  if (mdCtx == NULL)
  {
    printf("EVP_MD_CTX_new failed, error 0x%lx\n", ERR_get_error());
    goto cleanup;
  }
  if (EVP_DigestVerifyInit(mdCtx, NULL, EVP_sha512(), NULL, key) != 1)
  {
    printf("EVP_DigestVerifyInit failed, error 0x%lx\n", ERR_get_error());
    goto cleanup;
  }
  if (EVP_DigestVerifyUpdate(mdCtx, msg, strlen(msg)) != 1)
  {
    printf("EVP_DigestVerifyUpdate failed, error 0x%lx\n", ERR_get_error());
    goto cleanup;
  }
  uint32_t const rc = EVP_DigestVerifyFinal(mdCtx, given_sig, strlen(given_sig));
  verified = rc == 1;

cleanup:
  // Cleanup
  if (mdCtx != NULL)
  {
    EVP_MD_CTX_free(mdCtx);
    mdCtx = NULL;
  }
  return verified;
}

bool SHA256_digest_verify_with_key(const char *msg, const char *given_sig, const char *key_file_path)
{
  EVP_PKEY *key = load_public_key(key_file_path);
  if (!key)
  {
    fprintf(stderr, "Error loading public key\n");
    return false;
  }

  bool verified = SHA256_digest_verify(msg, given_sig, key);
  EVP_PKEY_free(key);
  return verified;
}
