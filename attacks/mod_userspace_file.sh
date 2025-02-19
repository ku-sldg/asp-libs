#!/bin/bash

# Function to display usage instructions
usage() {
  echo "Usage: $0 [-a | -r] -g <good_targ> -b <bad_targ>"
  exit 1
}

ATTACK_BOOL='false'
REPAIR_BOOL='false'

# Parse command-line arguments
while getopts "arg:b:t:" opt; do
  case ${opt} in
    a )
      ATTACK_BOOL='true'
      ;;
    r )
      REPAIR_BOOL='true'
      ;;
    g )
      GOOD_PATH=$OPTARG
      ;;
    b )
      BAD_PATH=$OPTARG
      ;;
    t )
      TARG_PATH=$OPTARG
      ;;
    * )
      usage
      ;;
  esac
done

echo "GOOD_PATH: $GOOD_PATH"
echo "BAD_PATH: $BAD_PATH"
echo "TARG_PATH: $TARG_PATH"
if ($ATTACK_BOOL) then 
  echo "ATTACK_BOOL set" 
fi
if ($REPAIR_BOOL) then 
  echo "REPAIR_BOOL set" 
fi

# Check if all required arguments are provided
if [[ -z "$GOOD_PATH" ||  -z "$BAD_PATH" ||  -z "$TARG_PATH" || 
      (("$ATTACK_BOOL" = "true") && ("$REPAIR_BOOL" = "true")) || 
      (("$ATTACK_BOOL" = "false") && ("$REPAIR_BOOL" = "false"))  ]]; then
  usage
  exit 1
fi

if ($ATTACK_BOOL) then 
  cp $BAD_PATH $TARG_PATH 
fi 

if ($REPAIR_BOOL) then 
  cp $GOOD_PATH $TARG_PATH 
fi 

# Check if the JSON file exists
#if [[ ! -f "$JSON_TERM_FILE" ]]; then
#  echo "JSON file not found: $JSON_TERM_FILE"
#  exit 1
#fi




# Combine the length prefix and the message and send to the host and port

# NOTE: This is EXTREMELY COMPLEX, but basically we calculate the message length ourselves and send it as a 4-byte prefix before the actual message
#echo -e "\x$(printf "%x" $(((MESSAGE_LENGTH >> 24) & 0xFF)))\x$(printf "%x" $(((MESSAGE_LENGTH >> 16) & 0xFF)))\x$(printf "%x" $(((MESSAGE_LENGTH >> 8) & 0xFF)))\x$(printf "%x" $(((MESSAGE_LENGTH >> 0) & 0xFF)))$JSON_MESSAGE" \
#  | nc $HOST $PORT
