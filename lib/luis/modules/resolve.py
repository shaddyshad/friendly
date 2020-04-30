#define a function that takes string input and calls the result module
from .luis import endpoint


def resolve_input(text):
    lu_response = endpoint(text)

    parsed = parse_lu_response(lu_response)

    #find description intents and resolve them
    des = description(parsed)

    if des:
        #call Rita API to resolve for question number
        print(des)

    return parsed

# check if an intent has description
def description(intent):
    desc = list(filter(lambda x: has_description(x), intent["entities"]))



    # fetch the description
    if len(desc) > 0:
        head = desc[0]

        # get the index in this array
        index = intent["entities"].index(head)

        # get the entity
        val = head["entity"]["value"]

        return (val, index)
        


# check if a single entity has a key with description in entity

def has_description(entity):
    if "key" in entity["entity"]:
        if entity["entity"]["key"] == "description":
            return True

    return False

def parse_lu_response(lu_response):
    intent = {
        "reference": "start"
    }
    # print(lu_response)
    #extract the lop intent
    intent["top_intent"] = lu_response["top_intent"].lower()

    #extract the reference {start, current, end}
    if "Entities" in lu_response:
        entities = lu_response["Entities"]

        if not isinstance(entities, list):
            raise Exception("Could not understand your request")

        if len(entities) == 0:
            raise Exception("Cannot handle your request")

        head = entities[0]

        if "CHILD" in head :
            c = child(head["CHILD"])
            value = c["value"]
            

            if "relativeTo" in value:
                r = value["relativeTo"]

                intent["reference"] = r

    # extract the intents and their offset
    intent["entities"] = parse_lu_entities(lu_response["Entities"])

    return intent


def parse_lu_entities(lu_entities):
    if len(lu_entities) == 0:
        return []

    # process each entity in turn
    r = map(lambda x: parse_entity(x), lu_entities)

    return list(r)

# extract the typeofnav entity
def parse_entity(entity):
    out = {}

    # pass the type of entity into 'entity[entity]' key
    if "entity" in entity:
        e = entity["entity"]
        

        if e.startswith("typeofnav_"):
            i = e.find("_")

            out["entity"] = e[i+1:]

        elif e.startswith("locator_"):
            i = e.find("_")

            out["entity"] = e[i+1:]

            return out

    if "CHILD" in entity:
        head = child(entity["CHILD"])
        #check for description entities
        if "entity" in head:
            if head["entity"].endswith("description"):
                out["entity"] = {
                    "key": "description",
                    "value": head["value"]
                }
        k ="offset"

        if k in head:
            out[k] = head[k]
        else:
            out["offset"] = 1

    return out

def child(c):
    if not isinstance(c, list):
            raise Exception("Could not process your request")

    if len(c) > 1:
        raise Exception("Child cannot have more than one item")

    ch = c[0]

    return ch