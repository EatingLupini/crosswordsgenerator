# Script for downloading words and definitions

import requests
from bs4 import BeautifulSoup
import json
import re

def add_words(result, soup):
    tbody = soup.find_all('tbody')[0]
    trs = tbody.find_all('tr')
    for i in range(len(trs) - 1):
        tds = trs[i].find_all('td')

        key = re.sub(r'[^A-Za-z]', '', tds[0].string)
        values = list()

        raw_values = tds[1].find('pre').string.split('\r\n')
        for raw_value in raw_values:
            value = raw_value  # re.sub(r'[^A-Za-z]', '', raw_value)
            if value != '':
                values.append(value)
            
        result[key] = values


if __name__ == "__main__":
    result = dict()
    url = "https://www.crossword.one/pag_from_word_to_definition.asp"

    s = requests.Session()
    r = s.get(url)
    if r.status_code == 200:
        soup = BeautifulSoup(r.content, 'html.parser')
        add_words(result, soup)
    else:
        print("Requests error")
        exit(-1)
    
    counter = 1
    print(f"Downloaded pages: {counter}")

    done = False
    while not done:
        payload = dict(fpdbr_0_PagingMove='  >   ', Parola='')
        r = s.post(url, data=payload)
        soup = BeautifulSoup(r.content, 'html.parser')
        add_words(result, soup)

        input = soup.find_all('input', {"value": "  >   "})
        if len(input) <= 0:
            done = True
        
        counter += 1
        print(f"Downloaded pages: {counter}")

    print("\nWriting words on file...")
    with open("data/words.txt", "w") as f:
        f.write(json.dumps(result, indent=4))
    print("DONE!")
