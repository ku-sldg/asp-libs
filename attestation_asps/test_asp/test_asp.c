#include <stdio.h>
#include "../../json_headers/Copland.h"

#define EV_RESP_SIZE 1024

int main(int argc, char **argv)
{
  ASPRunRequest req = ASPRunRequest_from_string(argv[1]);
  size_t resp_ev_size = EV_RESP_SIZE;
  char *resp_ev = (char *)malloc(sizeof(char) * resp_ev_size);
  for (RawEv_T *cur_ev = req.raw_ev; cur_ev != NULL; cur_ev = cur_ev->next)
  {
    if (strlen(resp_ev) + strlen(cur_ev->ev_val) >= resp_ev_size)
    {
      resp_ev_size *= 2;
      resp_ev = (char *)realloc(resp_ev, sizeof(char) * resp_ev_size);
    }
    strcat(resp_ev, cur_ev->ev_val);
  }
  ASPRunResponse resp = {true, build_RawEv_T(resp_ev)};
  printf("%s", ASPRunResponse_to_string(resp));
  // Used for debugging the arguments
  // Print arguments
  // for (int i = 0; i < argc; i++)
  // {
  //   printf("Arg %d: %s\n", i, argv[i]);
  // }
  // printf("{ \"PAYLOAD\": { \"RawEv\": [\"616E6F6E6365\"] }, \"SUCCESS\": true }");
  return 0;
}
