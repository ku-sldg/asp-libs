#define COPLAND_JSON_HEADER
#include <stdlib.h>
#include <stdbool.h>
#include <stddef.h>
#include <string.h>
#include <stdio.h>

/**
 * Throws an error and exits the program
 * @param message The error message to print
 * @throws Exits the program with EXIT_FAILURE
 */
void throw_error(const char *message)
{
  fprintf(stderr, "Error: %s\n", message);
  exit(EXIT_FAILURE);
}

// Enumeration for InnerJSON types
typedef enum
{
  INJSON_OBJECT,
  INJSON_ARRAY,
  INJSON_STRING,
  INJSON_BOOLEAN
} InnerJSONType;

// Forward declaration of JSON and InnerJSON
typedef struct JSON JSON;
typedef struct InnerJSON InnerJSON;

// Structure for a map entry (key-value pair) in JSON_Object
typedef struct MapEntry
{
  char *key;
  InnerJSON *value;
  struct MapEntry *next;
} MapEntry;

// Structure for JSON_Object
typedef struct
{
  MapEntry *entries;
} JSONObject;

typedef struct
{
  InnerJSON **items;
  size_t size;
} JsonArray;

// Structure for InnerJSON
struct InnerJSON
{
  InnerJSONType type;
  union
  {
    JSON *object;
    JsonArray array;
    char *string;
    bool boolean;
  } data;
};

// Structure for JSON
struct JSON
{
  JSONObject object;
};

/**
 * Build an InnerJSON object
 */
InnerJSON *build_empty_InnerJSON()
{
  InnerJSON *innerjson = (InnerJSON *)malloc(sizeof(InnerJSON));
  // TODO: Do we need to initialize fields to NULL?
  return innerjson;
}

/**
 * Build a JSON object
 */
JSON *build_empty_JSON()
{
  JSON *json = (JSON *)malloc(sizeof(JSON));
  json->object.entries = NULL;
  return json;
}

/**
 * Get an InnerJSON from a JSON object by key
 * @param json The JSON object to get the InnerJSON from
 * @param key The key to get the InnerJSON for
 * @return The InnerJSON object for the key
 */
InnerJSON *get_InnerJSON(JSON *json, const char *key)
{
  MapEntry *entry = json->object.entries;
  while (entry != NULL)
  {
    if (strcmp(entry->key, key) == 0)
    {
      return entry->value;
    }
    entry = entry->next;
  }
  throw_error("Key not found!");
  // NOTE: This should not be reachable
  return NULL;
}

/**
 * Get a string from an InnerJSON object
 * @param innerjson The InnerJSON object to get the string from
 * @return The string value of the InnerJSON object
 */
char *get_InnerString(InnerJSON *innerjson)
{
  if (innerjson->type != INJSON_STRING)
    throw_error("Invalid InnerJSON type!");
  return innerjson->data.string;
}

/**
 * Get a bool from an InnerJSON object
 * @param innerjson The InnerJSON object to get the bool from
 * @return The bool value of the InnerJSON object
 */
bool get_InnerBool(InnerJSON *innerjson)
{
  if (innerjson->type != INJSON_BOOLEAN)
    throw_error("Invalid InnerJSON type!");
  return innerjson->data.boolean;
}

/**
 * Get an array from an InnerJSON object
 * @param innerjson The InnerJSON object to get the array from
 * @return The array value of the InnerJSON object
 */
JsonArray get_InnerArray(InnerJSON *innerjson)
{
  if (innerjson->type != INJSON_ARRAY)
    throw_error("Invalid InnerJSON type!");
  return innerjson->data.array;
}

/**
 * Get an object from an InnerJSON object
 * @param innerjson The InnerJSON object to get the object from
 * @return The object value of the InnerJSON object
 */
JSON *get_InnerObject(InnerJSON *innerjson)
{
  if (innerjson->type != INJSON_OBJECT)
    throw_error("Invalid InnerJSON type!");
  return innerjson->data.object;
}

/**
 * Create a new InnerJSON object that is an object
 * @return The newly created InnerJSON object
 */
InnerJSON *InnerJSON_Object(JSON *json)
{
  InnerJSON *innerjson = (InnerJSON *)malloc(sizeof(InnerJSON));
  innerjson->type = INJSON_OBJECT;
  innerjson->data.object = json;
  return innerjson;
}

/**
 * Create a new InnerJSON object that is an array
 * @return The newly created InnerJSON object
 */
InnerJSON *InnerJSON_Array(InnerJSON **items, size_t size)
{
  InnerJSON *innerjson = (InnerJSON *)malloc(sizeof(InnerJSON));
  innerjson->type = INJSON_ARRAY;
  innerjson->data.array.items = (InnerJSON **)malloc(size * sizeof(InnerJSON *));
  innerjson->data.array.items = items;
  innerjson->data.array.size = size;
  return innerjson;
}

