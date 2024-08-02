#include <stdio.h>
#include <stdbool.h>
#include "../../lib/Copland.h"

#define INITIAL_FILE_SIZE 512

bool read_until_eof(FILE *file, size_t INITIAL_BUFFER_SIZE, char **buffer, size_t *total_read)
{
  size_t buffer_size = INITIAL_BUFFER_SIZE;
  *total_read = 0;
  *buffer = malloc(buffer_size);

  if (buffer == NULL)
  {
    perror("read_until_eof: Failed to allocate initial buffer");
    return false;
  }

  size_t bytes_read;
  while ((bytes_read = fread(
              *buffer + *total_read,         // Start writing at the start of the buffer + any offset for what we've already read
              1,                             // Read 1 byte at a time
              buffer_size - *total_read - 1, // Read all the way to the end of the buffer - the items already read - 1 for null terminator
              file)) > 0)
  {
    *total_read += bytes_read;

    // Check if we need to resize the buffer
    if (*total_read > buffer_size)
    {
      // This should never happen, but just in case
      perror("read_until_eof: Buffer overflow");
      free(buffer);
      return false;
    }
    else if (*total_read == (buffer_size - 1))
    {
      buffer_size *= 2;
      // Resize and reallocate (safely moves the ptrs)
      char *new_buffer = realloc(*buffer, buffer_size);

      // Checks if realloc fails
      if (new_buffer == NULL)
      {
        perror("read_until_eof: Failed to reallocate buffer");
        // Ptr wasnt properly moved, so we need to free the old buffer
        free(*buffer);
        return false;
      }

      // Update the buffer ptr, (realloc already freed the old buffer)
      *buffer = new_buffer;
    }
  }

  // Handle errors in fread
  if (ferror(file))
  {
    perror("read_until_eof: Error reading from file");
    free(buffer);
    return false;
  }

  // Null-terminate the buffer
  buffer[*total_read] = '\0';

  return true;
}

int main(int argc, char **argv)
{
  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  char *file_arg = get_ArgMap_value(req.asp_args, "filepath");
  if (file_arg == NULL)
  {
    printf("%s", ErrorResponse("No file path provided"));
    return 1;
  }
  // Read the file given by file_arg
  FILE *file = fopen(file_arg, "r");
  if (file == NULL)
  {
    printf("%s", ErrorResponse("File not found"));
    return 1;
  }
  size_t file_buf_len = INITIAL_FILE_SIZE;
  char *file_buf = (char *)malloc(sizeof(char) * file_buf_len);
  if (!read_until_eof(file, file_buf_len, &file_buf, &file_buf_len))
  {
    printf("%s", ErrorResponse("Failed to read file"));
    return 1;
  }
  char *file_hex = to_Hex(file_buf);
  ASPRunResponse resp = {true, build_RawEv_T(file_hex)};
  printf("%s", ASPRunResponse_to_string(resp));
  return 0;
}
