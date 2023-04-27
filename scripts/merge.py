import sqlite3
import os
import json
from tqdm import tqdm
import mmap

def column_exists(conn, table_name, column_name):
    cursor = conn.cursor()
    cursor.execute(f"PRAGMA table_info({table_name});")
    columns = cursor.fetchall()
    return any(column[1] == column_name for column in columns)

def get_num_lines(file_path):
    fp = open(file_path, "r+")
    buf = mmap.mmap(fp.fileno(), 0)
    lines = 0
    while buf.readline():
        lines += 1
    return lines

def create_tables(c):
    create_paper_table = """
    CREATE TABLE IF NOT EXISTS papers (
    corpusid INTEGER PRIMARY KEY,
    url TEXT,
    title TEXT,
    venue TEXT,
    publicationvenueid INTEGER,
    year INTEGER,
    referencecount INTEGER,
    citationcount INTEGER,
    influentialcitationcount INTEGER,
    isopenaccess BOOLEAN,
    s2fieldsofstudy TEXT,
    publicationtypes TEXT,
    publicationdate TEXT,
    journal TEXT,
    updated TEXT
    );
    """
    create_authors_table = """
    CREATE TABLE IF NOT EXISTS authors (
        authorid INTEGER PRIMARY KEY,
        name TEXT
    );
    """

    create_paper_author_table = """
    CREATE TABLE IF NOT EXISTS paper_authors (
        corpusid INTEGER,
        authorid INTEGER,
        FOREIGN KEY (corpusid) REFERENCES papers (corpusid),
        FOREIGN KEY (authorid) REFERENCES authors (authorid)
    );
    """

    create_externalids_table = """
    CREATE TABLE IF NOT EXISTS externalids (
        corpusid INTEGER PRIMARY KEY,
        ACL TEXT,
        DBLP TEXT,
        ArXiv TEXT,
        MAG TEXT,
        PubMed TEXT,
        DOI TEXT,
        PubMedCentral TEXT,
        FOREIGN KEY (corpusid) REFERENCES papers (corpusid)
    );
    """
    create_categories_table = """
    CREATE TABLE IF NOT EXISTS categories (
        corpusid INTEGER,
        category TEXT,
        source TEXT,
        FOREIGN KEY (corpusid) REFERENCES papers (corpusid)
    ); 
    """

    for sql in [create_paper_table, create_authors_table, create_paper_author_table, create_externalids_table, create_categories_table]:
        c.execute(sql)

def flatten(l):
    return [item for sublist in l for item in sublist]

def insert_batched(c, batch):
    papers_table_command = """
                INSERT OR IGNORE INTO papers (corpusid, url, title, venue, publicationvenueid, year, referencecount, citationcount, influentialcitationcount, isopenaccess, publicationdate, updated)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                """
    authors_table_command = """
                INSERT OR IGNORE INTO authors (authorid, name) VALUES (?, ?)
                """
    externalids_table_command = """
                INSERT OR IGNORE INTO externalids (corpusid, ACL, DBLP, ArXiv, MAG, PubMed, DOI, PubMedCentral)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                """
    paper_authors_table_command = """
                INSERT OR IGNORE INTO paper_authors (corpusid, authorid) VALUES (?, ?)
                """
    categories_table_command = """
                INSERT OR IGNORE INTO categories (corpusid, category, source) VALUES (?, ?, ?)
                """
        #c.execute("INSERT OR IGNORE INTO authors (authorid, name) VALUES (?, ?)", (author['authorId'], author['name']))
    def to_base_tuple(data):
        return (
            data['corpusid'],
            data['url'],
            data['title'],
            data['venue'],
            data['publicationvenueid'],
            data['year'],
            data['referencecount'],
            data['citationcount'],
            data['influentialcitationcount'],
            data['isopenaccess'],
            data['publicationdate'],
            data['updated']
            )
    def to_authors_tuple_list(data):
        return [(data['corpusid'], author['authorId'], author['name']) for author in (data['authors'] if data['authors'] else [])]

    def to_externalids_tuple(data):
        return (
            data['corpusid'],
            data['externalids']['ACL'],
            data['externalids']['DBLP'],
            data['externalids']['ArXiv'],
            data['externalids']['MAG'],
            data['externalids']['PubMed'],
            data['externalids']['DOI'],
            data['externalids']['PubMedCentral']
            )
    
    def to_categories_tuple_list(data):
        return [(data['corpusid'], category['category'], category['source']) for category in (data['s2fieldsofstudy'] if data['s2fieldsofstudy'] else [])]

    papers_table_batch = [to_base_tuple(data) for data in batch]
    authors_table_batch = list(map(lambda x: (x[1], x[2]), flatten([to_authors_tuple_list(data) for data in batch])))
    externalids_table_batch = [to_externalids_tuple(data) for data in batch]
    paper_authors_table_batch = list(map(lambda x: (x[0], x[1]), flatten([to_authors_tuple_list(data) for data in batch])))
    categories_table_batch = flatten([to_categories_tuple_list(data) for data in batch])

    c.executemany(papers_table_command, papers_table_batch)
    c.executemany(authors_table_command, authors_table_batch)
    c.executemany(externalids_table_command, externalids_table_batch)
    c.executemany(paper_authors_table_command, paper_authors_table_batch)
    c.executemany(categories_table_command, categories_table_batch)


def inesert_abstracts_batched(c, batch):
    command = """UPDATE papers SET abstract = ? WHERE corpusid = ?;"""
    abstracts_table_batch = [(data['abstract'], data['corpusid']) for data in batch]
    c.executemany(command, abstracts_table_batch)


def main():
    # Connect to the SQLite database file (or create it if it doesn't exist)
    conn = sqlite3.connect('scripts/db/papers.db')

    # Create a cursor object to execute SQL commands
    c = conn.cursor()

    create_tables(c)
    if not column_exists(conn, "papers", "abstract"):
            # If it doesn't exist, add it
            c.execute("ALTER TABLE papers ADD COLUMN abstract TEXT;")
            conn.commit()

    # Read the JSON file
    paper_files = os.listdir('/Users/paul/dev/datasets/semantic_scholar/papers/json')
    file_paths = ['/Users/paul/dev/datasets/semantic_scholar/papers/json/' + paperfile for paperfile in paper_files]
    for file_path in file_paths:
        print('Processing file: {}'.format(file_path))
        with open(file_path) as f:
            batch = []
            for line in tqdm(f, total=get_num_lines(file_path)):
                data = json.loads(line)
                batch.append(data)
                if len(batch) >= 1000:
                    insert_batched(c, batch)
                    conn.commit()
                    batch = []

    conn.commit()
    abstract_files = os.listdir('/Users/paul/dev/datasets/semantic_scholar/abstracts/json')
    file_paths = ['/Users/paul/dev/datasets/semantic_scholar/abstracts/json/' + abstractfile for abstractfile in abstract_files]
    for file_path in file_paths:
        print('Processing file: {}'.format(file_path))
        with open(file_path) as f:
            batch = []
            for line in tqdm(f, total=get_num_lines(file_path)):
                data = json.loads(line)
                batch.append(data)
                if len(batch) >= 1000:
                    inesert_abstracts_batched(c, batch)
                    conn.commit()
                    batch = []
                
    # Commit the changes and close the connection
    conn.commit()
    conn.close()

if __name__ == '__main__':
    #increase_open_file_limit()
    main()