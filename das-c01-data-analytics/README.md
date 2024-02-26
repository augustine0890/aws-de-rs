# AWS Certified Data Analytics - Specialty
**[AWS Certified Data Analytics](https://aws.amazon.com/certification/certified-data-analytics-specialty/)**

## Data Collection Systems

1. Data Collection Systems and Data Streams in AWS
- Data collection systems give you the capability to ingest and kind of data, structured, unstructured, or semi-structured.
- The characteristics of your data streaming workload guide you in the selection of your streaming components.
- Can ingest using the appropriate frequency based on your situation
  - Batch, Streaming, Transactional
- **Database Migration Services (DMS)**: migrating data from various sources, including relational databases, to AWS S3 or Redshift
  - It supports both one-time migrations and continuous data replication (Change Data Capture)
- Transform and/or filter your data as you collect it
- **Fault Tolerance and Data Persistence**
  - Kinesis Producer Library (KPL) retries
  - KPL can send a group of multiple records in each request:
    - If a record fails, it's put back into the KPL buffer for a retry
    - One record's failure does not fail a whole set of records.
    - The KPL also has rate limiting: limits per-shard throughput sent from a single producer, can help prevent excessive retries.
  - Kinesis Data Streams replicates your data synchronously across three AZs in one region
    - Real-time data streaming and processing. It's not the best fit for batch-oriented data migration from RDS to a data lake.
    - Don't use Kinesis Data Streams for protracted data persistence
    - Your data is retained for 24 hours, which you can extend to 7 days
  - Kinesis Data Firehose streams your data directly to a data destination
    - Optimized for loading data into data lakes and data warehouses. 
    - Destinations: S3, Redshift, Elasticsearch, Splunk, and also Kinesis Data Analytics. 
    - Can transform your data (using Lambda function), prior to delivering the data
  - The Kinesis Consumer Library (KCL) processes your data from your Kinesis Data Stream:
    - Uses checkpointing using DynamoDB to track which records have been read from a shard
      - If a KCL read fails, the KCL uses the checkpoint cursor to resume at the failed record
    - Use unique names for your applications in the KCL, since DynamoDB tables use name
    - Watch out for provisioning throughput exceptions in DynamoDB: many shards or frequent checkpointing.
  - Alternatives to the KPL:
    - Use the Kinesis API instead of KPL when you need the fastest processing time
      - KPL uses RecordMaxBufferedTime to delay processing to accommodate aggregation
    - Kinesis Agent:
      - Kinesis Agent installs on your EC2 instance
      - Monitors files, such as log files, and streams new data to your Kinesis steam
      - Emits CloudWatch metrics to help with monitoring and error handling


2. Data Integration Services in AWS
- Analyze Frequency, Volume, Batch, Streaming and transactional data
- AWS Glue Service

3. Data Compression and Transformation in AWS
- Compare Data Collections Systems
- Implement Data Order, Format and Compression of Data
- Analyze Data Transform while Ingesting the data in AWS
