## TEMPLATE.txt
## General structure for ASP's written in rust

import sys
import json
import hashlib
import binascii
import base64

import copland

## ASP to compute the hash of the filenmae provided in the 'filepath' argument.

def body():

    # For every ASP, an ASPRunRequest appears as the single command-line argument
    numargs = len(sys.argv)
    if (numargs == 1):
        raise Exception("no ASPRunRequest provided to p_hashfile_id")
    json_req = sys.argv[1]
    request = json.loads(json_req, object_hook=copland.ASPRunRequest.from_json)


    asp_args = request.ASP_ARGS
    filename = asp_args['filepath']

    with open(filename,"rb") as f:
        bytes = f.read()

    hash_string = hashlib.sha256(bytes).hexdigest()
    # evidence as bytes
    hash_bytes = hash_string.encode()
    hash_b64 = base64.b64encode(hash_bytes).decode('ascii')

    evidence = copland.RAWEV([hash_b64])

    response = copland.successfulASPRunResponse(evidence)
    response_json = json.dumps(response, default=lambda o: o.__dict__)
    return response_json


if __name__ == "__main__":
    try:
        response_json = body()
    except BaseException as e:
        response = copland.failureASPRunResponse(str(e))
        response_json = json.dumps(response, default=lambda o: o.__dict__)
    finally:
        # The ASP output (ASPRunRequest) is written to stdout.
        # The caller will capture stdout to receive the response from this ASP.
        print(response_json)
