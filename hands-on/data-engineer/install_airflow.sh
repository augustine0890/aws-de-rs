#!/bin/bash
export AIRFLOW_HOME="/Users/supertree/Documents/code/data-engineering-aws/hands-on/data-engineer/airflow"
echo "Airflow home is set to: $AIRFLOW_HOME"

echo "Note From Instructor: This script will install Airflow WITH the example dags. If you want Airflow WITHOUT the example dags please run this command after this script finishes, before starting airflow: sed -i -e '/load_examples =/ s/= .*/= False/' ${AIRFLOW_HOME}/airflow.cfg"

sleep 3

AIRFLOW_VERSION=2.8.2
PYTHON_VERSION="$(python --version | cut -d " " -f 2 | cut -d "." -f 1-2)"
echo "Installing Airflow Version $AIRFLOW_VERSION, with Python $PYTHON_VERSION"
CONSTRAINT_URL="https://raw.githubusercontent.com/apache/airflow/constraints-${AIRFLOW_VERSION}/constraints-${PYTHON_VERSION}.txt"
pip install "apache-airflow==${AIRFLOW_VERSION}" --constraint "${CONSTRAINT_URL}"

echo "Running DB Init"
# create the database schema
airflow db migrate

airflow users create \
        --username admin \
        --firstname Augustine \
        --lastname Nguyen \
        --role Admin \
        --email augustino0890@gmail.com

airflow webserver --port 8080
