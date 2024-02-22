"""
Daily Passengers
Calculate how many passengers rode each day.
Use as little memory as you can.
"""
# Make sure install: pip install pyarrow
import pandas as pd

filename = 'data/yellow_tripdata_2021-02.parquet'
columns = ['tpep_pickup_datetime', 'passenger_count']
df = pd.read_parquet(filename, columns=columns)
# print(df.columns)
print(df.head())
print(df.groupby(df['tpep_pickup_datetime'].dt.round('D'))['passenger_count'].sum())
