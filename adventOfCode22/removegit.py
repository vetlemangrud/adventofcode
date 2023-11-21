import os
import shutil

for i in range(1, 23):
    try:
        shutil.rmtree(f"des{'0' if i < 10 else ''}{i}/.git")
        os.remove(f"des{'0' if i < 10 else ''}{i}/.gitignore")
    except:
        print(f"{i} fail")
