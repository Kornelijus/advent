# Downloads current day's input data from with personal session cookie

import os
import requests
import argparse
import time

parser = argparse.ArgumentParser()

# import dotenv if available
try:
    from dotenv import load_dotenv
    load_dotenv()
except ImportError:
    # warn that dotenv is not available, using regular environment variables
    print("WARNING: dotenv not available, using regular environment variables")

# env ADVENT_COOKIE or raise error if not set
cookie = os.environ['ADVENT_COOKIE'] if 'ADVENT_COOKIE' in os.environ else parser.error('ADVENT_COOKIE not set')

# argparse --year or -y or first argument or default current year
parser.add_argument('--year', type=int, default=time.localtime().tm_year)

# argparse --day or -d or default current day of month
parser.add_argument('--day', type=int, default=time.localtime().tm_mday)

args = parser.parse_args()

year = args.year
day = args.day

# request get text file with session cookie
r = requests.get(f"https://adventofcode.com/{year}/day/{day}/input", cookies={"session": cookie})

# and save to {day}.txt
with open(f"{day}.txt", "w") as f:
    f.write(r.text)