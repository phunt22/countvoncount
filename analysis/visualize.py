import json, sys, matplotlib.pyplot as plt, numpy as np
from pathlib import Path

def load_data(file):
    return [json.loads(line) for line in open(file) if line.strip()]

def has_correctness(data):
    return any('with_tools_correct' in item and item['with_tools_correct'] is not None for item in data)

def create_charts(data, output_dir):
    Path(output_dir).mkdir(exist_ok=True)
    
    with_times = [item['with_tools_duration_ms'] for item in data]
    without_times = [item['without_tools_duration_ms'] for item in data]
    with_lengths = [item['with_tools_length'] for item in data]
    without_lengths = [item['without_tools_length'] for item in data]
    
    colors = ['#3498db', '#e74c3c'] ## blue, orange
    
    plt.figure(figsize=(8, 5))
    plt.bar(['With Tools', 'Without Tools'], [np.mean(with_times), np.mean(without_times)], color=colors, alpha=0.8)
    plt.title('Average Response Time'), plt.ylabel('Time (ms)')
    plt.tight_layout(), plt.savefig(f'{output_dir}/time.png', dpi=300), plt.close()
    
    plt.figure(figsize=(8, 5))
    plt.bar(['With Tools', 'Without Tools'], [np.mean(with_lengths), np.mean(without_lengths)], color=colors, alpha=0.8)
    plt.title('Average Response Length'), plt.ylabel('Characters')
    plt.tight_layout(), plt.savefig(f'{output_dir}/length.png', dpi=300), plt.close()
    
    plt.figure(figsize=(12, 5))
    x = range(1, len(data) + 1)
    plt.plot(x, with_times, color=colors[0], label='With Tools', linewidth=2)
    plt.plot(x, without_times, color=colors[1], label='Without Tools', linewidth=2)
    plt.title('Time per Prompt'), plt.xlabel('Prompt'), plt.ylabel('Time (ms)'), plt.legend(), plt.grid(alpha=0.3)
    plt.tight_layout(), plt.savefig(f'{output_dir}/time_per_prompt.png', dpi=300), plt.close()
    
    plt.figure(figsize=(12, 5))
    plt.plot(x, with_lengths, color=colors[0], label='With Tools', linewidth=2)
    plt.plot(x, without_lengths, color=colors[1], label='Without Tools', linewidth=2)
    plt.title('Length per Prompt'), plt.xlabel('Prompt'), plt.ylabel('Characters'), plt.legend(), plt.grid(alpha=0.3)
    plt.tight_layout(), plt.savefig(f'{output_dir}/length_per_prompt.png', dpi=300), plt.close()
    
    if has_correctness(data):
        with_correct = sum(1 for item in data if item.get('with_tools_correct'))
        without_correct = sum(1 for item in data if item.get('without_tools_correct'))
        plt.figure(figsize=(8, 5))
        plt.bar(['With Tools', 'Without Tools'], [with_correct/len(data)*100, without_correct/len(data)*100], color=colors, alpha=0.8)
        plt.title('Accuracy'), plt.ylabel('Percentage'), plt.ylim(0, 100)
        plt.tight_layout(), plt.savefig(f'{output_dir}/accuracy.png', dpi=300), plt.close()
        
        groups = [
            [item['with_tools_length'] for item in data if item.get('with_tools_correct') == True],
            [item['with_tools_length'] for item in data if item.get('with_tools_correct') == False],
            [item['without_tools_length'] for item in data if item.get('without_tools_correct') == True],
            [item['without_tools_length'] for item in data if item.get('without_tools_correct') == False]
        ]
        avgs = [np.mean(g) if g else 0 for g in groups]
        plt.figure(figsize=(10, 5))
        plt.bar(['With\n(Correct)', 'With\n(Wrong)', 'Without\n(Correct)', 'Without\n(Wrong)'], 
                avgs, color=['#27ae60', '#3498db', '#f39c12', '#e74c3c'], alpha=0.8)
        plt.title('Length by Correctness'), plt.ylabel('Characters')
        plt.tight_layout(), plt.savefig(f'{output_dir}/length_by_correctness.png', dpi=300), plt.close()

def print_stats(data):
    with_times = [item['with_tools_duration_ms'] for item in data]
    without_times = [item['without_tools_duration_ms'] for item in data]
    with_lengths = [item['with_tools_length'] for item in data]
    without_lengths = [item['without_tools_length'] for item in data]
    
    print(f"Analyzed {len(data)} test cases")
    print(f"Time:   {np.mean(with_times):.0f}ms vs {np.mean(without_times):.0f}ms")
    print(f"Length: {np.mean(with_lengths):.0f} vs {np.mean(without_lengths):.0f} chars")
    
    time_diff = (np.mean(without_times) - np.mean(with_times))/np.mean(without_times)*100
    length_diff = (np.mean(without_lengths) - np.mean(with_lengths))/np.mean(without_lengths)*100
    print(f"Tools: {time_diff:+.0f}% time, {length_diff:+.0f}% length")
    
    if has_correctness(data):
        with_correct = sum(1 for item in data if item.get('with_tools_correct'))
        without_correct = sum(1 for item in data if item.get('without_tools_correct'))
        print(f"Accuracy: {with_correct/len(data)*100:.1f}% vs {without_correct/len(data)*100:.1f}%")
        
        with_correct_items = [item for item in data if item.get('with_tools_correct') == True]
        with_wrong_items = [item for item in data if item.get('with_tools_correct') == False]
        without_correct_items = [item for item in data if item.get('without_tools_correct') == True]
        without_wrong_items = [item for item in data if item.get('without_tools_correct') == False]
        
        if with_correct_items and with_wrong_items:
            print(f"With Tools - Correct: {np.mean([item['with_tools_length'] for item in with_correct_items]):.0f} chars, Wrong: {np.mean([item['with_tools_length'] for item in with_wrong_items]):.0f} chars")
        if without_correct_items and without_wrong_items:
            print(f"Without Tools - Correct: {np.mean([item['without_tools_length'] for item in without_correct_items]):.0f} chars, Wrong: {np.mean([item['without_tools_length'] for item in without_wrong_items]):.0f} chars")

def main():
    if len(sys.argv) < 2:
        print("Usage: python visualize.py <jsonl_file> [output_dir]")
        sys.exit(1)
    
    data = load_data(sys.argv[1])
    output_dir = sys.argv[2] if len(sys.argv) > 2 else 'output'
    
    create_charts(data, output_dir)
    print_stats(data)
    
    if not has_correctness(data):
        scored_file = Path(sys.argv[1]).parent / f"{Path(sys.argv[1]).stem}_with_correctness{Path(sys.argv[1]).suffix}"
        with open(scored_file, 'w') as f:
            for item in data:
                item['with_tools_correct'] = None
                item['without_tools_correct'] = None
                f.write(json.dumps(item) + '\n')
    
    print(f"Charts: {output_dir}/")

if __name__ == "__main__":
    main()
