import wget
import os
import json
from concurrent.futures import ThreadPoolExecutor

def download(urls, path):
    with ThreadPoolExecutor() as executor:
        for url in urls:
            executor.submit(wget.download, url, path)

def abstracts():
    abstracts_file = {}
    with open('abstracts.json') as file:
        abstracts_file = json.load(file)
    abstracts_urls = abstracts_file['files']
    outpath = '/Users/paul/dev/datasets/semantic_scholar/abstracts'
    download(abstracts_urls, outpath)

def papers():
    papers_file = {}
    with open('papers.json') as file:
        papers_file = json.load(file)
    papers_urls = papers_file['files']
    outpath = '/Users/paul/dev/datasets/semantic_scholar/papers'
    download(papers_urls, outpath)

def main():
    papers()
    abstracts()


if __name__ == '__main__':
    main()