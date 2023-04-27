import json
import os
from multiprocessing import Pool

def process_file(file, batch_size=1024):
    with open(file, encoding="utf-8") as f:
        sample = []
        batch_count = 0

        output_file = f"{file}_parsed.json"
        with open(output_file, 'w') as out_f:
            out_f.write('[\n')

            for line in f:
                obj = json.loads(line)
                sample.append(obj)

                if len(sample) >= batch_size:
                    out_f.write(',\n'.join(json.dumps(item, indent=4) for item in sample))
                    if not line.isspace():
                        out_f.write(',\n')
                    sample = []
                    batch_count += 1

            if sample:
                out_f.write(',\n'.join(json.dumps(item, indent=4) for item in sample))
                out_f.write('\n]')

    return output_file

if __name__ == "__main__":
    dataset_base_path = '/Users/paul/dev/datasets/semantic_scholar'
    papers_path = os.path.join(dataset_base_path, 'abstracts')
    files = list(filter(lambda x: x != "donot" and x != ".DS_Store", os.listdir(papers_path)))
    files = [os.path.join(papers_path, file) for file in files]
    with Pool() as pool:
        output_files = pool.map(process_file, files)

    print(f"Output files: {output_files}")