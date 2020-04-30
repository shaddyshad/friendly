import requests,json

def endpoint(text):	
    
#     text = str(request.args.get('text'))	
    if text=="":return("Empty Input")	
    debug=False
    resp={}
    main_url = "https://luis-final.cognitiveservices.azure.com/luis/prediction/v3.0/apps/07c0c9aa-7815-4e09-92da-787ab7310d20/slots/production/predict?subscription-key=3050a2b8146f4ea987465805faa49592&verbose=true&show-all-intents=true&log=true&query="
    trailing_subs=[" go to the"," in the"," of the", " in"," and" ," and then"," on the"," on"," then"," to"," from"," instead"]
    punctuations = '''!()-[]{};:'"\,<>./?@#$%^&*_~'''
    if text.endswith("\n"):
        text=text[:-1]
    resp['text']=text
    for x in text.lower(): # removing punctuations
        if x in punctuations: 
            text= text.replace(x, "") 
    url=main_url+text
    response = requests.request('GET',url)
    u=json.loads(response.text)

    top_intent=u['prediction']['topIntent']
    top_score=u['prediction']['intents'][u['prediction']['topIntent']]['score']
    resp['top_intent']=top_intent
    if debug:resp['top_score']=top_score

    entities=[] ## to store all the entities encountered
    sub_entities=[]
    hier_entities=[] ## to resolve the typeofnav entity 
    hierarchy=["typeofnav_chapter",'typeofnav_page','typeofnav_section',"typeofnav_passage", 'typeofnav_question', 'typeofnav_subpart', 'typeofnav_answer', 'typeofnav_rough', 'typeofnav_step', 'typeofnav_paragraph', 'typeofnav_sentence']
    ## to handle and description entities with there typeofnav entity
    entity_lis=[]
    entity_pos=[]
    entity_copy=[]
    entity_resolve_nav=[]
    pos_resolve_nav=[]
    
    ## Resolving for description:
    for entity in list(u['prediction']['entities'].keys())[:-1]:
        for i in range(len(u['prediction']['entities']["$instance"][entity])):
            ent=u['prediction']['entities']["$instance"][entity][i]['type']
            if "typeofnav" in ent:
                entity_lis.append(ent)
                entity_pos.append(u['prediction']['entities']["$instance"][entity][i]['startIndex'])  
            if "typeofnav" in ent or "description_nav" in ent :
                entity_resolve_nav.append(ent)
                pos_resolve_nav.append(u['prediction']['entities']["$instance"][entity][i]['startIndex'])  
    
    yx = sorted(zip(entity_pos, entity_lis))
    entity_lis = [x for y, x in yx]
    entity_copy=list(entity_lis)
    yx = sorted(zip(pos_resolve_nav, entity_resolve_nav))
    entity_resolve_nav = [x for y, x in yx]

    previous_flag=False
    next_flag=False
    nav_flag=False
    marked_flag=False
    status_total={} #checking if META in total
    status_flag=False #checking if META in section
    
## Entity Extraction   
    for entity in list(u['prediction']['entities'].keys())[:-1]:
## Father Entities (all except ordinal and number)
        for i in range(len(u['prediction']['entities']["$instance"][entity])):
            main_entity={}
            ent=u['prediction']['entities']["$instance"][entity][i]['type']
            tex=u['prediction']['entities']["$instance"][entity][i]['text']
            if "list" in ent or "datetime" in ent:continue ## not printing list entities (exact matches),prebuilt date entities 
## checking for insertion and deletion and changing the intent name from Navigation.
            if "delete" in ent or "write" in ent or "insert" in ent :
                resp['top_intent']="edit"
                if "description" in ent and tex.strip() in ["that","this","the","this specific","that specific","this entire","that entire"]:
                   tex="typeofnav"
            if "copy" in ent :
                resp['top_intent']="copy"
## will be added later if section is not present
            if "total" in ent:
                status_total['entity']=ent
                status_total['value']=tex
                continue
## Removing trailing subs 
            if "description" in ent: 
                for sub_t in trailing_subs:
                    if tex.endswith(sub_t):
                        tex=tex[:tex.rindex(sub_t)]  
#not printing repeated locator_marked  
            if "marked" in ent:
                if marked_flag:continue
                marked_flag=True
## Resolution of description entities (linking them with their parent entity)
            if "description1" in ent:
                ent=entity_lis[0].split("_")[1]+"_description"
                entity_copy.remove(entity_lis[0])
            if "description2" in ent:
                ent=entity_lis[1].split("_")[1]+"_description"
                entity_copy.remove(entity_lis[1])
            if "description_nav" in ent:
                ind=entity_resolve_nav.index("description_nav")
                ent=entity_resolve_nav[(ind-1)].split("_")[1]+"_description"
                entity_copy.remove(entity_resolve_nav[(ind-1)])

## Rule based previous and next entities to be linked later
            if "previous" in ent:
                previous_flag=True
                continue
            if "next" in ent:
                next_flag=True
                continue     
            if "section" in ent:
                status_flag=True
            if "typeofnav_" or "rough" in ent: #  to handle speed and repeat intent working as Navigation
                nav_flag=True
