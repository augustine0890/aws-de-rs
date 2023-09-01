# What is data engineering
## Data workflow
- Data Collection & Storage --> Data Preparation --> Exploration & Visualization --> Experimentation & Prediction
- Some DE tasks:
    - Gathering music consumption data from desktop and mobile sources
    - Build a pipeline collecting album covers and storing them
    - Optimizing the customers databases for analysis
- Data Scientist tasks:
    - Mining data, statistical modeling.
    - Identify which customers are likely to end their subscriptions, so marketing can target them and encourage them to renew.
    - Run an analysis on whether users prefer having the search bar on the top left or the top right of the desktop app.
- Data engineer's responsibilities
    - Ingest (Gather) data from different sources
    - Streamline data acquisition
    - Optimize databases for analysis
    - Remove corrupted data
    - Develop, construct, test, and maintain scalable data architectures
- Big data
    - Have to think about how to deal with its size
    - So large traditional methods don't work anymore
- The five Vs: volumne, variety, volocity, veracity, value
- Data Engineering: ingest, process, store, pipelines, provide up-to-date (accurate, relevant data)
- Data pipelines ensure an efficient flow of the data
    - Automate: extracting, transforming, combining, validating, loading.
    - Reduce: human intervention, errors, time it takes data to flow
- Database: MySQL, PostgreSQL
    - Hold large amounts of data
    - Support application
    - Other databases are used for analyses
- Processing: Spark, Hive
    - Distributed over clusters of virtual machines.
    - Clean, aggregate, join data
- Scheduling: airflow
    - Make sure jobs run in a specific order and all dependencies are resolved correctly
- ETL: popular framework for designing data pipelines
    - Extract data
    - Transform extracted Data
    - Load transformed data to another database
- Data pipelines:
    - Move data from one system to another
    - May follow ETL
    - Data may not be transformed
    - Data may be directly loaded in applications

## Data Engineering Toolbox
### Database
- A usually large collection of data organized especially for rapid search and retrieval (holds, retrieve/search, and organizes data)
- SQL and NoSQL
- The star schema consists of one or more fact tables referencing any number of dimension tables
    - Facts: things that happened
    - Dimensions: information on the world
### Parallel computing
- Apache Hadoop (HDFS, Hive, MapReduce)
- Apache Spark: avoid disk writes
    - Resilient distributed datasets (RDD)
    - PySpark: python interface to Spark
### Workflow scheduling
- Spotify's Luigi
- Apache Airflow (DAGs)

## Extract - Transfrom - Load (ETL)
## Case Study

# Storing data
## Data structures
- Structured data
    - Easy to search and organize
    - Consistent model, rows and columns
    - Defined types
    - Can be grouped to form relations
- Semi-structured data
    - Relatively easy to search and organize
- Unstructured data
    - Does not follow a model, can't be contained in rows and columns
    - Difficult to search and organize
    - Usually text, sound, pictures or videos
    - Usually stored in data lakes, can appear in data warehouses or databases

## SQL databases
- Allows you to access many records at once, and group, filter or aggregate them
- Data engineers use SQL to create and maintain databases
## Data warehouse and data lakes
- Data lake:
    - Stores all the raw data (all data structures)
    - Can be petabytes
    - Cost-effective
    - Difficult to analyze
    - Requires an up-to-date data catalog
    - Big data, real-time analytics
- Data warehouse: type of database
    - Specific data for specific use (mainly structured data)
    - Optimized for data analysis
    - Ad-hoc, read-only queries
- Data catalog for data lakes
    - What is the source of this data?
    - Where is this data used?
    - Who is the owner of the data?
    - How often is the data updated?
    - Ensures reproducibility

# Moving and processing data
## Processing data
- Converting raw data into meaningful information
- Remove unwanted data
- Convert data from one type to another
- Organize data
- To fit into a schema/structure
- Increase productivity
- How data engineers process data:
    - Data manipulation, cleaning, and tidying tasks
    - Store data in a sanely structured database
    - Create views on top of the database tables
    - Optimizing the performance of the database

## Scheduling data
- Can apply to any task listed in data processing
- Holds each piece and organize how they work together
- Runs tasks in a specific order and resolvies all dependencies
- Batches
    - Group records at intervals
    - Cheap
- Streams
    - Send individual records right away

## Parallel computing
- Split tasks up into several smaller subtasks
- Distribute these subtasks over several computers
- Extra processing power, reduced memory footprint
- Disadvantages: moving data incurs a cost, communication time

## Cloud computing
- Database reliability: data replication
- Risk with sensitive data