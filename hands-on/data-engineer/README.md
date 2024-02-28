# Data Engineering Foundations
- The data engineer moves data from transactional databases to analytical database to make life easier for the data scientist.
## DE Tools
- Storage: Databases
  - A large collection of data organized in efficient structures and formats to support rapid search and retrieval 
  - Used to hold large amount of data
  - Support for applications and analyses
  - SQL, NoSQL: MySQL, PostgreSQL, MongoDB, Redis
  - DB schema: describes the structure and relations of a databases
- Processing: Frameworks
  - Split task into subtasks and distribute subtasks on several computers
    - Overhead due to communication between nodes
    - Task needs to be large
    - Need several processing units
  - Data cleaning, data aggregation, data clustering
  - Batch and stream processing
  - Apache Spark, Hive, Kafka, Dask
  - Hive is built from the need to use structured queries for parallel processing.
- Automation: Scheduling
  - Directed Acyclic Graph (DAG):
    - A collection of all the tasks that need to be run, organized in a way that reflects their relationship and dependencies.
  - Set up and manage workflows
  - Plan jobs with specific intervals
  - Resolve dependency requirements of jobs
  - Apache Airflow, Oozie, Luigi: describing, executing, and monitoring workflows or pipelines.

## ETL Pipelines
- **Online Transaction Processing (OLTP)**
  - Application db, many transactions, row oriented, stored per record.
- **Online Analytical Processing (OLAP)**
  - Analytical database, aggregate queries, column oriented, parallelization.
-  How to install [PySpark](https://sparkbyexamples.com/pyspark/how-to-install-pyspark-on-mac/) on Mac