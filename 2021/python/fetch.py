# Downloads current day's input data from with personal session cookie

import os
import requests
import argparse
import time

# load env from .env file if available
try:
    from dotenv import load_dotenv
    load_dotenv()
except ImportError:
    print("WARNING: dotenv not available, using regular environment variables")

parser = argparse.ArgumentParser()
cookie = os.environ['ADVENT_COOKIE'] if 'ADVENT_COOKIE' in os.environ else parser.error('ADVENT_COOKIE not set')
parser.add_argument('--year', type=int, default=time.localtime().tm_year)
parser.add_argument('--day', type=int, default=time.localtime().tm_mday)
args = parser.parse_args()

# request get text file with session cookie
r = requests.get(f"https://adventofcode.com/{args.year}/day/{args.day}/input", cookies={"session": cookie})

# and save to {day}.txt
with open(f"{args.day}.txt", "w") as f:
    f.write(r.text)