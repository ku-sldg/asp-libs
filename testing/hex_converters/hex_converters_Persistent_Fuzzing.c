#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <limits.h>

void byte_to_hex(unsigned char byte, char *hex_str)
{
  sprintf(hex_str, "%02x", byte);
}

void byte_from_hex(const char *hex_str, unsigned char *byte)
{
  sscanf(hex_str, "%2hhx", byte);
}

unsigned char *from_Hex(const char *str)
{
  size_t len = strlen(str);
  if (len % 2 != 0)
  {
    // Intentionally crash the program with a segfault
    int *p = NULL;
    *p = 42;
    fprintf(stderr, "Invalid hex string\n");
    exit(1);
  }
  size_t ret_str_entries = len / 2;
  unsigned char *ret_str = (unsigned char *)malloc(ret_str_entries + 1);
  if (ret_str == NULL)
  {
    fprintf(stderr, "Failed to malloc in from_Hex\n");
    exit(1);
  }

  for (size_t i = 0; i < ret_str_entries; i++)
  {
    byte_from_hex(str + i * 2, &ret_str[i]);
  }
  ret_str[ret_str_entries] = '\0';
  return ret_str;
}

unsigned char *to_Hex(const char *str)
{
  size_t len = strlen(str);
  unsigned char *hex_str = (unsigned char *)malloc(len * 2 + 1);
  if (hex_str == NULL)
  {
    fprintf(stderr, "Failed to malloc in to_Hex\n");
    exit(1);
  }
  for (size_t i = 0; i < len; i++)
  {
    byte_to_hex(str[i], (char *)hex_str + i * 2);
  }
  hex_str[len * 2] = '\0';
  return hex_str;
}

#ifndef __AFL_FUZZ_TESTCASE_LEN

ssize_t fuzz_len;
unsigned char fuzz_buf[1024000];
#define __AFL_FUZZ_TESTCASE_LEN fuzz_len
#define __AFL_FUZZ_TESTCASE_BUF fuzz_buf
#define __AFL_FUZZ_INIT() void sync(void)
#define __AFL_LOOP(x) \
  ((fuzz_len = read(0, fuzz_buf, sizeof(fuzz_buff))) > 0 ? 1 : 0)
#define __AFL_INIT() sync()

#endif

__AFL_FUZZ_INIT();

int main(int argc, char **argv)
{
  unsigned char *buf;

  __AFL_INIT();
  buf = __AFL_FUZZ_TESTCASE_BUF;

  while (__AFL_LOOP(UINT_MAX))
  {
    unsigned char *hex_str = to_Hex((const char *)buf);
    unsigned char *orig_str = from_Hex((const char *)hex_str);
    if (strcmp((const char *)orig_str, (const char *)buf) != 0)
    {
      // Intentionally crash the program with a segfault
      int *p = NULL;
      *p = 42;
    }
    free(hex_str);
  }
  return 0;
}