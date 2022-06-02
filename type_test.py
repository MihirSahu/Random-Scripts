#!/usr/bin/python3

import requests

word_site = "https://www.mit.edu/~ecprice/wordlist.10000"

response = requests.get(word_site)
words = response.content.splitlines()

for idx, word in enumerate(words):
    words[idx] = word.decode('utf-8')


