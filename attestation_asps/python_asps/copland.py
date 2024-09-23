class ASP_PARAMS:
    def __init__(self, asp_id, asp_args, plc, targ_id):
        self.ASP_ID = asp_id
        self.ASP_ARGS = asp_args
        self.ASP_PLC = asp
        self.ASP_TARG_ID = asp_targ_id

class Attestation_Session:
    def __init__(self, session_plc, plc_mapping, pubkey_mapping):
        self.Session_Plc = session_plc
        self.Plc_Mapping = plc_mapping
        self.PubKey_Mapping = pubkey_mapping

class ProtocolRunRequest:
    def __init__(self, type, action, req_plc, term, rawev, attestation_session):
        self.TYPE = type
        self.ACTION = action
        self.REQ_PLC = req_plc
        self.TERM = term
        self.RAWEV = rawev
        self.ATTESTATION_SESSION = attestation_session

class ProtocolRunResponse:
    def __init__(self, type, action, success, payload):
        self.TYPE = type
        self.ACTION = action
        self.SUCCESS = success,
        self.PAYLOAD = payload

class ProtocolAppraiseRequest:
    def __init__(self, type, action, attestation_session, term, req_plc, evidence, rawev):
        self.TYPE = type,
        self.ACTION = action,
        self.ATTESTATION_SESSION = attestation_session,
        self.TERM = term,
        self.REQ_PLC = req_plc,
        self.EVIDENCE = evidence,
        self.RAWEV:  rawev

class ProtocolAppraiseResponse:
    def __init__(self, type, action, success, payload):
        self.TYPE = type
        self.ACTION = action
        self.SUCCESS = success
        self.PAYLOAD = payload

class RAWEV:
    def __init__(self, rawev):
        self.RawEv = rawev

    @staticmethod
    def from_json(dct):
        keys = dct.keys()
        if len(keys) == 1 and 'RawEv' in keys:
            return RAWEV(dct['RawEv'])
        else:
            return dct

class ASPRunRequest:
    def __init__(self, type, action, asp_id, asp_args, asp_plc, asp_targ_id, rawev):
        self.TYPE = type
        self.ACTION = action
        self.ASP_ID = asp_id
        self.ASP_ARGS = asp_args
        self.ASP_PLC = asp_plc
        self.ASP_TARG_ID = asp_targ_id
        self.RAWEV = rawev

    @staticmethod
    def from_json(dct):
        keys = dct.keys()
        if 'TYPE' in keys and 'ACTION' in keys and 'ASP_ID' in keys and 'ASP_ARGS' in keys and 'ASP_PLC' in keys and 'ASP_TARG_ID' in keys and 'RAWEV' in keys:
            return ASPRunRequest(dct['TYPE'], dct['ACTION'], dct['ASP_ID'], dct['ASP_ARGS'], dct['ASP_PLC'], dct['ASP_TARG_ID'], dct['RAWEV'])
        elif len(keys) == 1 and 'RawEv' in keys:
            return RAWEV.from_json(dct)
        else:
            return dct

class ASPRunResponse:
    def __init__(self, type, action, success, payload):
        self.TYPE = type
        self.ACTION = action
        self.SUCCESS = success
        self.PAYLOAD = payload

    @staticmethod
    def from_json(dct):
        keys = dct.keys()
        if 'TYPE' in keys and 'ACTION' in keys and 'SUCCESS' in keys and 'PAYLOAD' in keys:
            return ASPRunResponse(dct['TYPE'], dct['ACTION'], dct['SUCCESS'], dct['PAYLOAD'])
        elif len(keys) == 1 and 'RawEv' in keys:
            return RAWEV.from_json(dct)
        else:
            return dct


def failureASPRunResponse (error_msg):
    empty_evidence = RAWEV([])
    response = ASPRunResponse("RESPONSE",
                              "ASP_RUN",
                              False,
                              empty_evidence)
    return response

def successfulASPRunResponse (evidence):
    response = ASPRunResponse("RESPONSE",
                              "ASP_RUN",
                              True,
                              evidence)
    return response
