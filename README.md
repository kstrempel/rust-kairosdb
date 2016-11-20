# Rust Client for KairosDB &emsp; [![Build Status](https://api.travis-ci.org/kstrempel/rust-kairosdb.svg?branch=master)](https://travis-ci.org/kstrempel/rust-kairosdb) [![Cargo](https://img.shields.io/crates/v/kairosdb.svg)](https://crates.io/crates/kairosdb)

## Description

A simple rust language client for the time series database [KairosDB](http://kairosdb.github.io/).

Development is ongoing. Currently you can add Datapoints, query them and delete them.

## Introduction

The Client itself is used as the central access point, from which
numerous operations are defined implementing each of the specific
KairosDB APIs.

```
use kairosdb::Client;
let client = Client::new("localhost", 8080);
```

A main job of a time series database is collecting and querying data.
To add data to KairosDB we have to create a `Datapoints` struct and add
the data to the object.

```
use kairosdb::datapoints::Datapoints;

let mut datapoints = Datapoints::new("myMetric", 0);
datapoints.add_ms(1000, 11.0);
datapoints.add_ms(2000, 12.0);
datapoints.add_ms(3000, 13.0);
datapoints.add_tag("test", "first");
let result = client.add(&datapoints);
assert!(result.is_ok());
```

To query data we have to create a `Query` Object with the start and end
of the query. The start and the end can be a relative time. Check the
'Time' structure for more information.

```
use std::collections::HashMap;
use kairosdb::query::{Query, Time, Metric, TimeUnit};

let mut query = Query::new(
   Time::Nanoseconds(1000),
   Time::Nanoseconds(2000));

let mut tags: HashMap<String, Vec<String>> = HashMap::new();
let metric = Metric::new("myMetric", tags, vec![]);
query.add(metric);

let result = client.query(&query).unwrap();

assert!(result.contains_key("myMetric"));
assert_eq!(result["myMetric"].len(), 2);
assert_eq!(result["myMetric"][0].time, 1000);
assert_eq!(result["myMetric"][0].value, 11.0);
assert_eq!(result["myMetric"][1].time, 2000);
assert_eq!(result["myMetric"][1].value, 12.0);
```

Deleting data is like querying data.

```
use std::collections::HashMap;
use kairosdb::query::{Query, Time, TimeUnit, Metric};

let mut query = Query::new(
   Time::Nanoseconds(1000),
   Time::Nanoseconds(3000));

let mut tags: HashMap<String, Vec<String>> = HashMap::new();
tags.insert("test".to_string(), vec!["first".to_string()]);
let metric = Metric::new("myMetric", tags, vec![]);
query.add(metric);

let result = client.delete(&query);
assert!(result.is_ok());
```


## Documentation

[Full documentation for `rust-kairosdb`.](https://kstrempel.github.io/rust-kairosdb/kairosdb/index.html)

## TODO's

### Endpoints

- [x] Add Data Points
- [x] Delete Data Points
- [ ] Delete Metric
- [ ] Health Checks
- [ ] List Metric Names
- [ ] List Tag Names
- [ ] List Tag Values
- [x] Query Metrics
- [ ] Aggregators
- [ ] Query Metric Tags
- [ ] Roll-ups
- [ ] Create Roll-up Task
- [ ] List Roll-up Tasks
- [ ] Get Roll-up Task
- [ ] Delete Roll-up Task
- [ ] Update Roll-up Task
- [x] Version

### Documentation

- [x] Overview
- [x] Add Data Points
- [x] Delete Data Points
- [ ] Delete Metric
- [ ] Health Checks
- [ ] List Metric Names
- [ ] List Tag Names
- [ ] List Tag Values
- [x] Query Metrics
- [ ] Aggregators
- [ ] Query Metric Tags
- [ ] Roll-ups
- [ ] Create Roll-up Task
- [ ] List Roll-up Tasks
- [ ] Get Roll-up Task
- [ ] Delete Roll-up Task
- [ ] Update Roll-up Task
- [x] Version

## Licence

```
   Copyright 2016 Kai Strempel

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```
