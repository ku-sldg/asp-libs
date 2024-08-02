#include <stdio.h>
#include "../lib/Copland.h"

// Function to read the whole file into a char* buffer
char *read_file_to_string(const char *filename, size_t *size)
{
  FILE *file = fopen(filename, "rb");
  if (!file)
  {
    perror("fopen");
    return NULL;
  }

  // Seek to the end of the file to determine the size
  fseek(file, 0, SEEK_END);
  long file_size = ftell(file);
  rewind(file);

  if (file_size < 0)
  {
    perror("ftell");
    fclose(file);
    return NULL;
  }

  // Allocate memory for the file content + null terminator
  char *buffer = (char *)malloc(file_size + 1);
  if (!buffer)
  {
    perror("malloc");
    fclose(file);
    return NULL;
  }

  // Read the file into the buffer
  size_t read_size = fread(buffer, 1, file_size, file);
  if (read_size != file_size)
  {
    perror("fread");
    free(buffer);
    fclose(file);
    return NULL;
  }

  // Null-terminate the buffer
  buffer[file_size] = '\0';

  // Set the size if the pointer is provided
  if (size)
  {
    *size = file_size;
  }

  fclose(file);
  return buffer;
}
int main(int argc, char **argv)
{
  if (argc != 2)
  {
    printf("Usage: %s <request_file>.json\n", argv[0]);
    return 1;
  }
  size_t size;
  char *content = read_file_to_string(argv[1], &size);
  if (!content)
  {
    fprintf(stderr, "Failed to read file %s\n", argv[1]);
    return 1;
  }
  ASPRunRequest req = ASPRunRequest_from_string(content);
  free(content);
  RawEv_T *ev;
  if (req.raw_ev == NULL)
  {
    printf("RawEv is NULL\n");
    ev = build_RawEv_T("had_empty_evidence");
  }
  else
  {
    ev = build_RawEv_T(req.raw_ev->ev_val);
  }
  ASPRunResponse resp = {true, ev};
  char *resp_str = ASPRunResponse_to_string(resp);
  printf("%s", resp_str);
  free_ASPRunRequest(&req);
  free(ev);
  free(resp_str);
  return 0;
}
