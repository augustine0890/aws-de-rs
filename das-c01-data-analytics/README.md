# AWS Certified Data Analytics - Specialty
**[AWS Certified Data Analytics](https://aws.amazon.com/certification/certified-data-analytics-specialty/)**

## Data Collection Systems

1. Data Collection Systems and Data Streams in AWS
- Data collection systems give you the capability to ingest and kind of data, structured, unstructured, or semi-structured.
- 
- Can ingest using the appropriate frequency based on your situation
  - Batch, Streaming, Transactional
- Transform and/or filter your data as you collect it
- **Fault Tolerance and Data Persistence**
  - Kinesis Producer Library (KPL) retries
  - KPL can send a group of multiple records in each request:
    - If a record fails, it's put back into the KPL buffer for a retry
    - One record's failure does not fail a whole set of records.
    - The KPL also has rate limiting: limits per-shard throughput sent from a single producer, can help prevent excessive retries.
  - Kinesis Data Streams replicates your data synchronously across three AZs in one region
    - Don't use Kinesis Data Streams for protracted data persistence
    - Your data is retained for 24 hours, which you can extend to 7 days
  - Kinesis Data Firehose streams your data directly to a data destination
    - Destinations: S3, Redshift, Elasticsearch, Splunk, and also Kinesis Data Analytics. 

2. Data Integration Services in AWS
- Analyze Frequency, Volume, Batch, Streaming and transactional data
- AWS Glue Service

3. Data Compression and Transformation in AWS
- Compare Data Collections Systems
- Implement Data Order, Format and Compression of Data
- Analyze Data Transform while Ingesting the data in AWS
