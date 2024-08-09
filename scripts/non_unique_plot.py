import argparse
import asyncio
import json
import os
import matplotlib.pyplot as plt
import pandas as pd
import subprocess
import sys


# Get the directory of the script
SCRIPT_DIR = os.path.dirname(os.path.abspath(sys.argv[0]))
RUST_DIR = os.path.dirname(SCRIPT_DIR)
CARGO_TOML = os.path.join(RUST_DIR, 'Cargo.toml')

def cargo_build():
    result = subprocess.run(
        "cargo build --release".split(" "),
        cwd=RUST_DIR,
        text=True,
    )
    return result

def run_cargo_command(command: str):
    result = subprocess.run(
        ['./target/release/reperm_gen'] + command.split(" "),
        cwd=RUST_DIR,
        text=True,
    )
    return result

def remove_file(file_path: str):
    file_path = os.path.join(RUST_DIR, file_path)
    try:
        os.remove(file_path)
        print(f"{file_path} has been removed.")
    except FileNotFoundError:
        print(f"{file_path} does not exist.")
    except PermissionError:
        print(f"Permission denied to delete {file_path}.")
    except Exception as e:
        print(f"An error occurred: {e}")

def load_json(file_path: str):
    try:
        with open(file_path, 'r') as file:
            return json.load(file)
    except FileNotFoundError:
        print(f"{file_path} does not exist.")
    except json.JSONDecodeError:
        print(f"Error decoding JSON from {file_path}.")
    except Exception as e:
        print(f"An error occurred: {e}")

async def process(n: int):
    """n is the n in S_n"""
    rankings_arg = ','.join(str(i) for i in range(s,n + 1))
    cargo_cmd = f"find-chain -s {n} -l lru -c {rankings_arg} -o S_{n}.json"
    run_cargo_command(cargo_cmd)
    return n, f"S_{n}.json"



async def process_all(s, n):
    tasks = [process(i) for i in range(s, n + 1)]
    return await asyncio.gather(*tasks)


if __name__ == "__main__":
    print("Currently compiling cargo to build release!")
    cargo_build()

    parser = argparse.ArgumentParser(description="A simple command-line argument parser to plot the results of finding chains in symmetric locality.")
    parser.add_argument("-n", "--symmetric_n", help="Input S_n")
    
    args = parser.parse_args()
    n = int(args.symmetric_n)
    s = 3
    results = asyncio.run(process_all(s, n))
    tuples = []
    for n, res in results:
        data = load_json(res)
        chain_data = data["chain_data"]
        length = chain_data["length_chain"]
        non_unique_choices = chain_data["length_non_unique"]
        tuples.append((n, length, non_unique_choices))
        # cleanup
        remove_file(res)

    dtypes = {
        'n': 'int', 
        'chain_length': 'int', 
        'non_unique_choices': 'int'
    }
    frame = pd.DataFrame(tuples, columns=["n", "chain_length", "non_unique_choices"])
    csv_file_path = 'output.csv'
    frame.to_csv(csv_file_path, index=False)


    frame['ratio'] = frame['non_unique_choices'] / frame['chain_length']

    plt.figure(figsize=(10, 6))
    plt.plot(frame['n'], frame['ratio'], marker='o', linestyle='-', color='b', label='Non-Unique Choices / Chain Length')

    plt.title('Ratio of Non-Unique Choices to Chain Length', fontsize=16)
    plt.xlabel('n', fontsize=14)
    plt.ylabel('Non-Unique Choices / Chain Length', fontsize=14)
    plt.xticks(frame['n'])  # Set x-ticks to be the values of n
    plt.grid(True)
    plt.legend()
    plt.tight_layout()

    plt.show()

    plt.savefig(f'non_unique_choices_ratio_plot_S_{n}.png')