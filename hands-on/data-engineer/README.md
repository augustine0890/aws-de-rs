# Data Engineering Foundations
- The data engineer moves data from transactional databases to analytical database to make life easier for the data scientist.
- Modern Data Pipelines:
  - Ingesting data from a source system
  - Transforming and cleaning data
  - Storing data in a persistent medium
  - Goal: ensure data is in a format that can be analyzed
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
- Automation: Scheduling (Orchestration)
  - Directed Acyclic Graph (DAG):
    - A collection of all the tasks that need to be run, organized in a way that reflects their relationship and dependencies.
  - Set up and manage workflows
  - Plan jobs with specific intervals
  - Resolve dependency requirements of jobs
  - Apache Airflow, Oozie, Luigi: describing, executing, and monitoring workflows or pipelines.
  - Proprietary: AWS Step Functions, Google Cloud Composer, AWS MWAA

## ETL (Extract-Transform-Load) Pipelines
- The ETL process involves extracting data from a source system, transforming it into a format that is suitable for analysis, and loading it into a target system, such as a data warehouse or a database.
- Tasks: represents a discrete unit of work that needs to be performed, such as reading data from a file, applying a transformation, or writing data to a database.
- 
- **Online Transaction Processing (OLTP)**
  - Application db, many transactions, row oriented, stored per record.
- **Online Analytical Processing (OLAP)**
  - Analytical database, aggregate queries, column oriented, parallelization.
- How to install [PySpark](https://sparkbyexamples.com/pyspark/how-to-install-pyspark-on-mac/) on Mac

## Apache Airflow
- Airflow pipelines are configuration as code (Python), allowing for dynamic pipeline generation.
- Install `apache-airflow`: `pip install apache-airflow`
- Initialize the database: `airflow db migrate`
- The `airflow standalone` command initialize the database, creates a user, and starts all components:
  - `airflow standalone`
  - Run Airflow manually:
    ```
      airflow db migrate
      
      airflow users create \
        --username admin \
        --firstname Augustine \
        --lastname Nguyen \
        --role Admin \
        --email augustino0890@gmail.com
  
      airflow webserver --port 8080
  
      airflow scheduler
      ```
- Airflow webserver is a web-based UI that allows to interact with you Airflow environment:
  - Responds to HTTP requests and is configured with viewer and editor modes
- Airflow's scheduler manages the execution of DAGs and tasks
  - Schedules and triggers DAG runs based on the dependencies and parameters specified in the DAG definition
  - Manages the state and status of each task
  - Distributed task execution across the workers


Airflow [Quick Start](https://airflow.apache.org/docs/apache-airflow/stable/start.html)

## Run Llama 2 Locally
- Download the Large Language Model:
  - [Hugging Face](https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGML/tree/main) Host (All models)
  - Model descriptions: [README](https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGML#provided-files)
- Install the `llama-cpp-python` package: `pip install llama-cpp-python`