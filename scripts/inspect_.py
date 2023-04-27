import os
import json

def abstracts():
    dataset_base_path = '/Users/paul/dev/datasets/semantic_scholar'
    abstracts_path = os.path.join(dataset_base_path, 'abstracts')
    files = os.listdir(abstracts_path)
    print(files)
    with open(os.path.join(abstracts_path, files[1])) as file:
        sample = []
        for line in file:
            obj = json.loads(line)
            sample.append(obj)

        json.dump(sample, open('abstracts_sample.json', 'w'), indent=4)

def papers():
    dataset_base_path = '/Users/paul/dev/datasets/semantic_scholar'
    papers_path = os.path.join(dataset_base_path, 'papers')
    files = list(filter(lambda x: x != "donot" and x != ".DS_Store", os.listdir(papers_path)))

    with open(os.path.join(papers_path, files[0]), encoding="utf-8") as file:
        sample = []
        for line in file:
            obj = json.loads(line)
            sample.append(obj)
            break

        json.dump(sample, open('paper_sample.json', 'w'), indent=4)

def main():
    #abstracts()
    papers()

if __name__ == '__main__':
    main()
    