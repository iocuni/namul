with open("src.c", "rb") as f:
    data = f.read()
data = data.decode("utf-16-le")
with open("src.c", "w", encoding="utf-8", newline="") as f:
    f.write(data)