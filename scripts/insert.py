import sqlite3
import os
import json
from tqdm import tqdm
import mmap
import grpc
import protos.surface_pb2_grpc as paper_pb2_rpc
import protos.surface_pb2 as paper_pb2


def connect_to_db(db_path):
    conn = sqlite3.connect(db_path)
    c = conn.cursor()
    return conn, c

def connect_to_paper_endpoint():
    channel = grpc.insecure_channel('localhost:8005')
    stub = paper_pb2_rpc.PaperSearchStub(channel)
    return channel, stub
    
def create_index(c):
    create_idx_cmd = ["CREATE INDEX IF NOT EXISTS idx_papers_corpusid ON papers(corpusid);", "CREATE INDEX IF NOT EXISTS idx_paper_authors_corpusid ON paper_authors(corpusid);", "CREATE INDEX IF NOT EXISTS idx_externalids_corpusid ON externalids(corpusid);", "CREATE INDEX IF NOT EXISTS idx_categories_corpusid ON categories(corpusid);"]
    for cmd in create_idx_cmd:
        print(f"executing {cmd}")
        c.execute(cmd)

def retrieve_n_papers(c, n):

    if isinstance(n, int):
        base_query = f"""
        SELECT * FROM papers
        ORDER BY corpusid
        LIMIT {n};
        """
    else:
        base_query = f"""
        SELECT * FROM papers
        ORDER BY corpusid
        BETWEEN {n[0]} AND {n[1]};
        """

    author_query = f"""
    SELECT pa.corpusid, a.authorid, a.name
    FROM paper_authors pa
    JOIN authors a ON pa.authorid = a.authorid
    WHERE pa.corpusid IN (SELECT corpusid FROM papers ORDER BY corpusid LIMIT {n});
    """

    externalids_query = f"""
    SELECT * FROM externalids
    WHERE corpusid IN (SELECT corpusid FROM papers ORDER BY corpusid LIMIT {n});
    """

    categories_query = f"""
    SELECT * FROM categories
    WHERE corpusid IN (SELECT corpusid FROM papers ORDER BY corpusid LIMIT {n});
    """

    c.execute(base_query)
    papers = c.fetchall()

    #print(papers)

    # c.execute(author_query)
    # authors = c.fetchall()

    # print(authors)

    c.execute(externalids_query)
    externalids = c.fetchall()

    #print(externalids)

    # c.execute(categories_query)
    # categories = c.fetchall()

    # print(categories)

    rets = []

    for paper, externalid in zip(papers, externalids):
        eids = {
            "mag" : externalid[4],
            "acl" : externalid[1],
            "doi" : externalid[6],
            "pmid" : externalid[7],
            "arxiv" : externalid[3],
        }
        _eids = paper_pb2.Externalids(**eids)
        openaccessinfo = {
            "externalids" : eids,
            "license" : None,
            "url" : paper[1],
            "status" : None,
        }
        _openaccessinfo = paper_pb2.OpenAccessInfo(**openaccessinfo)

        ret = {
            "corpusid" : str(paper[0]),
            "openaccessinfo" : openaccessinfo,
            "abstract" : paper[3],
            "title" : paper[2],
            "fulltext" : "",
            "updated" : paper[15],
        }
        _ret = paper_pb2.Paper(**ret)

        rets.append(_ret)

    return rets

def collect_intermediate_results(results):
    responses = []
    print("collecting intermediate results...")
    for result in tqdm(results):
        responses.append(result.result())
    return responses

def main():
    db_conn, db_cursor = connect_to_db('scripts/db/papers.db')
    channel, stub = connect_to_paper_endpoint()
    # print("creating index")
    # create_index(db_cursor)
    # print("index created")
    
    papers = retrieve_n_papers(db_cursor, 2000000)
    print(papers[1])
    print("inserting papers...")
    results = []
    for paper in tqdm(papers):
        results.append(stub.InsertPaper.future(paper_pb2.InsertPaperRequest(paper = paper)))
        if len(results) > 2048:
            responses = collect_intermediate_results(results)
            results = []

    print(responses)

if __name__ == "__main__":
    main()