/**
 * Create a new InnerJSON object that is a string
 * @return The newly created InnerJSON object
 */
InnerJSON *InnerJSON_String(const char *string)
{
  InnerJSON *innerjson = (InnerJSON *)malloc(sizeof(InnerJSON));
  innerjson->type = INJSON_STRING;
  innerjson->data.string = strdup(string);
  return innerjson;
}

/**
 * Create a new InnerJSON object that is a boolean
 * @return The newly created InnerJSON object
 */
InnerJSON *InnerJSON_Bool(bool boolean)
{
  InnerJSON *innerjson = (InnerJSON *)malloc(sizeof(InnerJSON));
  innerjson->type = INJSON_BOOLEAN;
  innerjson->data.boolean = boolean;
  return innerjson;
}

void free_JSON(JSON *json);
void free_InnerJSON(InnerJSON *innerjson);

/**
 * Frees a JSON object
 * @param json The JSON object to free
 */
void free_JSON(JSON *json)
{
  if (json == NULL)
    return;
  MapEntry *entry = json->object.entries;
  while (entry != NULL)
  {
    MapEntry *next = entry->next;
    free(entry->key);
    free_InnerJSON(entry->value);
    free(entry);
    entry = next;
  }
  free(json);
}

/**
 * Frees an InnerJSON object
 * @param innerjson The InnerJSON object to free
 */
void free_InnerJSON(InnerJSON *innerjson)
{
  if (innerjson == NULL)
    return;
  switch (innerjson->type)
  {
  case INJSON_OBJECT:
    free_JSON(innerjson->data.object);
    break;
  case INJSON_ARRAY:
    for (size_t i = 0; i < innerjson->data.array.size; i++)
    {
      free_InnerJSON(innerjson->data.array.items[i]);
    }
    free(innerjson->data.array.items);
    break;
  case INJSON_STRING:
    free(innerjson->data.string);
    break;
  case INJSON_BOOLEAN:
    // No additional memory to free since bool is stacked
    break;
  }
  free(innerjson);
}

// Add an entry to a JSON object
void add_entry_to_json_object(JSON *json, const char *key, InnerJSON *value)
{
  MapEntry *entry = (MapEntry *)malloc(sizeof(MapEntry));
  entry->key = strdup(key);
  entry->value = value;
  entry->next = json->object.entries;
  json->object.entries = entry;
}

/**
 * Parses a JSON string into an InnerJSON object
 * @param json_string The JSON string to parse
 * @param out The return slot for the InnerJSON object
 * @return The remaining string
 */
const char *parse_inner_json_string(const char *json_string, InnerJSON **out);

/**
 * Parses a JSON string into a JSON object
 * @param json_string The JSON string to parse
 * @param out The return slot for the JSON object
 * @return The remaining string
 */
const char *parse_json_string(const char *json_string, JSON **out);

/**
 * Skip whitespace characters in a string
 * @param str The string to skip whitespace in
 * @return The position in the string after skipping whitespace
 */
const char *skip_whitespace(const char *str)
{
  while (*str && (*str == ' ' || *str == '\t' || *str == '\n' || *str == '\r'))
  {
    str++;
  }
  return str;
}

/**
 * Skip whitespace and commas in a string
 * @param str The string to skip whitespace and commas in
 * @return The position in the string after skipping whitespace and commas
 */
const char *skip_white_and_commas(const char *str)
{
  bool comma_skipped = false;
  if (*str == ',')
  {
    str++;
    comma_skipped = true;
  }
  // Someone could conceiably have WHITESPACE, COMMA, WHITESPACE
  str = skip_whitespace(str);
  if (*str == ',')
  {
    if (comma_skipped)
    {
      throw_error("Multiple commas found!");
      return NULL;
    }
    str++;
  }
  str = skip_whitespace(str);
  return str;
}

/**
 * Parse a string from the JSON string
 * @return The position in the string after the string value
 * @throws If the string is not a quoted string
 */
const char *parse_string(const char *json, char **out)
{
  // If we do not start with a quote, we arent a string
  if (*json != '"')
    throw_error("Invalid string value!");
  // We must be a string
  json++; // Skip opening quote
  const char *start = json;
  while (*json && *json != '"')
  {
    if (*json == '\\')
      json++; // Skip escaped character
    json++;
  }
  size_t length = json - start;
  *out = (char *)malloc(length + 1);
  strncpy(*out, start, length);
  (*out)[length] = '\0';
  return json + 1; // Skip closing quote
}

