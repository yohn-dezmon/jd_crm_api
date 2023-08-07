# Endpoint Definitions and Usage

## Singular Record Endpoints 

### `/topic`
**HTTP Type:** GET
Returns a single topic record.

#### Parameters

`id`: int, topic id.

#### Example Usage 

`/topic?id=1`

### `/term`
**HTTP Type:** GET
Returns a single term record.

#### Parameters

`id`: int, term id.

#### Example Usage 

`/term?id=1`

### `/source`
**HTTP Type:** GET
Returns a single source record. 

#### Parameters

`id`: int, source id.

#### Example Usage 
`source?id=1`


## Multiple Record Endpoints

### `/topics`
**HTTP Type:** GET
Returns all available topics.

### `/terms`
**HTTP Type:** GET
Returns all available terms.

### `/sources` 
**HTTP Type:** GET
Returns all available sources.

## Relational Endpoints

### `/terms-from-topic`
**HTTP Type:** GET

Returns all of the terms related to a given topic.

#### Parameters

`topic`: string

#### Example Usage 

`/terms-from-topic?topic=climate change`

## Entity Creation Endpoints

### `/new-topic`

**HTTP Type:** POST

#### POST Body Parameters

`name`: string  
`is_verified`: bool  
`brief_description`: string, optional  
`full_description`: string, optional  
`bullet_points`: string[], optional  
`examples`: string[], optional  
`parallels`: string[], optional  
`ai_brief_description`: string, optional  
`ai_full_description`: string, optional  
`ai_bullet_points`: string[], optional  
`ai_parallels`: string[], optional  
`ai_examples`: string[], optional  
`related_terms`: string[], optional  
`related_topics`: string[], optional  
`related_sources`: string[], optional  

#### Example Usage 

See `/new-term` example usage.

### `/new-term`
**HTTP Type:** POST

#### POST Body Parameters

`name`: string  
`is_verified`: bool  
`brief_description`: string, optional  
`full_description`: string, optional  
`bullet_points`: string[], optional  
`examples`: string[], optional  
`parallels`: string[], optional  
`ai_brief_description`: string, optional  
`ai_full_description`: string, optional  
`ai_bullet_points`: string[], optional  
`ai_parallels`: string[], optional  
`ai_examples`: string[], optional  
`related_terms`: string[], optional  
`related_topics`: string[], optional  
`related_sources`: string[], optional  

#### Example Usage 

```
POST localhost:3000/new-term
BODY:
{ 
    "name":"Storm",
    "is_verified": true,
    "brief_description": "a disturbance of the atmosphere marked by wind and usually by rain, snow, hail, sleet, or thunder and lightning.",
    "bullet_points" : ["bullet1", "bullet2"],
    "examples": ["example1", "example2"]
}
```

### `/new-source` 

**HTTP Type:** POST

#### Parameters

#### Example Usage 



### `/link-entities`

**HTTP Type:** POST
Link two entities that already exist in the database.

#### POST Body Parameters

`parent_entity_type`: string, must be an existing entity type, e.g. `topic`
`child_entity_type`: string, must be an existing entity type, e.g. `term`
`parent_id`: int, the id of the parent entity 
`related_topic_ids`: int[], optional, list of IDs the parent entity is related to
`related_term_ids`: int[], optional, list of IDs the parent entity is related to
`related_source_ids`: int[], optional, list of IDS the parent entity is related to

#### Example Usage 

```
POST localhost:3000/link-entities
BODY:
{
    "parent_entity_type": "term",
    "child_entity_type": "topic",
    "parent_id": 3,
    "related_topic_ids": [1]
}
```


