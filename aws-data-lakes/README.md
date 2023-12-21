# Designing Data Lakes on AWS

## Data Lakes
- A centralized repository to securely store structured and unstructured data, with no scaling restrictions and run different types of analytics.
- The ingested data can be later processed and visualized for specific needs.
- Reasons for a data lake:
    - Inrease operational efficiency by providing different ways to collect, process, and store data
    - Make data available
    - Lower transaction costs --> store data without needing to think about its structure.
    - Databases usually have their storage and processing mechanisms tied together, which makes them less flexible to scale --> offload databases and data warehouses to improve performance and save costs
- Data lakes should be data-agnostic
- Data lake components: ingest and store, catalog and search, process and serve, protect and secure.
- Organizations are collecting and analyzing increasing amounts of data making it difficult for traditional on-premises solutions for data storage, data management, and analytics.
- Amazon S3 and S3 Glacier provide an ideal storage solution for data lakes.
- Data warehouses: schema-on-write architecture
    - Optimized to analyze relation data
    - Advance to optimize for fast SQL queries
    - Data is cleaned, enriched, and transformed --> single source of truth
- Data lakes: schema-on-read architecture
    - Can store non-relational data from mobile apps, IoT devices, and social media.
    - The structure of the data or schema is not defined when data is captured
    - Different types of analytics on your data like SQL queries, big data analytics, full text search, real-time analytics, and machine learning can be used to uncover insights.
- Data lakes are often viewed as complementary to data warehouses. It allows businesses to store and access large volumnes of raw, unprocessed data in a data lake, while also processing and transforming that data into a structured format for analysis in a data warehouses.
- More information on Data Lake - [Snowflake](https://snowflake.hub.hushly.com/data-lake-stream/cloud-data-lakes-for-dummies])

### Example architecture
- Some common architectures use data lakes to ingest, store, and clean data. Then, the solutions architect can move that data into a data warehouse for visualization.
- S3: 
    - Object storage (logs, images, videos, and archives) unstructured and semi-structured data
    - Not directly designed for complex data analysis
- Kinesis firehose: ETL service from AWS. It takes real-time, streaming data and seamlessly delivers it to various destinations likes data lakes, data stores, and analytics services.
```
Web servers --(send web logs)--> Kinesis firehose -(store)-> S3 <--(query SQL)-- Athena (insights: http errors or unique visitors)
``````