## extracting value for ordinal entity     
            if "ordinalV2" in ent:
                tex=u['prediction']['entities']['ordinalV2'][0]
        
            main_entity['entity']=ent
            main_entity['value']=tex
## some entities do not have score(Previous and next)
            if debug:
                try:
                    score=u['prediction']['entities']["$instance"][entity][i]['score']
                    main_entity['score']=score
                except : pass
##description entities are child entities so they will be added later
            if "description" in ent and not("copy" in ent or"write" in ent or "delete" in ent or "note" in ent or "reminder" in ent):
                sub_entities.append([main_entity])
                continue
            if ent in hierarchy:
                hier_entities.append(main_entity)
                continue
            entities.append(main_entity)
#child entity (ordinal and number)
        try:
            if not "." in ent:
                for c in range(len(u['prediction']['entities'][ent])):
                    for child in list(u['prediction']['entities'][ent][i].keys())[:-1]:
                        child_entity={}
                        ent2=child
                        tex=u['prediction']['entities'][ent][c]["$instance"][child][0]['text']
                        score=u['prediction']['entities'][ent][c]["$instance"][child][0]['score']
                        flag_last=False
                        if "ordinal" in ent2:
                            if tex.strip()=='last':
                                flag_last=True
                            tex=(u['prediction']['entities'][ent][c][child][0])
                        child_entity['entity']=ent2
                        child_entity['value']=tex
                        
                        if debug: child_entity['score']=score
                        entity_copy.remove(("typeofnav_"+ent2.split("_")[0]))
                        if flag_last:
                            if debug :
                                sub_entities.append([child_entity,dict({'entity': 'question_ordinal','value': {'offset': -1, 'relativeTo': 'current'},'score': 1.0})])
                            else :
                                sub_entities.append([child_entity,dict({'entity': 'question_ordinal','value': {'offset': -1, 'relativeTo': 'current'}})])
                            continue
                        sub_entities.append([child_entity])
        except:pass   
    if previous_flag and len(entity_copy)>=1:
        child_entity={} 
        for en in hierarchy:
            if en in entity_copy:
                child_entity['entity']=en.split("_")[1]+'_ordinal'
        child_entity['value']={'offset': -1, 'relativeTo': 'current'}
        sub_entities.append([child_entity])
    if next_flag and len(entity_copy)>=1:
        child_entity={}
        for en in hierarchy:
            if en in entity_copy:
                child_entity['entity']=en.split("_")[1]+'_ordinal'
        child_entity['value']={'offset': 1, 'relativeTo': 'current'}
        sub_entities.append([child_entity])
        
    for ij in hierarchy:
        for jk in hier_entities:
            if jk['entity']==ij:
                for kl in sub_entities:
                    if (kl[0]['entity']).split('_')[0] in ij:
                        jk['CHILD']=kl
                        break
                entities.append(jk)
## IF no section entity was present (status_flag=False) and total entity was present (len(status_total) is not 0) then only add "total" entity
    if not status_flag:
        if not len(status_total)==0:
            entities.append(status_total)
    resp['Entities']=entities
    #when other intents act like Navigation and uniting Navigation1 and Navigation2
    if ((resp['top_intent']=="Repeat" or resp['top_intent']=="speed" ) and nav_flag ) or "Navigation" in resp['top_intent']:
        resp["top_intent"]="Navigation"
    
# dividing Meta to- questions vs marks('marks_check') vs time ('time_check')    
    if resp["top_intent"]=='META':
        for i in resp['Entities']:
            if i['entity']=='meta_time':
                resp["top_intent"]='time_check'
                break
            if i['entity']=='meta_marks':
                resp["top_intent"]='marks_check'
                break
    
    
    if resp["top_intent"]=='BOOL_META':
        child_flag=True # check if all typeofnav have a child i.e. all typeofnav are specified
        tag_flag=False  # check if any tag is present
        quantity_flag=False # check if unassociated number entity present
        
        for entity_iter in resp['Entities']:
            if entity_iter['entity'] in ['locator_marked','locator_skipped','status_attempted','status_remaining','total']:
                tag_flag=True      
            if 'typeofnav_' in entity_iter['entity'] and 'CHILD' in entity_iter:
                childless_flag=False
            if entity_iter['entity']=='meta_marks':
                resp["top_intent"]='boolean_marks_check'
                break
            if entity_iter['entity']=='meta_time':
                resp["top_intent"]='boolean_time_check'
                break
            if entity_iter['entity']=='builtin.number':
                quantity_flag=True
        
        if "boolean" in resp["top_intent"]:return(resp) # time and marks boolean intents
## no tag present and all specified typeofnav entities
        if child_flag and not tag_flag and not quantity_flag :
            resp["top_intent"]='boolean_position_check'
            return(resp)
## IF a non child number entity present then it's assumed to be a check of quantity        
        if quantity_flag:
            resp["top_intent"]="boolean_quantity_check"
            return(resp)
        resp["top_intent"]="boolean_status_check"    
      
    return(resp)