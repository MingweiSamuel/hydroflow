window.BENCHMARK_DATA = 
{
  "lastUpdate": 1635875900888,
  "repoUrl": "https://github.com/hydro-project/hydroflow",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "committer": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "id": "c7a7304291dea4fd8fd95509c17b6aa1d4ea029c",
          "message": "fixup! Update CI for new index.md",
          "timestamp": "2021-10-30T00:20:02Z",
          "url": "https://github.com/hydro-project/hydroflow/commit/c7a7304291dea4fd8fd95509c17b6aa1d4ea029c"
        },
        "date": 1635553707322,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/hydroflow/compiled",
            "value": 375679,
            "range": "± 2779",
            "unit": "ns/iter"
          },
          {
            "name": "fan_in/hydroflow",
            "value": 203020318,
            "range": "± 942138",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/scheduled",
            "value": 188451017,
            "range": "± 1640198",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow",
            "value": 8449248,
            "range": "± 44861",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow_builder",
            "value": 41778614,
            "range": "± 190313",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow",
            "value": 43210330,
            "range": "± 395969",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow/compiled",
            "value": 12952216,
            "range": "± 8883",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow/scheduled",
            "value": 2318677,
            "range": "± 3391",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow",
            "value": 2349046,
            "range": "± 2204",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "committer": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "id": "8399cf953a2563110977cd61c750fe8308f54a92",
          "message": "Implement reachability/hydroflow (scheduled + compiled) benchmark",
          "timestamp": "2021-10-30T00:39:34Z",
          "url": "https://github.com/hydro-project/hydroflow/commit/8399cf953a2563110977cd61c750fe8308f54a92"
        },
        "date": 1635555313000,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/hydroflow/compiled",
            "value": 375647,
            "range": "± 2908",
            "unit": "ns/iter"
          },
          {
            "name": "fan_in/hydroflow",
            "value": 189152213,
            "range": "± 790008",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/scheduled",
            "value": 182167295,
            "range": "± 1577713",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow",
            "value": 9399075,
            "range": "± 21406",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow_builder",
            "value": 46056758,
            "range": "± 104428",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow",
            "value": 47882792,
            "range": "± 337911",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow/compiled",
            "value": 14676732,
            "range": "± 12000",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow/scheduled",
            "value": 2582580,
            "range": "± 1270",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow",
            "value": 2585103,
            "range": "± 1997",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "committer": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "id": "8399cf953a2563110977cd61c750fe8308f54a92",
          "message": "Implement reachability/hydroflow (scheduled + compiled) benchmark",
          "timestamp": "2021-10-30T00:39:34Z",
          "url": "https://github.com/hydro-project/hydroflow/commit/8399cf953a2563110977cd61c750fe8308f54a92"
        },
        "date": 1635555922725,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/hydroflow/compiled",
            "value": 442207,
            "range": "± 6134",
            "unit": "ns/iter"
          },
          {
            "name": "fan_in/hydroflow",
            "value": 212331529,
            "range": "± 1121643",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/scheduled",
            "value": 213734991,
            "range": "± 2432352",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow",
            "value": 11164260,
            "range": "± 176971",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow_builder",
            "value": 56509458,
            "range": "± 3421375",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow",
            "value": 56616409,
            "range": "± 1097022",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow/compiled",
            "value": 17743046,
            "range": "± 1049780",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow/scheduled",
            "value": 3063127,
            "range": "± 48540",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow",
            "value": 3046803,
            "range": "± 37017",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "committer": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "id": "8399cf953a2563110977cd61c750fe8308f54a92",
          "message": "Implement reachability/hydroflow (scheduled + compiled) benchmark",
          "timestamp": "2021-10-30T00:39:34Z",
          "url": "https://github.com/hydro-project/hydroflow/commit/8399cf953a2563110977cd61c750fe8308f54a92"
        },
        "date": 1635565697726,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/hydroflow/compiled",
            "value": 391735,
            "range": "± 18957",
            "unit": "ns/iter"
          },
          {
            "name": "fan_in/hydroflow",
            "value": 200863567,
            "range": "± 4144721",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/scheduled",
            "value": 192958632,
            "range": "± 6767139",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow",
            "value": 9414247,
            "range": "± 404288",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow_builder",
            "value": 48966351,
            "range": "± 2541011",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow",
            "value": 50325679,
            "range": "± 3289662",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow/compiled",
            "value": 15669888,
            "range": "± 761113",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow/scheduled",
            "value": 2737481,
            "range": "± 117040",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow",
            "value": 2715808,
            "range": "± 116018",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "committer": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "id": "8399cf953a2563110977cd61c750fe8308f54a92",
          "message": "Implement reachability/hydroflow (scheduled + compiled) benchmark",
          "timestamp": "2021-10-30T00:39:34Z",
          "url": "https://github.com/hydro-project/hydroflow/commit/8399cf953a2563110977cd61c750fe8308f54a92"
        },
        "date": 1635652084444,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/hydroflow/compiled",
            "value": 375761,
            "range": "± 2816",
            "unit": "ns/iter"
          },
          {
            "name": "fan_in/hydroflow",
            "value": 202092415,
            "range": "± 563865",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/scheduled",
            "value": 193003411,
            "range": "± 2192581",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow",
            "value": 9431773,
            "range": "± 15649",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow_builder",
            "value": 47063629,
            "range": "± 188646",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow",
            "value": 48263527,
            "range": "± 355608",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow/compiled",
            "value": 14681844,
            "range": "± 7898",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow/scheduled",
            "value": 2564455,
            "range": "± 3510",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow",
            "value": 2576607,
            "range": "± 2895",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "committer": {
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel",
            "email": "mingwei.samuel@gmail.com"
          },
          "id": "8399cf953a2563110977cd61c750fe8308f54a92",
          "message": "Implement reachability/hydroflow (scheduled + compiled) benchmark",
          "timestamp": "2021-10-30T00:39:34Z",
          "url": "https://github.com/hydro-project/hydroflow/commit/8399cf953a2563110977cd61c750fe8308f54a92"
        },
        "date": 1635738504544,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/hydroflow/compiled",
            "value": 375679,
            "range": "± 2939",
            "unit": "ns/iter"
          },
          {
            "name": "fan_in/hydroflow",
            "value": 196343875,
            "range": "± 1227717",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/scheduled",
            "value": 195172065,
            "range": "± 2304817",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow",
            "value": 9345891,
            "range": "± 26936",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow_builder",
            "value": 47631627,
            "range": "± 152823",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow",
            "value": 48418903,
            "range": "± 338575",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow/compiled",
            "value": 14685187,
            "range": "± 29852",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow/scheduled",
            "value": 2672987,
            "range": "± 2518",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow",
            "value": 2593610,
            "range": "± 2511",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Justin Jaffray",
            "username": "justinj",
            "email": "justin.jaffray@gmail.com"
          },
          "committer": {
            "name": "Justin Jaffray",
            "username": "justinj",
            "email": "justin.jaffray@gmail.com"
          },
          "id": "c22ed90898193d05436d6743ed282338755d03f7",
          "message": "Add TeeingHandoff",
          "timestamp": "2021-10-29T21:58:32Z",
          "url": "https://github.com/hydro-project/hydroflow/commit/c22ed90898193d05436d6743ed282338755d03f7"
        },
        "date": 1635791856227,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/hydroflow/compiled",
            "value": 389310,
            "range": "± 15953",
            "unit": "ns/iter"
          },
          {
            "name": "fan_in/hydroflow",
            "value": 193643352,
            "range": "± 2884312",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/scheduled",
            "value": 194953275,
            "range": "± 4838714",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/teer",
            "value": 81623672,
            "range": "± 1597776",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow",
            "value": 9537620,
            "range": "± 405853",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow_builder",
            "value": 52154485,
            "range": "± 1292359",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow",
            "value": 50485401,
            "range": "± 2072238",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow/compiled",
            "value": 15244301,
            "range": "± 718444",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow/scheduled",
            "value": 2723170,
            "range": "± 111775",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow",
            "value": 2821681,
            "range": "± 108703",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Justin Jaffray",
            "username": "justinj",
            "email": "justin.jaffray@gmail.com"
          },
          "committer": {
            "name": "Justin Jaffray",
            "username": "justinj",
            "email": "justin.jaffray@gmail.com"
          },
          "id": "df21b5d61e524f71f58d502a60a203e6817617f4",
          "message": "split out handoffs",
          "timestamp": "2021-11-01T19:16:16Z",
          "url": "https://github.com/hydro-project/hydroflow/commit/df21b5d61e524f71f58d502a60a203e6817617f4"
        },
        "date": 1635824893552,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/hydroflow/compiled",
            "value": 375642,
            "range": "± 2635",
            "unit": "ns/iter"
          },
          {
            "name": "fan_in/hydroflow",
            "value": 185909925,
            "range": "± 277878",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/scheduled",
            "value": 181779076,
            "range": "± 2324279",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/teer",
            "value": 57969395,
            "range": "± 217161",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow",
            "value": 11908413,
            "range": "± 75621",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow_builder",
            "value": 44655375,
            "range": "± 93963",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow",
            "value": 48238780,
            "range": "± 332151",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow/compiled",
            "value": 14674819,
            "range": "± 10872",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow/scheduled",
            "value": 2554236,
            "range": "± 1270",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow",
            "value": 2562939,
            "range": "± 2224",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "mingwei.samuel@gmail.com",
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel"
          },
          "committer": {
            "email": "mingwei.samuel@gmail.com",
            "name": "Mingwei Samuel",
            "username": "MingweiSamuel"
          },
          "distinct": true,
          "id": "2e3eb331c796a9347b6b69f87c68f2a8e51c1306",
          "message": "Add [ci-bench] tag to trigger CI benchmark on push",
          "timestamp": "2021-11-02T10:49:23-07:00",
          "tree_id": "5fa89012d3b52c03c6e179d5776daf9e46bb5b9d",
          "url": "https://github.com/hydro-project/hydroflow/commit/2e3eb331c796a9347b6b69f87c68f2a8e51c1306"
        },
        "date": 1635875900887,
        "tool": "cargo",
        "benches": [
          {
            "name": "arithmetic/hydroflow/compiled",
            "value": 375637,
            "range": "± 2734",
            "unit": "ns/iter"
          },
          {
            "name": "fan_in/hydroflow",
            "value": 198990009,
            "range": "± 969183",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/scheduled",
            "value": 197408439,
            "range": "± 2149795",
            "unit": "ns/iter"
          },
          {
            "name": "fan_out/hydroflow/teer",
            "value": 58293374,
            "range": "± 637446",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow",
            "value": 8643096,
            "range": "± 15054",
            "unit": "ns/iter"
          },
          {
            "name": "fork_join/hydroflow_builder",
            "value": 45835993,
            "range": "± 159493",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow",
            "value": 48377676,
            "range": "± 1238118",
            "unit": "ns/iter"
          },
          {
            "name": "identity/hydroflow/compiled",
            "value": 14679331,
            "range": "± 13158",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow/scheduled",
            "value": 2602169,
            "range": "± 4748",
            "unit": "ns/iter"
          },
          {
            "name": "reachability/hydroflow",
            "value": 2634837,
            "range": "± 14829",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}