from pyspark.sql import SparkSession, DataFrame
from pyspark.sql.types import StructType, StructField, StringType, IntegerType, FloatType
from pyspark.sql.functions import col, avg
import pandas as pd

# Configure the SparkSession to set the spark.driver.bindAddress property
spark = SparkSession.builder \
    .appName("movies_etl") \
    .config("spark.driver.bindAddress", "localhost") \
    .getOrCreate()

# Read CSV File into DataFrame
movies_csv_file_path = "movies.csv"
movies_schema= StructType([
    StructField("id", IntegerType(), True),
    StructField("name", StringType(), True),
    StructField("description", StringType(), True),
    StructField("category", StringType(), True)
])
movies_df = spark.read.csv(movies_csv_file_path, schema=movies_schema, header=False)

user_csv_file_path = "users.csv"
users_schema= StructType([
    StructField("id", IntegerType(), True),
    StructField("movie_id", IntegerType(), True),
    StructField("rating", FloatType(), True)
])
users_df = spark.read.csv(user_csv_file_path, schema=users_schema, header=False)

# Print the schema of the DataFrame
# df.printSchema()
# Uncomment to show the DataFrame
# movies_df.show()
# users_df.show()

# Calculate average rating *per movie*, ensuring column name consistency
avg_rating = users_df.groupBy("movie_id").agg(avg("rating").alias("avg_rating"))

# Join DataFrames with correct column names
df = movies_df.join(avg_rating, movies_df["id"] == avg_rating["movie_id"], "inner").drop(avg_rating.movie_id)
df.show()

pd_df = df.toPandas()
pd_df.to_csv("avg_ratings.csv", index=False)
# Coalesce into a single partition to avoid multiple files
# df = df.coalesce(1)
# Save the DataFrame as a CSV file (Reduce the number of partitions to 1 before writing)
# df.write.csv("avg_ratings.csv", mode="overwrite", header=True)

