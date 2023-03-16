import polars as pl
import matplotlib.pyplot as plt

df = pl.read_csv("wiener.txt", has_header=False, new_columns=["values"])
# change dtype from str to float
df = df.with_columns(pl.col("values").cast(pl.Float64))

plt.plot(df["values"])
plt.show()

# print(df)
