#define COPLAND_HEADER
#include <string.h>
#include <stdbool.h>
#ifndef COPLAND_JSON_HEADER
#include "CoplandJson.h"
#endif

#define DEFAULT_EV_STR_SIZE 512

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

typedef struct ArgMap
{
  char *key;
  char *value;
  struct ArgMap *next;
} ArgMap;

char *get_ArgMap_value(ArgMap *arg_map, const char *key)
{
  ArgMap *cur_arg = arg_map;
  while (cur_arg != NULL)
  {
    if (strcmp(cur_arg->key, key) == 0)
    {
      return cur_arg->value;
    }
    cur_arg = cur_arg->next;
  }
  return NULL;
}

void free_ArgMap(ArgMap *arg)
{
  if (arg == NULL)
  {
    return;
  }
  free_ArgMap(arg->next);
  free(arg->key);
  free(arg->value);
  free(arg);
}

typedef struct RawEv_T
{
  char *ev_val;
  struct RawEv_T *next;
} RawEv_T;

void free_RawEv_T(RawEv_T *ev)
{
  if (ev == NULL)
  {
    return;
  }
  free_RawEv_T(ev->next);
  free(ev->ev_val);
  free(ev);
}

char *concat_all_RawEv(RawEv_T *ev)
{
  if (ev == NULL)
  {
    return NULL;
  }
  char *cur_val = from_Hex(ev->ev_val);
  char *rec_val = concat_all_RawEv(ev->next);
  if (rec_val == NULL)
  {
    return cur_val;
  }
  size_t ret_val_size = strlen(cur_val) + strlen(rec_val) + 1;
  char *ret_val = (char *)malloc(sizeof(char) * ret_val_size);
  memset(ret_val, 0, ret_val_size);
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

void free_ASPRunRequest(ASPRunRequest *req)
{
  free(req->asp_id);
  free(req->targ_plc);
  free(req->targ);
  free_RawEv_T(req->raw_ev);
  free_ArgMap(req->asp_args);
}

ASPRunRequest build_ASPRunRequest(char *asp_id, ArgMap *asp_args, char *targ_plc, char *targ, RawEv_T *raw_ev)
{
  ASPRunRequest req = {asp_id, asp_args, targ_plc, targ, raw_ev};
  return req;
}

ASPRunRequest ASPRunRequest_from_string(const char *req)
{
  unsigned int MAX_DEPTH = 20;
  if (req == NULL)
  {
    fprintf(stderr, "Request string is null\n");
    exit(1);
  }
  JSON *json = build_empty_JSON();
  parse_json_string(req, MAX_DEPTH, &json);
  char *asp_id = strdup(get_InnerString(get_InnerJSON(json, "ASP_ID")));

  JSON *asp_args = get_InnerObject(get_InnerJSON(json, "ASP_ARGS"));
  ArgMap *top_arg = NULL;
  ArgMap *cur_arg = NULL;
  MapEntry *cur_entry = asp_args->object.entries;
  while (cur_entry != NULL)
  {
    char *key = cur_entry->key;
    char *value = get_InnerString(cur_entry->value);
    ArgMap *new_arg = (ArgMap *)malloc(sizeof(ArgMap));
    new_arg->key = strdup(key);
    new_arg->value = strdup(value);
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

  char *targ_plc = strdup(get_InnerString(get_InnerJSON(json, "TARG_PLC")));
  char *targ = strdup(get_InnerString(get_InnerJSON(json, "TARG")));

  JsonArray rawev = get_InnerArray(get_InnerJSON(get_InnerObject(get_InnerJSON(json, "RAWEV")), "RawEv"));
  RawEv_T *top_raw_ev = NULL;
  RawEv_T *cur_ev = NULL;
  for (int i = 0; i < rawev.size; i++)
  {
    InnerJSON *curev_json = rawev.items[i];
    char *ev_val = get_InnerString(curev_json);
    RawEv_T *new_ev = (RawEv_T *)malloc(sizeof(RawEv_T));
    new_ev->ev_val = strdup(ev_val);
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
  free_JSON(json);

  // This one will be a lot more complex I think
  return build_ASPRunRequest(asp_id, top_arg, targ_plc, targ, top_raw_ev);
}

typedef struct
{
  bool success;
  RawEv_T *raw_ev;
} ASPRunResponse;

void free_ASPRunResponse(ASPRunResponse *resp)
{
  free_RawEv_T(resp->raw_ev);
}

ASPRunResponse build_ASPRunResponse(bool success, RawEv_T *raw_ev)
{
  ASPRunResponse resp = {success, raw_ev};
  return resp;
}

const char *ErrorResponse(const char *resp_message)
{
  const char *preamble = "{ \"TYPE\": \"RESPONSE\", \"ACTION\": \"ASP_RUN\", \"SUCCESS\": false, \"PAYLOAD\": \"\0";
  const char *postamble = "\" }\0";
  size_t ret_val_size = strlen(preamble) + strlen(resp_message) + strlen(postamble);
  char *ret_val = (char *)malloc(sizeof(char) * ret_val_size);
  // Build the ret string
  sprintf(ret_val, "%s%s%s", preamble, resp_message, postamble);
  // Returning the final string
  return ret_val;
}

char *ASPRunResponse_to_string(ASPRunResponse resp)
{
  // Creating encoding for success
  const char *success_str = (resp.success ? "true\0" : "false\0");
  // Creating encoding for RawEv
  size_t ev_str_size = DEFAULT_EV_STR_SIZE;
  char *ev_str = (char *)malloc(sizeof(char) * ev_str_size);
  memset(ev_str, 0, ev_str_size);
  strcat(ev_str, "[");
  size_t used_ev_str_size = 1;
  RawEv_T *cur_ev = resp.raw_ev;
  while (cur_ev != NULL)
  {
    char *cur_val = cur_ev->ev_val;
    size_t cur_entry_size = strlen(cur_val) + 4; // cur_val + 2 quotes + comma + space (or ] + NULL)
    RawEv_T *next_val = cur_ev->next;
    if (used_ev_str_size + cur_entry_size > ev_str_size)
    {
      ev_str_size = (used_ev_str_size + cur_entry_size) * 2;
      ev_str = (char *)realloc(ev_str, sizeof(char) * ev_str_size);
      if (ev_str == NULL)
      {
        fprintf(stderr, "Failed to realloc in ASPRunResponse_to_string\n");
        exit(1);
      }
      memset(ev_str + used_ev_str_size, 0, ev_str_size - used_ev_str_size);
    }
    strcat(ev_str, "\"");
    strcat(ev_str, cur_val);
    strcat(ev_str, "\"");
    // sprintf(ev_str + used_ev_str_size, "\"%s\"", cur_val);
    used_ev_str_size += cur_entry_size;
    if (next_val != NULL)
    {
      strcat(ev_str, ", ");
      used_ev_str_size += 2;
    }
    cur_ev = cur_ev->next;
  }
  strcat(ev_str, "]\0");
  // Setup the hard coded values
  const char *preamble = "{ \"TYPE\": \"RESPONSE\", \"ACTION\": \"ASP_RUN\", \"SUCCESS\": ";
  const char *payload_str = ", \"PAYLOAD\": { \"RawEv\": ";
  const char *postamble = "} }";
  size_t ret_val_size = strlen(preamble) + strlen(success_str) + strlen(payload_str) + strlen(ev_str) + strlen(postamble) + 1;
  char *ret_val = (char *)malloc(sizeof(char) * ret_val_size);
  memset(ret_val, 0, ret_val_size);
  strcat(ret_val, preamble);
  strcat(ret_val, success_str);
  strcat(ret_val, payload_str);
  strcat(ret_val, ev_str);
  strcat(ret_val, postamble);
  // Cleanup
  free(ev_str);
  // Returning the final string
  return ret_val;
}