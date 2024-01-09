# Backfilling Tool
This tool will help us with backfilling provided we give it a proper csv to push data from.

## Features
* login to env (prod, preprod etc.)
* dynamic URI endpoint change (maybe with the help of cli args)
* Supports multipe data files csv, json etc. (initially just a CSV)
* adjust the number of calls to the WCU RCU count of a dynamo DB. 


## User story
* Client install the tool, do the login operation with maybe?
* Customer then provide (tool --env prod/preprod --uri [post-url] --data-csv [path-to-csv-file])