/**
 * Parse a boolean value from the JSON string
 * @return The position in the string after the boolean value
 * @throws If the string is not a boolean value
 * */
const char *parse_bool(const char *json, bool *out)
{
  if (strncmp(json, "true", 4) == 0)
  {
    *out = true;
    return json + 4;
  }
  else if (strncmp(json, "false", 5) == 0)
  {
    *out = false;
    return json + 5;
  }
  throw_error("Invalid boolean value!");
  // Note: this is not reachable
  return NULL;
}

const char *parse_inner_json_string(const char *json_string, InnerJSON **out)
{
  json_string = skip_whitespace(json_string);

  if (*json_string == '{')
  {
    JSON *json = build_empty_JSON();
    json_string = parse_json_string(json_string, &json);
    *out = InnerJSON_Object(json);
  }
  else if (*json_string == '[')
  {
    // Parse JSON array
    InnerJSON **items = (InnerJSON **)malloc(sizeof(InnerJSON *));
    size_t size = 0;
    json_string++; // Move past the opening bracket '['
    json_string = skip_whitespace(json_string);
    while (*json_string && *json_string != ']')
    {
      // Get the next item in the array
      InnerJSON *item = build_empty_InnerJSON();
      json_string = parse_inner_json_string(json_string, &item);
      if (item == NULL)
      {
        // Error parsing element
        free_InnerJSON(*out);
        throw_error("Error parsing array element!");
      }
      // Add item to items
      items = (InnerJSON **)realloc(items, (size + 1) * sizeof(InnerJSON *));
      items[size] = item;
      size++;
      // Skip whitespace after the item
      json_string = skip_white_and_commas(json_string);
    }
    if (*json_string == ']')
    {
      *out = InnerJSON_Array(items, size);
      json_string++; // Move past the closing bracket ']'
    }
    else
    {
      // How did we end without a closing bracket!?
      throw_error("Invalid JSON array!");
    }
  }
  else if (*json_string == '"')
  {
    // Parse JSON string
    char *ret_str = NULL;
    json_string = parse_string(json_string, &ret_str);
    if (json_string == NULL)
    {
      // Error parsing string
      free_InnerJSON(*out);
      throw_error("Error parsing JSON string value!");
    }
    *out = InnerJSON_String(ret_str);
  }
  else if (strncmp(json_string, "true", 4) == 0 || strncmp(json_string, "false", 5) == 0)
  {
    // Parse JSON boolean
    bool bool_value;
    json_string = parse_bool(json_string, &bool_value);
    if (json_string == NULL)
    {
      // Error parsing boolean
      free_InnerJSON(*out);
      throw_error("Error parsing JSON boolean value!");
    }
    *out = InnerJSON_Bool(bool_value);
  }
  else
  {
    // Unexpected JSON value
    free_InnerJSON(*out);
    throw_error("Invalid JSON value!");
  }
  return json_string;
}

// Function to parse a JSON string into InnerJSON
const char *parse_json_string(const char *json_string, JSON **out)
{
  json_string = skip_whitespace(json_string);

  // Determine the type of JSON value
  if (*json_string == '{')
  {
    // Advance past the opening brace '{'
    json_string++;
    // Parse JSON object
    json_string = skip_whitespace(json_string);
    while (*json_string && *json_string != '}')
    {
      // Get the next key
      char *key = NULL;
      if (*json_string == '"')
      {
        // We should be a key string
        json_string = parse_string(json_string, &key);
        if (json_string == NULL)
        {
          // Error parsing string
          free_JSON(*out);
          throw_error("Error parsing JSON object key!");
        }
      }
      else
      {
        // Error parsing key
        free_JSON(*out);
        throw_error("Invalid JSON object: No Key found!");
      }
      if (*json_string != ':')
      {
        // Error parsing key
        free_JSON(*out);
        throw_error("Invalid JSON object: No colon found after key (or maybe key parsing failed)!");
      }
      // Advanced past the colon ':'
      json_string++;
      // Get the next entry
      InnerJSON *item = build_empty_InnerJSON();
      json_string = parse_inner_json_string(json_string, &item);
      if (item == NULL)
      {
        // Error parsing element
        free_JSON(*out);
        throw_error("Error parsing top level map/JSON object element!");
      }
      add_entry_to_json_object(*out, key, item);
      // Skip whitespace and commas
      json_string = skip_white_and_commas(json_string);
    }
    if (*json_string == '}')
    {
      json_string++; // Move past the closing brace '}'
    }
    else
    {
      // How did we end without a closing brace!?
      throw_error("Invalid JSON object!");
    }
  }
  else
  {
    throw_error("Only JSON Objects are supported at the top level!");
  }
  return json_string;
}
