from sys import version_info
python_version = f'{version_info.major}.{version_info.minor}.{version_info.micro}'
import platform
os = f'{platform.system()}'
from datetime import datetime
time_start = datetime.now()
for i in range(1, 1000000001):
    if i % 10000000 == 0:
        print(i)
time_end = datetime.now()
time_run = time_end - time_start
print(f'Run time for Python version {str(python_version)} on {str(os)} is: {str(time_run)}')