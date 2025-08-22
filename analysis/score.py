import json, sys
from pathlib import Path

def score_benchmark(file):
    data = [json.loads(line) for line in open(file) if line.strip()]
    output_file = Path(file).parent / f"{Path(file).stem}_scored{Path(file).suffix}"
    
    print(f"Scoring {len(data)} results. Press Ctrl+C to save and quit.")
    
    for i, item in enumerate(data):
        if 'with_tools_correct' in item and item['with_tools_correct'] is not None:
            continue
            
        print(f"\n{'='*50}")
        print(f"Question {i+1}/{len(data)}: {item['prompt']}")
        print(f"Expected: {item['expected_output']}")
        print(f"{'='*50}")
        
        print(f"\n[WITH TOOLS] ({item['with_tools_duration_ms']}ms, {item['with_tools_length']} chars)")
        print(item['with_tools'])
        while True:
            correct = input("\nCorrect? (y/n): ").lower().strip()
            if correct in ['y', 'n']: break
        item['with_tools_correct'] = correct == 'y'
        
        print(f"\n[WITHOUT TOOLS] ({item['without_tools_duration_ms']}ms, {item['without_tools_length']} chars)")
        print(item['without_tools'])
        while True:
            correct = input("\nCorrect? (y/n): ").lower().strip()
            if correct in ['y', 'n']: break
        item['without_tools_correct'] = correct == 'y'
        
        if (i + 1) % 5 == 0:
            with open(output_file, 'w') as f:
                for d in data: f.write(json.dumps(d) + '\n')
            print(f"\nSaved progress ({i+1}/{len(data)})")
    
    with open(output_file, 'w') as f:
        for d in data: f.write(json.dumps(d) + '\n')
    
    print(f"\nDone! Scored file: {output_file}")
    print(f"Now run: ./run.sh {output_file}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python score.py <benchmark.jsonl>")
        sys.exit(1)
    
    try:
        score_benchmark(sys.argv[1])
    except KeyboardInterrupt:
        print("\nSaved progress!")
        sys.exit(0)
