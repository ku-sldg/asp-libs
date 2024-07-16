#define COPLAND_HEADER
#include <string.h>
#include <stdbool.h>
#ifndef COPLAND_JSON_HEADER
#include "CoplandJson.h"
#endif

#define DEFAULT_EV_STR_SIZE 512

typedef struct ArgMap
{
  char *key;
  char *value;
  struct ArgMap *next;
} ArgMap;

typedef struct RawEv_T
{
  char *ev_val;
  struct RawEv_T *next;
} RawEv_T;

char *concat_all_RawEv(RawEv_T *ev)
{
  if (ev == NULL)
  {
    return "";
  }
  char *cur_val = ev->ev_val;
  char *rec_val = concat_all_RawEv(ev->next);
  char *ret_val = (char *)malloc(sizeof(char) * (strlen(cur_val) + strlen(rec_val) + 1));
  strcat(ret_val, cur_val);
  strcat(ret_val, rec_val);
  return ret_val;
}

RawEv_T *build_RawEv_T(char *ev_val)
{
  RawEv_T *ev = (RawEv_T *)malloc(sizeof(RawEv_T));
  ev->ev_val = ev_val;
  ev->next = NULL;
  return ev;
}

typedef struct
{
  char *asp_id;
  ArgMap *asp_args;
  char *targ_plc;
  char *targ;
  RawEv_T *raw_ev;
} ASPRunRequest;

ASPRunRequest build_ASPRunRequest(char *asp_id, ArgMap *asp_args, char *targ_plc, char *targ, RawEv_T *raw_ev)
{
  ASPRunRequest req = {asp_id, asp_args, targ_plc, targ, raw_ev};
  return req;
}

ASPRunRequest ASPRunRequest_from_string(const char *req)
{
  if (req == NULL)
  {
    fprintf(stderr, "Request string is null\n");
    exit(1);
  }
  JSON *json = build_empty_JSON();
  parse_json_string(req, &json);
  char *asp_id = get_InnerString(get_InnerJSON(json, "ASP_ID"));

  JSON *asp_args = get_InnerObject(get_InnerJSON(json, "ASP_ARGS"));
  ArgMap *top_arg = NULL;
  ArgMap *cur_arg = NULL;
  MapEntry *cur_entry = asp_args->object.entries;
  while (cur_entry != NULL)
  {
    char *key = cur_entry->key;
    char *value = get_InnerString(cur_entry->value);
    ArgMap *new_arg = (ArgMap *)malloc(sizeof(ArgMap));
    new_arg->key = key;
    new_arg->value = value;
    new_arg->next = NULL;
    if (top_arg == NULL)
    {
      // If the top arg is null/this is the first entry,
      // then the top and current need to be set to the new_arg
      top_arg = new_arg;
      cur_arg = new_arg;
    }
    else
    {
      // If the top arg is not null/this is not the first entry,
      // then the new argument needs to be plugged into the chain, and the current argument needs to be updated to the new_arg (which is now the end)
      cur_arg->next = new_arg;
      cur_arg = new_arg;
    }
    cur_entry = cur_entry->next;
  }

  char *targ_plc = get_InnerString(get_InnerJSON(json, "TARG_PLC"));
  char *targ = get_InnerString(get_InnerJSON(json, "TARG"));

  JsonArray rawev = get_InnerArray(get_InnerJSON(get_InnerObject(get_InnerJSON(json, "RAWEV")), "RawEv"));
  RawEv_T *top_raw_ev = NULL;
  RawEv_T *cur_ev = NULL;
  for (int i = 0; i < rawev.size; i++)
  {
    InnerJSON *curev_json = rawev.items[i];
    char *ev_val = get_InnerString(curev_json);
    RawEv_T *new_ev = (RawEv_T *)malloc(sizeof(RawEv_T));
    new_ev->ev_val = ev_val;
    new_ev->next = NULL;
    if (i == 0)
    {
      // Both the top and current need to be set to the new_ev
      top_raw_ev = new_ev;
      cur_ev = new_ev;
    }
    else
    {
      // The new evidence needs to be plugged into the chain, and the current evidence needs to be updated to the new_ev (which is now the end)
      cur_ev->next = new_ev;
      cur_ev = new_ev;
    }
  }

  // This one will be a lot more complex I think
  return build_ASPRunRequest(asp_id, top_arg, targ_plc, targ, top_raw_ev);
}

typedef struct
{
  bool success;
  RawEv_T *raw_ev;
} ASPRunResponse;

ASPRunResponse build_ASPRunResponse(bool success, RawEv_T *raw_ev)
{
  ASPRunResponse resp = {success, raw_ev};
  return resp;
}

const char *ASPRunResponse_to_string(ASPRunResponse resp)
{
  // Creating encoding for success
  const char *success_str = (resp.success ? "true\0" : "false\0");
  // Creating encoding for RawEv
  size_t ev_str_size = DEFAULT_EV_STR_SIZE;
  char *ev_str = (char *)malloc(sizeof(char) * ev_str_size);
  strcat(ev_str, "[\0");
  size_t used_ev_str_size = 2;
  RawEv_T *cur_ev = resp.raw_ev;
  while (cur_ev != NULL)
  {
    char *cur_val = cur_ev->ev_val;
    size_t cur_val_size = strlen(cur_val) + 6; // Decided to go with 6 for the 2 quotes and 1 comma, 1 space, and possible 1 ending bracket + null terminator
    RawEv_T *next_val = cur_ev->next;
    if (next_val != NULL)
    {
      cur_val_size += 2;
    }
    if (used_ev_str_size + cur_val_size > ev_str_size)
    {
      ev_str_size *= 2;
      ev_str = (char *)realloc(ev_str, sizeof(char) * ev_str_size);
    }
    strcat(ev_str, "\"");
    strcat(ev_str, cur_val);
    strcat(ev_str, "\"");
    if (next_val != NULL)
    {
      strcat(ev_str, ", ");
    }
    cur_ev = cur_ev->next;
  }
  strcat(ev_str, "]");
  // Setup the hard coded values
  const char *preamble = "{ \"TYPE\": \"RESPONSE\", \"ACTION\": \"ASP_RUN\", \"SUCCESS\": \0";
  const char *payload_str = ", \"PAYLOAD\": { \"RawEv\": \0";
  const char *postamble = "} }\0";
  size_t ret_val_size = strlen(preamble) + strlen(success_str) + strlen(payload_str) + strlen(ev_str) + strlen(postamble);
  char *ret_val = (char *)malloc(sizeof(char) * ret_val_size);
  // Build the ret string
  strcat(ret_val, preamble);
  strcat(ret_val, success_str);
  strcat(ret_val, payload_str);
  strcat(ret_val, ev_str);
  strcat(ret_val, postamble);
  // Returning the final string
  return ret_val;
}