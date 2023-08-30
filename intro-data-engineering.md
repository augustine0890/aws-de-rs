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
## Extract - Transfrom - Load (ETL)
## Case Study

# Storing data
# Moving and processing data
