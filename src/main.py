import os
import re
import requests
from joblib import Memory

from pprint import pprint
from pandas import DataFrame

memory = Memory("./joblib_memory_cache")

@memory.cache
def get_data_from_g_sheet(sheet_id, range, api_key) -> DataFrame:
    request_url = f"https://sheets.googleapis.com/v4/spreadsheets/{sheet_id}/values/{range}?key={api_key}"
    g_sheet_response = requests.get(request_url)
    return g_sheet_response

def card_sheet_parser(g_sheet):
    g_sheet_rows = g_sheet["values"]
    not_reps_pattern = r"\d{1,}x\d{1,}"
    stack = {}
    for row in g_sheet_rows[2:]:
        if "week" in row[0].lower():
            continue
        elif len(re.findall(not_reps_pattern, row[0], re.I)) == 0:
            workout_name = row[0]
            stack[workout_name] = []
        else:
            stack[workout_name].append(row)

    pprint(stack)
    

def main():
    print("Hello world")
    sheet_id = os.environ["WORKOUT_ENGINE_SHEET_ID"]
    cell_range = "Cards!A:H";
    api_key = os.environ["GOOGLE_SHEETS_API_KEY"]

    g_data = get_data_from_g_sheet(sheet_id, cell_range, api_key).json()
    pprint(g_data)
    card_sheet_parser(g_data)

    # try:
    #     iterator = iter(the_element)
    # except TypeError:
    #     # not iterable
    # else:
    # iterable

# for obj in iterator:
#     pass

if __name__ == "__main__":
    exit(main())