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
  - KPL can send a group of multiple records in each request (aggregated the records):
    - If a record fails, it's put back into the KPL buffer for a retry
    - One record's failure does not fail a whole set of records.
    - The KPL also has rate limiting: limits per-shard throughput sent from a single producer, can help prevent excessive retries.
    - Automatically process retries for your data records. Spamming due to excessive retries is prevented by the rate-limiting feature.
  - Kinesis Data Streams replicates your data `synchronously` across three AZs in one region
    - Real-time data streaming and processing. It's not the best fit for batch-oriented data migration from RDS to a data lake.
    - Don't use Kinesis Data Streams for protracted data persistence
    - Your data is retained for 24 hours, which you can extend to 7 days
  - Kinesis Data Firehose streams your data directly to a data destination
    - Optimized for loading data into data lakes and data warehouses. 
    - Destinations: S3, Redshift, Elasticsearch, Splunk, and also Kinesis Data Analytics. 
    - Can transform your data (using Lambda function), prior to delivering the data
  - The Kinesis Consumer Library (KCL) processes your data from your Kinesis Data Stream:
    - Uses checkpointing using DynamoDB to track which records have been read from a shard
      - If a KCL read fails, the KCL uses the `checkpoint cursor` to resume at the failed record
    - Use unique names for your applications in the KCL, since DynamoDB tables use name
    - Watch out for provisioning throughput exceptions in DynamoDB: many shards or frequent checkpointing.
  - Alternatives to the KPL:
    - Use the Kinesis API instead of KPL when you need the fastest processing time
      - KPL uses RecordMaxBufferedTime to delay processing to accommodate aggregation
    - Kinesis Agent:
      - Kinesis Agent installs on your EC2 instance
      - Monitors files, such as log files, and streams new data to your Kinesis steam
      - Emits CloudWatch metrics to help with monitoring and error handling

**Kinesis Data Streams Lab - KPL and KCL**
- Kinesis Producer Library can send a group of multiple records in each request:
  - AggregationEnable = `"true"`
  - RecordMaxBufferedTime = `2000 milliseconds`
- If a record fails, it's put back into the KPL buffer for a retry
- The KPL also has rate limiting:
  - Limits per-shard throughput sent from a single producer, can help prevent excessive retries, 50% higher than shard limit is the default
- The initial number of shards for your shard estimation calculation should be the `Max of Inbound write capacity and Outgoing read capacity`.
  - Inbound write capacity: `Inbound write bandwidth in KB / 1000`
    - `Inbound write bandwidth in KB = (Average record size in KB * Number of records per second)`
  - Outgoing read capacity: `(Inbound write bandwidth in KB * Number of consumbers) / 2000`
  - Initial number of shards: `Max of {Inbound write capacity, Outgoing read capacity}`
- Kinesis Consumer Library process the data from Kinesis Data Stream:
  - Uses shard iterator with shard id to retrieve records
  - If a KCL read fails, the KCL can use the checkpoint cursor to resume at the failed record:
    - Use `IRecordProcessorCheckpointer` in Java and `amazon_kclpy.kcl.Checkpointer` in Python.

**Kinesis Data Firehose Lab**
- Fully managed that delivers directly to S3, Redshift, Elasticsearch, and Splunk.
- Can also feed Kinesis Data Analytics
- Can optionally transform your data, using Lambda, before delivering it to its destination
- Firehose `Producers` send records to Firehose:
  - Web server logs data
  - Kinesis Data Stream
  - Kinesis Agent
  - Kinesis Firehouse API using the AWS SDK
  - CloudWatch logs and/or events
  - AWS IoT
  - Firehose buffers incoming streaming data for a set buffer size (MBs) and a buffer interval (seconds)

2. Data Integration Services in AWS
- Analyze Frequency, Volume, Batch, Streaming and transactional data
- AWS Glue Service

3. Data Compression and Transformation in AWS
- Compare Data Collections Systems
- Implement Data Order, Format and Compression of Data
- Analyze Data Transform while Ingesting the data in AWS
